###############################################################################
#
#  Welcome to Baml! To use this generated code, please run the following:
#
#  $ bundle add baml sorbet-runtime sorbet-struct-comparable
#
###############################################################################

# This file was generated by BAML: please do not edit it. Instead, edit the
# BAML files and re-generate this code.
#
# frozen_string_literal: true
# rubocop: disable
# formatter:off
# typed: false
require "sorbet-runtime"
require "sorbet-struct-comparable"

module Baml
  
  module Types
    class Category < T::Enum
      enums do
        Refund = new("Refund")
        CancelOrder = new("CancelOrder")
        TechnicalSupport = new("TechnicalSupport")
        AccountIssue = new("AccountIssue")
        Question = new("Question")
      end
    end
    class Category2 < T::Enum
      enums do
        Refund = new("Refund")
        CancelOrder = new("CancelOrder")
        TechnicalSupport = new("TechnicalSupport")
        AccountIssue = new("AccountIssue")
        Question = new("Question")
      end
    end
    class Category3 < T::Enum
      enums do
        Refund = new("Refund")
        CancelOrder = new("CancelOrder")
        TechnicalSupport = new("TechnicalSupport")
        AccountIssue = new("AccountIssue")
        Question = new("Question")
      end
    end
    class Color < T::Enum
      enums do
        RED = new("RED")
        BLUE = new("BLUE")
        GREEN = new("GREEN")
        YELLOW = new("YELLOW")
        BLACK = new("BLACK")
        WHITE = new("WHITE")
      end
    end
    class DataType < T::Enum
      enums do
        Resume = new("Resume")
        Event = new("Event")
      end
    end
    class DynEnumOne < T::Enum
    end
    class DynEnumTwo < T::Enum
    end
    class EnumInClass < T::Enum
      enums do
        ONE = new("ONE")
        TWO = new("TWO")
      end
    end
    class EnumOutput < T::Enum
      enums do
        ONE = new("ONE")
        TWO = new("TWO")
        THREE = new("THREE")
      end
    end
    class Hobby < T::Enum
      enums do
        SPORTS = new("SPORTS")
        MUSIC = new("MUSIC")
        READING = new("READING")
      end
    end
    class NamedArgsSingleEnum < T::Enum
      enums do
        ONE = new("ONE")
        TWO = new("TWO")
      end
    end
    class NamedArgsSingleEnumList < T::Enum
      enums do
        ONE = new("ONE")
        TWO = new("TWO")
      end
    end
    class OptionalTest_CategoryType < T::Enum
      enums do
        Aleph = new("Aleph")
        Beta = new("Beta")
        Gamma = new("Gamma")
      end
    end
    class OrderStatus < T::Enum
      enums do
        ORDERED = new("ORDERED")
        SHIPPED = new("SHIPPED")
        DELIVERED = new("DELIVERED")
        CANCELLED = new("CANCELLED")
      end
    end
    class Tag < T::Enum
      enums do
        Security = new("Security")
        AI = new("AI")
        Blockchain = new("Blockchain")
      end
    end
    class TestEnum < T::Enum
      enums do
        A = new("A")
        B = new("B")
        C = new("C")
        D = new("D")
        E = new("E")
        F = new("F")
        G = new("G")
      end
    end
    class Blah < T::Struct; end
    class ClassOptionalOutput < T::Struct; end
    class ClassOptionalOutput2 < T::Struct; end
    class ClassWithImage < T::Struct; end
    class DynInputOutput < T::Struct; end
    class DynamicClassOne < T::Struct; end
    class DynamicClassTwo < T::Struct; end
    class DynamicOutput < T::Struct; end
    class Education < T::Struct; end
    class Email < T::Struct; end
    class Event < T::Struct; end
    class FakeImage < T::Struct; end
    class InnerClass < T::Struct; end
    class InnerClass2 < T::Struct; end
    class NamedArgsSingleClass < T::Struct; end
    class OptionalTest_Prop1 < T::Struct; end
    class OptionalTest_ReturnType < T::Struct; end
    class OrderInfo < T::Struct; end
    class Person < T::Struct; end
    class RaysData < T::Struct; end
    class ReceiptInfo < T::Struct; end
    class ReceiptItem < T::Struct; end
    class Response < T::Struct; end
    class Resume < T::Struct; end
    class SearchParams < T::Struct; end
    class SomeClassNestedDynamic < T::Struct; end
    class TestClassAlias < T::Struct; end
    class TestClassNested < T::Struct; end
    class TestClassWithEnum < T::Struct; end
    class TestOutputClass < T::Struct; end
    class UnionTest_ReturnType < T::Struct; end
    class WithReasoning < T::Struct; end
    class Blah < T::Struct
      include T::Struct::ActsAsComparable
      const :prop4, T.nilable(String)
    end
    class ClassOptionalOutput < T::Struct
      include T::Struct::ActsAsComparable
      const :prop1, String
      const :prop2, String
    end
    class ClassOptionalOutput2 < T::Struct
      include T::Struct::ActsAsComparable
      const :prop1, T.nilable(String)
      const :prop2, T.nilable(String)
      const :prop3, T.nilable(Baml::Types::Blah)
    end
    class ClassWithImage < T::Struct
      include T::Struct::ActsAsComparable
      const :myImage, Baml::Image
      const :param2, String
      const :fake_image, Baml::Types::FakeImage
    end
    class DynInputOutput < T::Struct
      include T::Struct::ActsAsComparable
      const :testKey, String
    end
    class DynamicClassOne < T::Struct
      include T::Struct::ActsAsComparable
    end
    class DynamicClassTwo < T::Struct
      include T::Struct::ActsAsComparable
      const :hi, String
      const :some_class, Baml::Types::SomeClassNestedDynamic
      const :status, Baml::Types::DynEnumOne
    end
    class DynamicOutput < T::Struct
      include T::Struct::ActsAsComparable
    end
    class Education < T::Struct
      include T::Struct::ActsAsComparable
      const :institution, String
      const :location, String
      const :degree, String
      const :major, T::Array[String]
      const :date, T.nilable(String)
    end
    class Email < T::Struct
      include T::Struct::ActsAsComparable
      const :subject, String
      const :body, String
      const :from_address, String
    end
    class Event < T::Struct
      include T::Struct::ActsAsComparable
      const :title, String
      const :date, String
      const :location, String
      const :description, String
    end
    class FakeImage < T::Struct
      include T::Struct::ActsAsComparable
      const :url, String
    end
    class InnerClass < T::Struct
      include T::Struct::ActsAsComparable
      const :prop1, String
      const :prop2, String
      const :inner, Baml::Types::InnerClass2
    end
    class InnerClass2 < T::Struct
      include T::Struct::ActsAsComparable
      const :prop2, Integer
      const :prop3, Float
    end
    class NamedArgsSingleClass < T::Struct
      include T::Struct::ActsAsComparable
      const :key, String
      const :key_two, T::Boolean
      const :key_three, Integer
    end
    class OptionalTest_Prop1 < T::Struct
      include T::Struct::ActsAsComparable
      const :omega_a, String
      const :omega_b, Integer
    end
    class OptionalTest_ReturnType < T::Struct
      include T::Struct::ActsAsComparable
      const :omega_1, T.nilable(Baml::Types::OptionalTest_Prop1)
      const :omega_2, T.nilable(String)
      const :omega_3, T::Array[T.nilable(Baml::Types::OptionalTest_CategoryType)]
    end
    class OrderInfo < T::Struct
      include T::Struct::ActsAsComparable
      const :order_status, Baml::Types::OrderStatus
      const :tracking_number, T.nilable(String)
      const :estimated_arrival_date, T.nilable(String)
    end
    class Person < T::Struct
      include T::Struct::ActsAsComparable
      const :name, T.nilable(String)
      const :hair_color, T.nilable(Baml::Types::Color)
    end
    class RaysData < T::Struct
      include T::Struct::ActsAsComparable
      const :dataType, Baml::Types::DataType
      const :value, T.any(Baml::Types::Resume, Baml::Types::Event)
    end
    class ReceiptInfo < T::Struct
      include T::Struct::ActsAsComparable
      const :items, T::Array[Baml::Types::ReceiptItem]
      const :total_cost, T.nilable(Float)
    end
    class ReceiptItem < T::Struct
      include T::Struct::ActsAsComparable
      const :name, String
      const :description, T.nilable(String)
      const :quantity, Integer
      const :price, Float
    end
    class Response < T::Struct
      include T::Struct::ActsAsComparable
      const :type, String
      const :reason, String
      const :appropriate_for_video_games, T::Boolean
      const :score, Integer
    end
    class Resume < T::Struct
      include T::Struct::ActsAsComparable
      const :name, String
      const :email, String
      const :phone, String
      const :experience, T::Array[String]
      const :education, T::Array[Baml::Types::Education]
      const :skills, T::Array[String]
    end
    class SearchParams < T::Struct
      include T::Struct::ActsAsComparable
      const :dateRange, T.nilable(Integer)
      const :location, T::Array[String]
      const :jobTitle, T.nilable(Baml::Types::WithReasoning)
      const :company, T.nilable(Baml::Types::WithReasoning)
      const :description, T::Array[Baml::Types::WithReasoning]
      const :tags, T::Array[T.any(Baml::Types::Tag, String)]
    end
    class SomeClassNestedDynamic < T::Struct
      include T::Struct::ActsAsComparable
      const :hi, String
    end
    class TestClassAlias < T::Struct
      include T::Struct::ActsAsComparable
      const :key, String
      const :key2, String
      const :key3, String
      const :key4, String
      const :key5, String
    end
    class TestClassNested < T::Struct
      include T::Struct::ActsAsComparable
      const :prop1, String
      const :prop2, Baml::Types::InnerClass
    end
    class TestClassWithEnum < T::Struct
      include T::Struct::ActsAsComparable
      const :prop1, String
      const :prop2, Baml::Types::EnumInClass
    end
    class TestOutputClass < T::Struct
      include T::Struct::ActsAsComparable
      const :prop1, String
      const :prop2, Integer
    end
    class UnionTest_ReturnType < T::Struct
      include T::Struct::ActsAsComparable
      const :prop1, T.any(String, T::Boolean)
      const :prop2, T::Array[T.any(Float, T::Boolean)]
      const :prop3, T.any(T::Array[T::Boolean], T::Array[Integer])
    end
    class WithReasoning < T::Struct
      include T::Struct::ActsAsComparable
      const :value, String
      const :reasoning, String
    end
  end
end