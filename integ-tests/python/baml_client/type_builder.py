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
import typing
from baml_py.type_builder import FieldType, TypeBuilder as _TypeBuilder, ClassPropertyBuilder, EnumValueBuilder, EnumBuilder, ClassBuilder

class TypeBuilder(_TypeBuilder):
    def __init__(self):
        super().__init__(classes=set(
          ["Blah","ClassOptionalOutput","ClassOptionalOutput2","ClassWithImage","DynInputOutput","DynamicClassOne","DynamicClassTwo","DynamicOutput","Education","Email","Event","FakeImage","InnerClass","InnerClass2","NamedArgsSingleClass","OptionalTest_Prop1","OptionalTest_ReturnType","OrderInfo","Person","RaysData","ReceiptInfo","ReceiptItem","Response","Resume","SearchParams","SomeClassNestedDynamic","TestClassAlias","TestClassNested","TestClassWithEnum","TestOutputClass","UnionTest_ReturnType","WithReasoning",]
        ), enums=set(
          ["Category","Category2","Category3","Color","DataType","DynEnumOne","DynEnumTwo","EnumInClass","EnumOutput","Hobby","NamedArgsSingleEnum","NamedArgsSingleEnumList","OptionalTest_CategoryType","OrderStatus","Tag","TestEnum",]
        ))



    @property
    def DynInputOutput(self) -> "DynInputOutputBuilder":
        return DynInputOutputBuilder(self)


    @property
    def DynamicClassOne(self) -> "DynamicClassOneBuilder":
        return DynamicClassOneBuilder(self)


    @property
    def DynamicClassTwo(self) -> "DynamicClassTwoBuilder":
        return DynamicClassTwoBuilder(self)


    @property
    def DynamicOutput(self) -> "DynamicOutputBuilder":
        return DynamicOutputBuilder(self)


    @property
    def Person(self) -> "PersonBuilder":
        return PersonBuilder(self)


    @property
    def SomeClassNestedDynamic(self) -> "SomeClassNestedDynamicBuilder":
        return SomeClassNestedDynamicBuilder(self)




    @property
    def Color(self) -> "ColorBuilder":
        return ColorBuilder(self)


    @property
    def DynEnumOne(self) -> "DynEnumOneBuilder":
        return DynEnumOneBuilder(self)


    @property
    def DynEnumTwo(self) -> "DynEnumTwoBuilder":
        return DynEnumTwoBuilder(self)


    @property
    def Hobby(self) -> "HobbyBuilder":
        return HobbyBuilder(self)


class DynInputOutputBuilder:
    def __init__(self, tb: _TypeBuilder):
        self.__bldr = tb._tb.class_("DynInputOutput")
        self.__properties = set([ "testKey", ])
        self.__props = DynInputOutputProperties(self.__bldr, self.__properties)

    def type(self) -> FieldType:
        return self.__bldr.field()

    @property
    def props(self) -> "DynInputOutputProperties":
        return self.__props
    
    def list_properties(self) -> typing.List[typing.Tuple[str, ClassPropertyBuilder]]:
        return [(name, self.__bldr.property(name)) for name in self.__properties]

    def add_property(self, name: str, type: FieldType) -> ClassPropertyBuilder:
        if name in self.__properties:
            raise ValueError(f"Property {name} already exists.")
        return ClassPropertyBuilder(self.__bldr.property(name).type(type))

class DynInputOutputProperties:
    def __init__(self, cls_bldr: ClassBuilder, properties: typing.Set[str]):
        self.__bldr = cls_bldr
        self.__properties = properties

    

    @property
    def testKey(self) -> ClassPropertyBuilder:
        return self.__bldr.property("testKey")

    def __getattr__(self, name: str) -> ClassPropertyBuilder:
        if name not in self.__properties:
            raise AttributeError(f"Property {name} not found.")
        return ClassPropertyBuilder(self.__bldr.property(name))
