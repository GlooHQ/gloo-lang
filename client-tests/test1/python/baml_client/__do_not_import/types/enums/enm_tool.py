# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from baml_lib._impl.deserializer import register_deserializer
from enum import Enum


@register_deserializer({
  "k1": "CodeInterpreter",
  "k1: Use this tool if the user is asking to compute something": "CodeInterpreter",
  "k2": "DrawImage",
  "k2: Use this tool if the user is asking to draw something": "DrawImage",
  "k3": "GenerateText",
  "k3: Use this tool if the user is asking to generate text": "GenerateText"
})
class Tool(str, Enum):
    CodeInterpreter = "CodeInterpreter"
    DrawImage = "DrawImage"
    GenerateText = "GenerateText"
