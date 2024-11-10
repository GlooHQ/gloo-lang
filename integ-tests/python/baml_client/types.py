###############################################################################
#
#  Welcome to Baml! To use this generated code, please run the following:
#
#  $ pip install baml
#
###############################################################################

# This file was generated by BAML: please do not edit it. Instead, edit the
# BAML files and re-generate this code.
#
# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off
import baml_py
from enum import Enum
from pydantic import BaseModel, ConfigDict
from typing import Dict, Generic, List, Literal, Optional, TypeVar, Union


T = TypeVar('T')
CheckName = TypeVar('CheckName', bound=str)

class Check(BaseModel):
    name: str
    expression: str
    status: str

class Checked(BaseModel, Generic[T,CheckName]):
    value: T
    checks: Dict[CheckName, Check]

def get_checks(checks: Dict[CheckName, Check]) -> List[Check]:
    return list(checks.values())

def all_succeeded(checks: Dict[CheckName, Check]) -> bool:
    return all(check.status == "succeeded" for check in get_checks(checks))



class AliasedEnum(str, Enum):
    
    KEY_ONE = "KEY_ONE"
    KEY_TWO = "KEY_TWO"

class Category(str, Enum):
    
    Refund = "Refund"
    CancelOrder = "CancelOrder"
    TechnicalSupport = "TechnicalSupport"
    AccountIssue = "AccountIssue"
    Question = "Question"

class Category2(str, Enum):
    
    Refund = "Refund"
    CancelOrder = "CancelOrder"
    TechnicalSupport = "TechnicalSupport"
    AccountIssue = "AccountIssue"
    Question = "Question"

class Category3(str, Enum):
    
    Refund = "Refund"
    CancelOrder = "CancelOrder"
    TechnicalSupport = "TechnicalSupport"
    AccountIssue = "AccountIssue"
    Question = "Question"

class Color(str, Enum):
    
    RED = "RED"
    BLUE = "BLUE"
    GREEN = "GREEN"
    YELLOW = "YELLOW"
    BLACK = "BLACK"
    WHITE = "WHITE"

class DataType(str, Enum):
    
    Resume = "Resume"
    Event = "Event"

class DynEnumOne(str, Enum):
    pass

class DynEnumTwo(str, Enum):
    pass

class EnumInClass(str, Enum):
    
    ONE = "ONE"
    TWO = "TWO"

class EnumOutput(str, Enum):
    
    ONE = "ONE"
    TWO = "TWO"
    THREE = "THREE"

class Hobby(str, Enum):
    
    SPORTS = "SPORTS"
    MUSIC = "MUSIC"
    READING = "READING"

class NamedArgsSingleEnum(str, Enum):
    
    ONE = "ONE"
    TWO = "TWO"

class NamedArgsSingleEnumList(str, Enum):
    
    ONE = "ONE"
    TWO = "TWO"

class OptionalTest_CategoryType(str, Enum):
    
    Aleph = "Aleph"
    Beta = "Beta"
    Gamma = "Gamma"

class OrderStatus(str, Enum):
    
    ORDERED = "ORDERED"
    SHIPPED = "SHIPPED"
    DELIVERED = "DELIVERED"
    CANCELLED = "CANCELLED"

class Tag(str, Enum):
    
    Security = "Security"
    AI = "AI"
    Blockchain = "Blockchain"

class TestEnum(str, Enum):
    
    A = "A"
    B = "B"
    C = "C"
    D = "D"
    E = "E"
    F = "F"
    G = "G"

class BigNumbers(BaseModel):
    
    
    a: int
    b: float

class Blah(BaseModel):
    
    
    prop4: Optional[str] = None

class BlockConstraint(BaseModel):
    
    
    foo: int
    bar: str

class BlockConstraintForParam(BaseModel):
    
    
    bcfp: int
    bcfp2: str

class BookOrder(BaseModel):
    
    
    orderId: str
    title: str
    quantity: int
    price: float