class DynamicClassOneBuilder:
    def __init__(self, tb: _TypeBuilder):
        self.__bldr = tb._tb.class_("DynamicClassOne")
        self.__properties = set([])
        self.__props = DynamicClassOneProperties(self.__bldr, self.__properties)

    def type(self) -> FieldType:
        return self.__bldr.field()

    @property
    def props(self) -> "DynamicClassOneProperties":
        return self.__props
    
    def list_properties(self) -> typing.List[typing.Tuple[str, ClassPropertyBuilder]]:
        return [(name, self.__bldr.property(name)) for name in self.__properties]

    def add_property(self, name: str, type: FieldType) -> ClassPropertyBuilder:
        if name in self.__properties:
            raise ValueError(f"Property {name} already exists.")
        return ClassPropertyBuilder(self.__bldr.property(name).type(type))

class DynamicClassOneProperties:
    def __init__(self, cls_bldr: ClassBuilder, properties: typing.Set[str]):
        self.__bldr = cls_bldr
        self.__properties = properties

    

    def __getattr__(self, name: str) -> ClassPropertyBuilder:
        if name not in self.__properties:
            raise AttributeError(f"Property {name} not found.")
        return ClassPropertyBuilder(self.__bldr.property(name))
class DynamicClassTwoBuilder:
    def __init__(self, tb: _TypeBuilder):
        self.__bldr = tb._tb.class_("DynamicClassTwo")
        self.__properties = set([ "hi",  "some_class",  "status", ])
        self.__props = DynamicClassTwoProperties(self.__bldr, self.__properties)

    def type(self) -> FieldType:
        return self.__bldr.field()

    @property
    def props(self) -> "DynamicClassTwoProperties":
        return self.__props
    
    def list_properties(self) -> typing.List[typing.Tuple[str, ClassPropertyBuilder]]:
        return [(name, self.__bldr.property(name)) for name in self.__properties]

    def add_property(self, name: str, type: FieldType) -> ClassPropertyBuilder:
        if name in self.__properties:
            raise ValueError(f"Property {name} already exists.")
        return ClassPropertyBuilder(self.__bldr.property(name).type(type))

class DynamicClassTwoProperties:
    def __init__(self, cls_bldr: ClassBuilder, properties: typing.Set[str]):
        self.__bldr = cls_bldr
        self.__properties = properties

    

    @property
    def hi(self) -> ClassPropertyBuilder:
        return self.__bldr.property("hi")

    @property
    def some_class(self) -> ClassPropertyBuilder:
        return self.__bldr.property("some_class")

    @property
    def status(self) -> ClassPropertyBuilder:
        return self.__bldr.property("status")

    def __getattr__(self, name: str) -> ClassPropertyBuilder:
        if name not in self.__properties:
            raise AttributeError(f"Property {name} not found.")
        return ClassPropertyBuilder(self.__bldr.property(name))
class DynamicOutputBuilder:
    def __init__(self, tb: _TypeBuilder):
        self.__bldr = tb._tb.class_("DynamicOutput")
        self.__properties = set([])
        self.__props = DynamicOutputProperties(self.__bldr, self.__properties)

    def type(self) -> FieldType:
        return self.__bldr.field()

    @property
    def props(self) -> "DynamicOutputProperties":
        return self.__props
    
    def list_properties(self) -> typing.List[typing.Tuple[str, ClassPropertyBuilder]]:
        return [(name, self.__bldr.property(name)) for name in self.__properties]

    def add_property(self, name: str, type: FieldType) -> ClassPropertyBuilder:
        if name in self.__properties:
            raise ValueError(f"Property {name} already exists.")
        return ClassPropertyBuilder(self.__bldr.property(name).type(type))

class DynamicOutputProperties:
    def __init__(self, cls_bldr: ClassBuilder, properties: typing.Set[str]):
        self.__bldr = cls_bldr
        self.__properties = properties

    

    def __getattr__(self, name: str) -> ClassPropertyBuilder:
        if name not in self.__properties:
            raise AttributeError(f"Property {name} not found.")
        return ClassPropertyBuilder(self.__bldr.property(name))
