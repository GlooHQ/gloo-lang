from typing import Any, Callable, Dict, Optional, Tuple

class FunctionResult:
    """The result of a BAML function call.

    Represents any of:

        - a successful LLM call, with a successful type parse
        - a successful LLM call, with a failed type parse
        - a failed LLM call, due to a provider outage or other network error
        - a failed LLM call, due to an inability to build the request
        - or any other outcome, really

    We only expose the parsed result to Python right now.
    """

    def __str__(self) -> str: ...
    def parsed(self) -> Any: ...
    # Returns True if the function call was successful, False otherwise
    def is_ok(self) -> bool: ...
    def internals(self) -> str: ...

class FunctionResultStream:
    """The result of a BAML function stream.

    Provides a callback interface to receive events from a BAML result stream.

    Use `on_event` to set the callback, and `done` to drive the stream to completion.
    """

    def __str__(self) -> str: ...
    def on_event(
        self, on_event: Callable[[FunctionResult], None]
    ) -> FunctionResultStream: ...
    async def done(self, ctx: RuntimeContextManager) -> FunctionResult: ...

class SyncFunctionResultStream:
    """The result of a BAML function stream.

    Provides a callback interface to receive events from a BAML result stream.

    Use `on_event` to set the callback, and `done` to drive the stream to completion.
    """

    def __str__(self) -> str: ...
    def on_event(
        self, on_event: Callable[[FunctionResult], None]
    ) -> SyncFunctionResultStream: ...
    def done(self, ctx: RuntimeContextManager) -> FunctionResult: ...

class BamlImagePy:
    @staticmethod
    def from_url(url: str) -> BamlImagePy: ...
    @staticmethod
    def from_base64(media_type: str, base64: str) -> BamlImagePy: ...
    def is_url(self) -> bool: ...
    def is_base64(self) -> bool: ...
    def as_url(self) -> str: ...
    def as_base64(self) -> Tuple[str, str]: ...

class BamlAudioPy:
    @staticmethod
    def from_url(url: str) -> BamlAudioPy: ...
    @staticmethod
    def from_base64(media_type: str, base64: str) -> BamlAudioPy: ...
    def is_url(self) -> bool: ...
    def is_base64(self) -> bool: ...
    def as_url(self) -> str: ...
    def as_base64(self) -> Tuple[str, str]: ...

class RuntimeContextManager:
    def upsert_tags(self, tags: Dict[str, Any]) -> None: ...
    def deep_clone(self) -> RuntimeContextManager: ...

class BamlRuntime:
    @staticmethod
    def from_directory(directory: str, env_vars: Dict[str, str]) -> BamlRuntime: ...
    async def call_function(
        self,
        function_name: str,
        args: Dict[str, Any],
        ctx: RuntimeContextManager,
        tb: Optional[TypeBuilder],
        cr: Optional[ClientRegistry],
    ) -> FunctionResult: ...
    @staticmethod
    def from_files(
        root_path: str, files: Dict[str, str], env_vars: Dict[str, str]
    ) -> BamlRuntime: ...
    def stream_function(
        self,
        function_name: str,
        args: Dict[str, Any],
        on_event: Optional[Callable[[FunctionResult], None]],
        ctx: RuntimeContextManager,
        tb: Optional[TypeBuilder],
        cr: Optional[ClientRegistry],
    ) -> FunctionResultStream: ...
    def stream_function_sync(
        self,
        function_name: str,
        args: Dict[str, Any],
        on_event: Optional[Callable[[FunctionResult], None]],
        ctx: RuntimeContextManager,
        tb: Optional[TypeBuilder],
        cr: Optional[ClientRegistry],
    ) -> SyncFunctionResultStream: ...
    def create_context_manager(self) -> RuntimeContextManager: ...
    def flush(self) -> None: ...
    def drain_stats(self) -> TraceStats: ...
    def set_log_event_callback(
        self, handler: Optional[Callable[[BamlLogEvent], None]]
    ) -> None: ...

class LogEventMetadata:
    event_id: str
    parent_id: Optional[str]
    root_event_id: str

    def __init__(
        self, event_id: str, parent_id: Optional[str], root_event_id: str
    ) -> None: ...

class BamlLogEvent:
    metadata: LogEventMetadata
    prompt: Optional[str]
    raw_output: Optional[str]
    parsed_output: Optional[str]
    start_time: str

    def __init__(
        self,
        metadata: LogEventMetadata,
        prompt: Optional[str],
        raw_output: Optional[str],
        parsed_output: Optional[str],
        start_time: str,
    ) -> None: ...

class TraceStats:
    @property
    def failed(self) -> int: ...
    @property
    def started(self) -> int: ...
    @property
    def finalized(self) -> int: ...
    @property
    def submitted(self) -> int: ...
    @property
    def sent(self) -> int: ...
    @property
    def done(self) -> int: ...

class BamlSpan:
    @staticmethod
    def new(
        runtime: BamlRuntime,
        function_name: str,
        args: Dict[str, Any],
        ctx: RuntimeContextManager,
    ) -> BamlSpan: ...
    def finish(self, result: Any, ctx: RuntimeContextManager) -> str | None: ...

class TypeBuilder:
    def __init__(self) -> None: ...
    def enum(self, name: str) -> EnumBuilder: ...
    def class_(self, name: str) -> ClassBuilder: ...
    def string(self) -> FieldType: ...
    def int(self) -> FieldType: ...
    def float(self) -> FieldType: ...
    def bool(self) -> FieldType: ...
    def list(self, element_type: FieldType) -> FieldType: ...
    def null(self) -> FieldType: ...
    def optional(self, inner_type: FieldType) -> FieldType: ...
    def map(self, key_type: FieldType, value_type: FieldType) -> FieldType: ...
    def union(self, *types: FieldType) -> FieldType: ...

class ClientRegistry:
    def __init__(self) -> None: ...
    def add_llm_client(
        self,
        name: str,
        provider: str,
        options: Dict[str, Any],
        retry_policy: Optional[str] = None,
    ) -> None: ...
    def set_primary(self, name: str) -> None: ...

class FieldType:
    def list(self) -> FieldType: ...
    def optional(self) -> FieldType: ...

class EnumBuilder:
    def value(self, name: str) -> EnumValueBuilder: ...
    def alias(self, alias: Optional[str]) -> EnumBuilder: ...
    def field(self) -> FieldType: ...

class EnumValueBuilder:
    def alias(self, alias: Optional[str]) -> EnumValueBuilder: ...
    def skip(self, skip: Optional[bool] = True) -> EnumValueBuilder: ...
    def description(self, description: Optional[str]) -> EnumValueBuilder: ...

class ClassBuilder:
    def field(self) -> FieldType: ...
    def property(self, name: str) -> ClassPropertyBuilder: ...

class ClassPropertyBuilder:
    def type(self, field_type: FieldType) -> ClassPropertyBuilder: ...
    def alias(self, alias: Optional[str]) -> ClassPropertyBuilder: ...
    def description(self, description: Optional[str]) -> ClassPropertyBuilder: ...

def invoke_runtime_cli() -> None: ...