class ClassOptionalOutput(BaseModel):
    
    
    prop1: str
    prop2: str

class ClassOptionalOutput2(BaseModel):
    
    
    prop1: Optional[str] = None
    prop2: Optional[str] = None
    prop3: Optional["Blah"] = None

class ClassWithImage(BaseModel):
    
    
    myImage: baml_py.Image
    param2: str
    fake_image: "FakeImage"

class CompoundBigNumbers(BaseModel):
    
    
    big: "BigNumbers"
    big_nums: List["BigNumbers"]
    another: "BigNumbers"

class ContactInfo(BaseModel):
    
    
    primary: Union["PhoneNumber", "EmailAddress"]
    secondary: Union["PhoneNumber", "EmailAddress", None]

class CustomTaskResult(BaseModel):
    
    
    bookOrder: Union["BookOrder", Optional[None]]
    flightConfirmation: Union["FlightConfirmation", Optional[None]]
    groceryReceipt: Union["GroceryReceipt", Optional[None]]

class DummyOutput(BaseModel):
    
    model_config = ConfigDict(extra='allow')
    
    nonce: str
    nonce2: str

class DynInputOutput(BaseModel):
    
    model_config = ConfigDict(extra='allow')
    
    testKey: str

class DynamicClassOne(BaseModel):
    
    model_config = ConfigDict(extra='allow')
    

class DynamicClassTwo(BaseModel):
    
    model_config = ConfigDict(extra='allow')
    
    hi: str
    some_class: "SomeClassNestedDynamic"
    status: Union["DynEnumOne", str]

class DynamicOutput(BaseModel):
    
    model_config = ConfigDict(extra='allow')
    

class Earthling(BaseModel):
    
    
    age: Checked[int,Literal["earth_aged", "no_infants"]]

class Education(BaseModel):
    
    
    institution: str
    location: str
    degree: str
    major: List[str]
    graduation_date: Optional[str] = None

class Email(BaseModel):
    
    
    subject: str
    body: str
    from_address: str

class EmailAddress(BaseModel):
    
    
    value: str

class Event(BaseModel):
    
    
    title: str
    date: str
    location: str
    description: str

class FakeImage(BaseModel):
    
    
    url: str

class FlightConfirmation(BaseModel):
    
    
    confirmationNumber: str
    flightNumber: str
    departureTime: str
    arrivalTime: str
    seatNumber: str

class FooAny(BaseModel):
    
    
    planetary_age: Union["Martian", "Earthling"]
    certainty: Checked[int,Literal["unreasonably_certain"]]
    species: Checked[str,Literal["regex_bad", "regex_good", "trivial"]]

class GroceryReceipt(BaseModel):
    
    
    receiptId: str
    storeName: str
    items: List[Union[str, int, float]]
    totalAmount: float

class InnerClass(BaseModel):
    
    
    prop1: str
    prop2: str
    inner: "InnerClass2"

class InnerClass2(BaseModel):
    
    
    prop2: int
    prop3: float

class InputClass(BaseModel):
    
    
    key: str
    key2: str

class InputClassNested(BaseModel):
    
    
    key: str
    nested: "InputClass"

class LiteralClassHello(BaseModel):
    
    
    prop: Literal["hello"]

class LiteralClassOne(BaseModel):
    
    
    prop: Literal["one"]

class LiteralClassTwo(BaseModel):
    
    
    prop: Literal["two"]

class MalformedConstraints(BaseModel):
    
    
    foo: Checked[int,Literal["foo_check"]]

class MalformedConstraints2(BaseModel):
    
    
    foo: int

class Martian(BaseModel):
    
    
    age: Checked[int,Literal["young_enough"]]

class NamedArgsSingleClass(BaseModel):
    
    
    key: str
    key_two: bool
    key_three: int

