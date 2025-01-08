import pytest
from assertpy import assert_that
from baml_py import errors
from .test_setup import b
from ..baml_client.types import (
    BlockConstraintForParam,
    NestedBlockConstraintForParam,
    MalformedConstraints2,
)

def all_succeeded(checks):
    return all(check.status == "succeeded" for check in checks.values())

@pytest.mark.asyncio
async def test_constraints():
    res = await b.PredictAge("Greg")
    assert res.certainty.checks["unreasonably_certain"].status == "failed"
    assert not (all_succeeded(res.certainty.checks))


@pytest.mark.asyncio
async def test_constraint_union_variant_checking():
    res = await b.ExtractContactInfo(
        "Reach me at help@boundaryml.com, or 111-222-3333 if needed."
    )
    assert res.primary.value is not None
    assert res.primary.value == "help@boundaryml.com"
    assert res.secondary.value is not None
    assert res.secondary.value == "111-222-3333"


@pytest.mark.asyncio
async def test_return_malformed_constraint():
    with pytest.raises(errors.BamlError) as e:
        res = await b.ReturnMalformedConstraints(1)
        assert res.foo.value == 2
        assert res.foo.checks["foo_check"].status == "failed"
    assert "Failed to coerce value" in str(e)


@pytest.mark.asyncio
async def test_use_malformed_constraint():
    with pytest.raises(errors.BamlError) as e:
        res = await b.UseMalformedConstraints(MalformedConstraints2(foo=2))
        assert res == 3
    assert "object has no method named length" in str(e)


@pytest.mark.asyncio
async def test_block_constraints():
    ret = await b.MakeBlockConstraint()
    assert ret.checks["cross_field"].status == "failed"


@pytest.mark.asyncio
async def test_nested_block_constraints():
    ret = await b.MakeNestedBlockConstraint()
    print(ret)
    assert ret.nbc.checks["cross_field"].status == "succeeded"


@pytest.mark.asyncio
async def test_block_constraint_arguments():
    with pytest.raises(errors.BamlInvalidArgumentError) as e:
        block_constraint = BlockConstraintForParam(bcfp=1, bcfp2="too long!")
        await b.UseBlockConstraint(block_constraint)
    assert "Failed assert: hi" in str(e)

    with pytest.raises(errors.BamlInvalidArgumentError) as e:
        nested_block_constraint = NestedBlockConstraintForParam(nbcfp=block_constraint)
        await b.UseNestedBlockConstraint(nested_block_constraint)
    assert "Failed assert: hi" in str(e)