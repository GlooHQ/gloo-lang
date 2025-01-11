import pytest
import asyncio
import concurrent.futures
import random
import time
from assertpy import assert_that
from baml_py import errors
from baml_py.baml_py import BamlLogEvent
from ..baml_client.tracing import trace, set_tags, on_log_event
from .test_setup import b, sync_b, DO_NOT_USE_DIRECTLY_UNLESS_YOU_KNOW_WHAT_YOURE_DOING_RUNTIME

@trace
async def parent_async(myStr: str):
    set_tags(myKey="myVal")
    await async_dummy_func(myStr)
    await b.FnOutputClass(myStr)
    sync_dummy_func(myStr)
    return "hello world parentasync"


@trace
async def parent_async2(myStr: str):
    return "hello world parentasync2"


@trace
def parent_sync(myStr: str):
    import time
    import random
    time.sleep(0.5 + random.random())
    sync_dummy_func(myStr)
    return "hello world parentsync"


@trace
async def async_dummy_func(myArgggg: str):
    await asyncio.sleep(0.5 + random.random())
    return "asyncDummyFuncOutput"


@trace
def sync_dummy_func(dummyFuncArg: str):
    return "pythonDummyFuncOutput"


@trace
def trace_thread_pool():
    with concurrent.futures.ThreadPoolExecutor() as executor:
        futures = [executor.submit(parent_sync, "second-dummycall-arg") for _ in range(10)]
        for future in concurrent.futures.as_completed(futures):
            future.result()


@trace
async def trace_thread_pool_async():
    with concurrent.futures.ThreadPoolExecutor() as executor:
        futures = [executor.submit(trace_async_gather) for _ in range(10)]
        for future in concurrent.futures.as_completed(futures):
            _ = await future.result()


@trace
async def trace_async_gather():
    await asyncio.gather(*[async_dummy_func("handcrafted-artisan-arg") for _ in range(10)])


@pytest.mark.asyncio
async def test_tracing_async_only():
    @trace
    async def top_level_async_tracing():
        @trace
        async def nested_dummy_fn(_foo: str):
            time.sleep(0.5 + random.random())
            return "nested dummy fn"

        @trace
        async def dummy_fn(foo: str):
            await asyncio.gather(
                b.FnOutputClass(foo),
                nested_dummy_fn(foo),
            )
            return "dummy fn"

        await asyncio.gather(
            dummy_fn("dummy arg 1"),
            dummy_fn("dummy arg 2"),
            dummy_fn("dummy arg 3"),
        )
        await asyncio.gather(parent_async("first-arg-value"), parent_async2("second-arg-value"))
        return 1

    # Clear any existing traces
    DO_NOT_USE_DIRECTLY_UNLESS_YOU_KNOW_WHAT_YOURE_DOING_RUNTIME.flush()
    _ = DO_NOT_USE_DIRECTLY_UNLESS_YOU_KNOW_WHAT_YOURE_DOING_RUNTIME.drain_stats()

    res = await top_level_async_tracing()
    assert_that(res).is_equal_to(1)

    DO_NOT_USE_DIRECTLY_UNLESS_YOU_KNOW_WHAT_YOURE_DOING_RUNTIME.flush()
    stats = DO_NOT_USE_DIRECTLY_UNLESS_YOU_KNOW_WHAT_YOURE_DOING_RUNTIME.drain_stats()
    print("STATS", stats)
    assert_that(stats.started).is_equal_to(15)
    assert_that(stats.finalized).is_equal_to(stats.started)
    assert_that(stats.submitted).is_equal_to(stats.started)
    assert_that(stats.sent).is_equal_to(stats.started)
    assert_that(stats.done).is_equal_to(stats.started)
    assert_that(stats.failed).is_equal_to(0)


def test_tracing_sync():
    _ = sync_dummy_func("second-dummycall-arg")


def test_tracing_thread_pool():
    trace_thread_pool()


@pytest.mark.asyncio
async def test_tracing_thread_pool_async():
    await trace_thread_pool_async()


@pytest.mark.asyncio
async def test_tracing_async_gather():
    await trace_async_gather()


@pytest.mark.asyncio
async def test_tracing_async_gather_top_level():
    await asyncio.gather(*[async_dummy_func("second-dummycall-arg") for _ in range(10)])


@pytest.mark.asyncio
async def test_event_log_hook():
    def event_log_hook(event: BamlLogEvent):
        print("Event log hook1: ")
        print("Event log event ", event)

    DO_NOT_USE_DIRECTLY_UNLESS_YOU_KNOW_WHAT_YOURE_DOING_RUNTIME.flush()  # clear any existing hooks
    on_log_event(event_log_hook)
    res = await b.TestFnNamedArgsSingleStringList(["a", "b", "c"])
    assert res
    DO_NOT_USE_DIRECTLY_UNLESS_YOU_KNOW_WHAT_YOURE_DOING_RUNTIME.flush()  # clear the hook
    on_log_event(None)