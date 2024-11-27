use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Index,
};

use internal_baml_diagnostics::DatamodelError;
use internal_baml_parser_database::{Tarjan, TypeWalker};
use internal_baml_schema_ast::ast::{FieldType, SchemaAst, TypeExpId, WithName, WithSpan};

use crate::validate::validation_pipeline::context::Context;

/// Validates if the dependency graph contains one or more infinite cycles.
pub(super) fn validate(ctx: &mut Context<'_>) {
    // First, build a graph of all the "required" dependencies represented as an
    // adjacency list. We're only going to consider type dependencies that can
    // actually cause infinite recursion. Unions and optionals can stop the
    // recursion at any point, so they don't have to be part of the "dependency"
    // graph because technically an optional field doesn't "depend" on anything,
    // it can just be null.
    let dependency_graph = HashMap::from_iter(ctx.db.walk_classes().map(|class| {
        let expr_block = &ctx.db.ast()[class.id];

        // TODO: There's already a hash set that returns "dependencies" in
        // the DB, it shoudn't be necessary to traverse all the fields here
        // again and build yet another graph, we need to refactor
        // .dependencies() or add a new method that returns not only the
        // dependency name but also field arity. The arity could be computed at
        // the same time as the dependencies hash set. Code is here:
        //
        // baml-lib/parser-database/src/types/mod.rs
        // fn visit_class()
        let mut dependencies = HashSet::new();

        for field in &expr_block.fields {
            if let Some(field_type) = &field.expr {
                insert_required_deps(class.id, field_type, ctx, &mut dependencies);
            }
        }

        (class.id, dependencies)
    }));

    report_infinite_cycles(
        &dependency_graph,
        ctx,
        "These classes form a dependency cycle",
    );

    // The graph for aliases is already built when visiting each alias. Maybe
    // we can use the same logic for the required dependencies graph above.
    report_infinite_cycles(
        &ctx.db.type_alias_dependencies(),
        ctx,
        "These aliases form a dependency cycle",
    );
}

/// Finds and reports all the infinite cycles in the given graph.
///
/// It prints errors like this:
///
/// "Error validating: These classes form a dependency cycle: A -> B -> C"
fn report_infinite_cycles<V: Ord + Eq + Hash + Copy>(
    graph: &HashMap<V, HashSet<V>>,
    ctx: &mut Context<'_>,
    message: &str,
) where
    SchemaAst: Index<V>,
    <SchemaAst as Index<V>>::Output: WithName,
    <SchemaAst as Index<V>>::Output: WithSpan,
{
    for component in Tarjan::components(graph) {
        let cycle = component
            .iter()
            .map(|id| ctx.db.ast()[*id].name().to_string())
            .collect::<Vec<_>>()
            .join(" -> ");

        // TODO: We can push an error for every sinlge class here (that's what
        // Rust does), for now it's an error for every cycle found.
        ctx.push_error(DatamodelError::new_validation_error(
            &format!("{message}: {cycle}"),
            ctx.db.ast()[component[0]].span().clone(),
        ));
    }
}

/// Inserts all the required dependencies of a field into the given set.
///
/// Recursively deals with unions of unions. Can be implemented iteratively with
/// a while loop and a stack/queue if this ends up being slow / inefficient or
/// it reaches stack overflows with large inputs.
fn insert_required_deps(
    id: TypeExpId,
    field: &FieldType,
    ctx: &Context<'_>,
    deps: &mut HashSet<TypeExpId>,
) {
    match field {
        FieldType::Symbol(arity, ident, _) if arity.is_required() => {
            match ctx.db.find_type_by_str(ident.name()) {
                Some(TypeWalker::Class(class)) => {
                    deps.insert(class.id);
                }
                Some(TypeWalker::TypeAlias(alias)) => {
                    // TODO: By the time this code runs we would ideally want
                    // type aliases to be resolved but we can't do that because
                    // type alias cycles are not validated yet, we have to
                    // do that in this file. Take a look at the `validate`
                    // function at `baml-lib/baml-core/src/lib.rs`.
                    //
                    // First we run the `ParserDatabase::validate` function
                    // which creates the alias graph by visiting all aliases.
                    // Then we run the `validate::validate` which ends up
                    // running this code here. Finally we run the
                    // `ParserDatabase::finalize` which is the place where we
                    // can resolve type aliases since we've already validated
                    // that there are no cycles so we won't run into infinite
                    // recursion. Ideally we want this:
                    //
                    // insert_required_deps(id, alias.resolved(), ctx, deps);

                    // But we'll run this instead which will follow all the
                    // alias pointers again until it finds the resolved type.
                    insert_required_deps(id, alias.target(), ctx, deps);
                }
                _ => {}
            }
        }

        FieldType::Union(arity, field_types, _, _) if arity.is_required() => {
            // All the dependencies of the union.
            let mut union_deps = HashSet::new();

            // All the dependencies of a single field in the union. This is
            // reused on every iteration of the loop below to avoid allocating
            // a new hash set every time.
            let mut nested_deps = HashSet::new();

            for f in field_types {
                insert_required_deps(id, f, ctx, &mut nested_deps);

                // No nested deps found on this component, this makes the
                // union finite, so no need to go deeper.
                if nested_deps.is_empty() {
                    return;
                }

                // Add the nested deps to the overall union deps and clear the
                // iteration hash set.
                union_deps.extend(nested_deps.drain());
            }

            // A union does not depend on itself if the field can take other
            // values. However, if it only depends on itself, it means we have
            // something like this:
            //
            // class Example {
            //    field: Example | Example | Example
            // }
            if union_deps.len() > 1 {
                union_deps.remove(&id);
            }

            deps.extend(union_deps);
        }

        _ => {}
    }
}
