###############################################################################
#
#  Welcome to Baml! To use this generated code, please run the following:
#
#  $ bundle add baml sorbet-runtime
#
###############################################################################

# This file was generated by BAML: please do not edit it. Instead, edit the
# BAML files and re-generate this code.
#
# frozen_string_literal: true
# rubocop: disable
# formatter:off
require 'set'

module Baml
    class TypeBuilder
        def initialize
            @registry = Baml::Ffi::TypeBuilder.new
            @classes = Set[ "BigNumbers",  "BinaryNode",  "Blah",  "BlockConstraint",  "BlockConstraintForParam",  "BookOrder",  "ClassForNullLiteral",  "ClassOptionalOutput",  "ClassOptionalOutput2",  "ClassToRecAlias",  "ClassWithBlockDone",  "ClassWithImage",  "ClassWithoutDone",  "ComplexMemoryObject",  "CompoundBigNumbers",  "ContactInfo",  "CustomTaskResult",  "DummyOutput",  "DynInputOutput",  "DynamicClassOne",  "DynamicClassTwo",  "DynamicOutput",  "Earthling",  "Education",  "Email",  "EmailAddress",  "Event",  "FakeImage",  "FlightConfirmation",  "FooAny",  "Forest",  "FormatterTest0",  "FormatterTest1",  "FormatterTest2",  "FormatterTest3",  "GroceryReceipt",  "InnerClass",  "InnerClass2",  "InputClass",  "InputClassNested",  "LinkedList",  "LinkedListAliasNode",  "LiteralClassHello",  "LiteralClassOne",  "LiteralClassTwo",  "MalformedConstraints",  "MalformedConstraints2",  "Martian",  "MemoryObject",  "MergeAttrs",  "NamedArgsSingleClass",  "Nested",  "Nested2",  "NestedBlockConstraint",  "NestedBlockConstraintForParam",  "Node",  "NodeWithAliasIndirection",  "OptionalListAndMap",  "OptionalTest_Prop1",  "OptionalTest_ReturnType",  "OrderInfo",  "OriginalA",  "OriginalB",  "Person",  "PhoneNumber",  "Quantity",  "RaysData",  "ReceiptInfo",  "ReceiptItem",  "Recipe",  "Resume",  "Schema",  "SearchParams",  "SemanticContainer",  "SmallThing",  "SomeClassNestedDynamic",  "StringToClassEntry",  "TestClassAlias",  "TestClassNested",  "TestClassWithEnum",  "TestMemoryOutput",  "TestOutputClass",  "Tree",  "TwoStoriesOneTitle",  "UnionTest_ReturnType",  "WithReasoning", ]
            @enums = Set[ "AliasedEnum",  "Category",  "Category2",  "Category3",  "Color",  "DataType",  "DynEnumOne",  "DynEnumTwo",  "EnumInClass",  "EnumOutput",  "Hobby",  "MapKey",  "NamedArgsSingleEnum",  "NamedArgsSingleEnumList",  "OptionalTest_CategoryType",  "OrderStatus",  "Tag",  "TestEnum", ]
        end

        def string
            @registry.string
        end
    
        def int
            @registry.int
        end

        def float
            @registry.float
        end

        def bool
            @registry.bool
        end

        def list(inner_type)
            @registry.list(inner_type)
        end
        
        def null
            @registry.null
        end

        def map(key_type, value_type)
            @registry.map(key_type, value_type)
        end

        def union(*types)
            @registry.union(*types)
        end

        def add_class(name)
            if @classes.include?(name)
                raise "Class with name #{name} already exists."
            end
            if @enums.include?(name)
                raise "Enum with name #{name} already exists."
            end
            @classes.add(name)
            ClassBuilder.new(@registry, name)
        end

        def add_enum(name)
            if @classes.include?(name)
                raise "Class with name #{name} already exists."
            end
            if @enums.include?(name)
                raise "Enum with name #{name} already exists."
            end
            @enums.add(name)
            EnumBuilder.new(@registry, name)
        end

        class ClassBuilder
            def initialize(registry, name, properties = nil)
                @builder = registry.class_(name)
                @properties = properties == nil ? Set.new : properties
            end

            def type
                @builder.field
            end

            def add_property(name, type)
                if @properties.include?(name)
                    raise "Property #{name} already exists."
                end
                @properties.add(name)
                @builder.property(name).type(type)
            end
        end

        class EnumBuilder
            def initialize(registry, name, values = nil)
                @builder = registry.enum(name)
                @values = values == nil ? Set.new : values
            end

            def type
                @builder.field
            end

            def add_value(name)
                if @values.include?(name)
                    raise "Value #{name} already exists."
                end
                @values.add(name)
                @builder.value(name)
            end
        end

        
        def DummyOutput
            ClassBuilder.new(@registry, "DummyOutput", Set[ "nonce",  "nonce2", ])
        end
        
        def DynInputOutput
            ClassBuilder.new(@registry, "DynInputOutput", Set[ "testKey", ])
        end
        
        def DynamicClassOne
            ClassBuilder.new(@registry, "DynamicClassOne", Set[])
        end
        
        def DynamicClassTwo
            ClassBuilder.new(@registry, "DynamicClassTwo", Set[ "hi",  "some_class",  "status", ])
        end
        
        def DynamicOutput
            ClassBuilder.new(@registry, "DynamicOutput", Set[])
        end
        
        def OriginalB
            ClassBuilder.new(@registry, "OriginalB", Set[ "value", ])
        end
        
        def Person
            ClassBuilder.new(@registry, "Person", Set[ "name",  "hair_color", ])
        end
        
        def SomeClassNestedDynamic
            ClassBuilder.new(@registry, "SomeClassNestedDynamic", Set[ "hi", ])
        end
        

        
        def Color
            EnumBuilder.new(@registry, "Color", Set[ "RED",  "BLUE",  "GREEN",  "YELLOW",  "BLACK",  "WHITE", ])
        end
        
        def DynEnumOne
            EnumBuilder.new(@registry, "DynEnumOne", Set[])
        end
        
        def DynEnumTwo
            EnumBuilder.new(@registry, "DynEnumTwo", Set[])
        end
        
        def Hobby
            EnumBuilder.new(@registry, "Hobby", Set[ "SPORTS",  "MUSIC",  "READING", ])
        end
        
    end
end