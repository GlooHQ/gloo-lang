# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from .clients.client_anthropic import Anthropic
from .clients.client_azure_default import AZURE_DEFAULT
from .clients.client_azure_gpt4 import AZURE_GPT4
from .clients.client_azure_yes_no import AZURE_YES_NO
from .clients.client_large_response import LARGE_RESPONSE
from .clients.client_resilientgpt4 import ResilientGPT4
from .functions.fx_booleanfunc import BAMLBooleanFunc
from .functions.fx_classfunc import BAMLClassFunc
from .functions.fx_classifytool import BAMLClassifyTool
from .functions.fx_enumfunc import BAMLEnumFunc
from .functions.fx_intfunc import BAMLIntFunc
from .functions.fx_maybepolishtext import BAMLMaybePolishText
from .functions.fx_messagesimplifier import BAMLMessageSimplifier
from .functions.fx_namedfunc import BAMLNamedfunc
from .functions.fx_optionalfunc import BAMLOptionalFunc
from .functions.fx_optionalnamedfunc import BAMLOptionalNamedFunc
from .functions.fx_stringfunc import BAMLStringFunc
from baml_core.otel import add_message_transformer_hook, flush_trace_logs
from baml_core.provider_manager import LLMManager
from baml_core.services import LogSchema
from baml_lib import DeserializerException, baml_init
from typing import Callable, List, Optional


class BAMLClient:
    BooleanFunc = BAMLBooleanFunc
    ClassFunc = BAMLClassFunc
    ClassifyTool = BAMLClassifyTool
    EnumFunc = BAMLEnumFunc
    IntFunc = BAMLIntFunc
    MaybePolishText = BAMLMaybePolishText
    MessageSimplifier = BAMLMessageSimplifier
    Namedfunc = BAMLNamedfunc
    OptionalFunc = BAMLOptionalFunc
    OptionalNamedFunc = BAMLOptionalNamedFunc
    StringFunc = BAMLStringFunc
    AZURE_DEFAULT = AZURE_DEFAULT
    AZURE_GPT4 = AZURE_GPT4
    AZURE_YES_NO = AZURE_YES_NO
    Anthropic = Anthropic
    LARGE_RESPONSE = LARGE_RESPONSE
    ResilientGPT4 = ResilientGPT4

    def __init__(self):
        LLMManager.validate()
        baml_init()

    def configure(
        self,
        project_id: Optional[str] = None,
        secret_key: Optional[str] = None,
        base_url: Optional[str] = None,
        enable_cache: Optional[bool] = None,
        stage: Optional[str] = None,
    ):
        return baml_init(
            project_id=project_id,
            secret_key=secret_key,
            base_url=base_url,
            enable_cache=enable_cache,
            stage=stage,
        )

    def add_before_send_message_hook(self, hook: Callable[[LogSchema], None]):
        add_message_transformer_hook(hook)

    def flush(self):
        flush_trace_logs()


baml = BAMLClient()