class PersonBuilder:
    def __init__(self, tb: _TypeBuilder):
        self.__bldr = tb._tb.class_("Person")
        self.__properties = set([ "name",  "hair_color", ])
        self.__props = PersonProperties(self.__bldr, self.__properties)

    def type(self) -> FieldType:
        return self.__bldr.field()

    @property
    def props(self) -> "PersonProperties":
        return self.__props
    
    def list_properties(self) -> typing.List[typing.Tuple[str, ClassPropertyBuilder]]:
        return [(name, self.__bldr.property(name)) for name in self.__properties]

    def add_property(self, name: str, type: FieldType) -> ClassPropertyBuilder:
        if name in self.__properties:
            raise ValueError(f"Property {name} already exists.")
        return ClassPropertyBuilder(self.__bldr.property(name).type(type))

class PersonProperties:
    def __init__(self, cls_bldr: ClassBuilder, properties: typing.Set[str]):
        self.__bldr = cls_bldr
        self.__properties = properties

    

    @property
    def name(self) -> ClassPropertyBuilder:
        return self.__bldr.property("name")

    @property
    def hair_color(self) -> ClassPropertyBuilder:
        return self.__bldr.property("hair_color")

    def __getattr__(self, name: str) -> ClassPropertyBuilder:
        if name not in self.__properties:
            raise AttributeError(f"Property {name} not found.")
        return ClassPropertyBuilder(self.__bldr.property(name))
class SomeClassNestedDynamicBuilder:
    def __init__(self, tb: _TypeBuilder):
        self.__bldr = tb._tb.class_("SomeClassNestedDynamic")
        self.__properties = set([ "hi", ])
        self.__props = SomeClassNestedDynamicProperties(self.__bldr, self.__properties)

    def type(self) -> FieldType:
        return self.__bldr.field()

    @property
    def props(self) -> "SomeClassNestedDynamicProperties":
        return self.__props
    
    def list_properties(self) -> typing.List[typing.Tuple[str, ClassPropertyBuilder]]:
        return [(name, self.__bldr.property(name)) for name in self.__properties]

    def add_property(self, name: str, type: FieldType) -> ClassPropertyBuilder:
        if name in self.__properties:
            raise ValueError(f"Property {name} already exists.")
        return ClassPropertyBuilder(self.__bldr.property(name).type(type))

class SomeClassNestedDynamicProperties:
    def __init__(self, cls_bldr: ClassBuilder, properties: typing.Set[str]):
        self.__bldr = cls_bldr
        self.__properties = properties

    

    @property
    def hi(self) -> ClassPropertyBuilder:
        return self.__bldr.property("hi")

    def __getattr__(self, name: str) -> ClassPropertyBuilder:
        if name not in self.__properties:
            raise AttributeError(f"Property {name} not found.")
        return ClassPropertyBuilder(self.__bldr.property(name))



class ColorBuilder:
    def __init__(self, tb: _TypeBuilder):
        self.__bldr = tb._tb.enum("Color")
        self.__values = set([ "RED",  "BLUE",  "GREEN",  "YELLOW",  "BLACK",  "WHITE", ])
        self.__vals = ColorValues(self.__bldr, self.__values)

    def type(self) -> FieldType:
        return self.__bldr.field()

    @property
    def values(self) -> "ColorValues":
        return self.__vals

    def list_values(self) -> typing.List[typing.Tuple[str, EnumValueBuilder]]:
        return [(name, self.__bldr.value(name)) for name in self.__values]

    def add_value(self, name: str) -> EnumValueBuilder:
        if name in self.__values:
            raise ValueError(f"Value {name} already exists.")
        self.__values.add(name)
        return self.__bldr.value(name)

class ColorValues:
    def __init__(self, enum_bldr: EnumBuilder, values: typing.Set[str]):
        self.__bldr = enum_bldr
        self.__values = values

    

    @property
    def RED(self) -> EnumValueBuilder:
        return self.__bldr.value("RED")
    

    @property
    def BLUE(self) -> EnumValueBuilder:
        return self.__bldr.value("BLUE")
    

    @property
    def GREEN(self) -> EnumValueBuilder:
        return self.__bldr.value("GREEN")
    

    @property
    def YELLOW(self) -> EnumValueBuilder:
        return self.__bldr.value("YELLOW")
    

    @property
    def BLACK(self) -> EnumValueBuilder:
        return self.__bldr.value("BLACK")
    

    @property
    def WHITE(self) -> EnumValueBuilder:
        return self.__bldr.value("WHITE")
    

    def __getattr__(self, name: str) -> EnumValueBuilder:
        if name not in self.__values:
            raise AttributeError(f"Value {name} not found.")
        return self.__bldr.value(name)