class Nested(BaseModel):
    
    
    prop3: Union[str, Optional[None]]
    prop4: Union[str, Optional[None]]
    prop20: "Nested2"

class Nested2(BaseModel):
    
    
    prop11: Union[str, Optional[None]]
    prop12: Union[str, Optional[None]]

class NestedBlockConstraint(BaseModel):
    
    
    nbc: Checked["BlockConstraint",Literal["cross_field"]]

class NestedBlockConstraintForParam(BaseModel):
    
    
    nbcfp: "BlockConstraintForParam"

class OptionalTest_Prop1(BaseModel):
    
    
    omega_a: str
    omega_b: int

class OptionalTest_ReturnType(BaseModel):
    
    
    omega_1: Optional["OptionalTest_Prop1"] = None
    omega_2: Optional[str] = None
    omega_3: List[Optional["OptionalTest_CategoryType"]]

class OrderInfo(BaseModel):
    
    
    order_status: "OrderStatus"
    tracking_number: Optional[str] = None
    estimated_arrival_date: Optional[str] = None

class OriginalA(BaseModel):
    
    
    value: int

class OriginalB(BaseModel):
    
    model_config = ConfigDict(extra='allow')
    
    value: int

class Person(BaseModel):
    
    model_config = ConfigDict(extra='allow')
    
    name: Optional[str] = None
    hair_color: Optional[Union["Color", str]] = None

class PhoneNumber(BaseModel):
    
    
    value: str

class Quantity(BaseModel):
    
    
    amount: Union[int, float]
    unit: Optional[str] = None

class RaysData(BaseModel):
    
    
    dataType: "DataType"
    value: Union["Resume", "Event"]

class ReceiptInfo(BaseModel):
    
    
    items: List["ReceiptItem"]
    total_cost: Optional[float] = None
    venue: Union[Literal["barisa"], Literal["ox_burger"]]

class ReceiptItem(BaseModel):
    
    
    name: str
    description: Optional[str] = None
    quantity: int
    price: float

class Recipe(BaseModel):
    
    
    ingredients: Dict[str, "Quantity"]
    recipe_type: Union[Literal["breakfast"], Literal["dinner"]]

class Resume(BaseModel):
    
    
    name: str
    email: str
    phone: str
    experience: List["Education"]
    education: List[str]
    skills: List[str]

class Schema(BaseModel):
    
    
    prop1: Union[str, Optional[None]]
    prop2: Union["Nested", str]
    prop5: List[Union[str, Optional[None]]]
    prop6: Union[str, List["Nested"]]
    nested_attrs: List[Union[str, Optional[None], "Nested"]]
    parens: Union[str, Optional[None]]
    other_group: Union[str, Union[int, str]]

class SearchParams(BaseModel):
    
    
    dateRange: Optional[int] = None
    location: List[str]
    jobTitle: Optional["WithReasoning"] = None
    company: Optional["WithReasoning"] = None
    description: List["WithReasoning"]
    tags: List[Union["Tag", str]]

class SomeClassNestedDynamic(BaseModel):
    
    model_config = ConfigDict(extra='allow')
    
    hi: str

class StringToClassEntry(BaseModel):
    
    
    word: str

class TestClassAlias(BaseModel):
    
    
    key: str
    key2: str
    key3: str
    key4: str
    key5: str

class TestClassNested(BaseModel):
    
    
    prop1: str
    prop2: "InnerClass"

class TestClassWithEnum(BaseModel):
    
    
    prop1: str
    prop2: "EnumInClass"

class TestOutputClass(BaseModel):
    
    
    prop1: str
    prop2: int

class TwoStoriesOneTitle(BaseModel):
    
    
    title: str
    story_a: str
    story_b: str

class UnionTest_ReturnType(BaseModel):
    
    
    prop1: Union[str, bool]
    prop2: List[Union[float, bool]]
    prop3: Union[List[bool], List[int]]

class WithReasoning(BaseModel):
    
    
    value: str
    reasoning: str
