# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.
#
# BAML version: 0.0.1
# Generated Date: __DATE__
# Generated by: vbv

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long

from baml_core._impl.provider import llm_provider_factory
from os import environ


AZURE_DEFAULT = llm_provider_factory(
    provider="openai-chat",
    options=dict(
        api_key=environ['OPENAI_API_KEY'],
        max_tokens=400,
        request_timeout=45,
        model="gpt-3.5-turbo",
    ),
)
