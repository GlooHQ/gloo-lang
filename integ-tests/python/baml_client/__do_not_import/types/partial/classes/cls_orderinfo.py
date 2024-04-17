# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from ...enums.enm_orderstatus import OrderStatus
from baml_lib._impl.deserializer import register_deserializer
from pydantic import BaseModel
from typing import Optional


@register_deserializer({  })
class PartialOrderInfo(BaseModel):
    order_status: Optional[OrderStatus] = None
    tracking_number: Optional[str] = None
    estimated_arrival_date: Optional[str] = None