class DynEnumOneBuilder:
    def __init__(self, tb: _TypeBuilder):
        self.__bldr = tb._tb.enum("DynEnumOne")
        self.__values = set([])
        self.__vals = DynEnumOneValues(self.__bldr, self.__values)

    def type(self) -> FieldType:
        return self.__bldr.field()

    @property
    def values(self) -> "DynEnumOneValues":
        return self.__vals

    def list_values(self) -> typing.List[typing.Tuple[str, EnumValueBuilder]]:
        return [(name, self.__bldr.value(name)) for name in self.__values]

    def add_value(self, name: str) -> EnumValueBuilder:
        if name in self.__values:
            raise ValueError(f"Value {name} already exists.")
        self.__values.add(name)
        return self.__bldr.value(name)

class DynEnumOneValues:
    def __init__(self, enum_bldr: EnumBuilder, values: typing.Set[str]):
        self.__bldr = enum_bldr
        self.__values = values

    

    def __getattr__(self, name: str) -> EnumValueBuilder:
        if name not in self.__values:
            raise AttributeError(f"Value {name} not found.")
        return self.__bldr.value(name)

class DynEnumTwoBuilder:
    def __init__(self, tb: _TypeBuilder):
        self.__bldr = tb._tb.enum("DynEnumTwo")
        self.__values = set([])
        self.__vals = DynEnumTwoValues(self.__bldr, self.__values)

    def type(self) -> FieldType:
        return self.__bldr.field()

    @property
    def values(self) -> "DynEnumTwoValues":
        return self.__vals

    def list_values(self) -> typing.List[typing.Tuple[str, EnumValueBuilder]]:
        return [(name, self.__bldr.value(name)) for name in self.__values]

    def add_value(self, name: str) -> EnumValueBuilder:
        if name in self.__values:
            raise ValueError(f"Value {name} already exists.")
        self.__values.add(name)
        return self.__bldr.value(name)

class DynEnumTwoValues:
    def __init__(self, enum_bldr: EnumBuilder, values: typing.Set[str]):
        self.__bldr = enum_bldr
        self.__values = values

    

    def __getattr__(self, name: str) -> EnumValueBuilder:
        if name not in self.__values:
            raise AttributeError(f"Value {name} not found.")
        return self.__bldr.value(name)

class HobbyBuilder:
    def __init__(self, tb: _TypeBuilder):
        self.__bldr = tb._tb.enum("Hobby")
        self.__values = set([ "SPORTS",  "MUSIC",  "READING", ])
        self.__vals = HobbyValues(self.__bldr, self.__values)

    def type(self) -> FieldType:
        return self.__bldr.field()

    @property
    def values(self) -> "HobbyValues":
        return self.__vals

    def list_values(self) -> typing.List[typing.Tuple[str, EnumValueBuilder]]:
        return [(name, self.__bldr.value(name)) for name in self.__values]

    def add_value(self, name: str) -> EnumValueBuilder:
        if name in self.__values:
            raise ValueError(f"Value {name} already exists.")
        self.__values.add(name)
        return self.__bldr.value(name)

class HobbyValues:
    def __init__(self, enum_bldr: EnumBuilder, values: typing.Set[str]):
        self.__bldr = enum_bldr
        self.__values = values

    

    @property
    def SPORTS(self) -> EnumValueBuilder:
        return self.__bldr.value("SPORTS")
    

    @property
    def MUSIC(self) -> EnumValueBuilder:
        return self.__bldr.value("MUSIC")
    

    @property
    def READING(self) -> EnumValueBuilder:
        return self.__bldr.value("READING")
    

    def __getattr__(self, name: str) -> EnumValueBuilder:
        if name not in self.__values:
            raise AttributeError(f"Value {name} not found.")
        return self.__bldr.value(name)


__all__ = ["TypeBuilder"]