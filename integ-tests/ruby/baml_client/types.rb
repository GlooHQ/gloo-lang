###############################################################################
#
#  Welcome to Baml! To use this generated code, please run the following:
#
#  $ bundle add baml sorbet-runtime
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

module Baml
  
  module Types
    class AliasedEnum < T::Enum
      enums do
        KEY_ONE = new("KEY_ONE")
        KEY_TWO = new("KEY_TWO")
      end
    end
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
      # An enum with three values,
      # ONE, TWO and THREE.
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
    class MapKey < T::Enum
      enums do
        A = new("A")
        B = new("B")
        C = new("C")
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
    class BigNumbers < T::Struct; end
    class BinaryNode < T::Struct; end
    class Blah < T::Struct; end
    class BlockConstraint < T::Struct; end
    class BlockConstraintForParam < T::Struct; end
    class BookOrder < T::Struct; end
    class ClassOptionalOutput < T::Struct; end
    class ClassOptionalOutput2 < T::Struct; end
    class ClassToRecAlias < T::Struct; end
    class ClassWithImage < T::Struct; end
    class CompoundBigNumbers < T::Struct; end
    class ContactInfo < T::Struct; end
    class CustomTaskResult < T::Struct; end
    class DummyOutput < T::Struct; end
    class DynInputOutput < T::Struct; end
    class DynamicClassOne < T::Struct; end
    class DynamicClassTwo < T::Struct; end
    class DynamicOutput < T::Struct; end
    class Earthling < T::Struct; end
    class Education < T::Struct; end
    class Email < T::Struct; end
    class EmailAddress < T::Struct; end
    class Event < T::Struct; end
    class FakeImage < T::Struct; end
    class FlightConfirmation < T::Struct; end
    class FooAny < T::Struct; end
    class Forest < T::Struct; end
    class GroceryReceipt < T::Struct; end
    class InnerClass < T::Struct; end
    class InnerClass2 < T::Struct; end
    class InputClass < T::Struct; end
    class InputClassNested < T::Struct; end
    class LinkedList < T::Struct; end
    class LinkedListAliasNode < T::Struct; end
    class LiteralClassHello < T::Struct; end
    class LiteralClassOne < T::Struct; end
    class LiteralClassTwo < T::Struct; end
    class MalformedConstraints < T::Struct; end
    class MalformedConstraints2 < T::Struct; end
    class Martian < T::Struct; end
    class MergeAttrs < T::Struct; end
    class NamedArgsSingleClass < T::Struct; end
    class Nested < T::Struct; end
    class Nested2 < T::Struct; end
    class NestedBlockConstraint < T::Struct; end
    class NestedBlockConstraintForParam < T::Struct; end
    class Node < T::Struct; end
    class NodeWithAliasIndirection < T::Struct; end
    class OptionalListAndMap < T::Struct; end
    class OptionalTest_Prop1 < T::Struct; end
    class OptionalTest_ReturnType < T::Struct; end
    class OrderInfo < T::Struct; end
    class OriginalA < T::Struct; end
    class OriginalB < T::Struct; end
    class Person < T::Struct; end
    class PhoneNumber < T::Struct; end
    class Quantity < T::Struct; end
    class RaysData < T::Struct; end
    class ReceiptInfo < T::Struct; end
    class ReceiptItem < T::Struct; end
    class Recipe < T::Struct; end
    class Resume < T::Struct; end
    class Schema < T::Struct; end
    class SearchParams < T::Struct; end
    class SomeClassNestedDynamic < T::Struct; end
    class StringToClassEntry < T::Struct; end
    class TestClassAlias < T::Struct; end
    class TestClassNested < T::Struct; end
    class TestClassWithEnum < T::Struct; end
    class TestOutputClass < T::Struct; end
    class Tree < T::Struct; end
    class TwoStoriesOneTitle < T::Struct; end
    class UnionTest_ReturnType < T::Struct; end
    class WithReasoning < T::Struct; end
    class BigNumbers < T::Struct
      include Baml::Sorbet::Struct
      const :a, Integer
      const :b, Float

      def initialize(props)
        super(
          a: props[:a],
          b: props[:b],
        )

        @props = props
      end
    end
    class BinaryNode < T::Struct
      include Baml::Sorbet::Struct
      const :data, Integer
      const :left, T.nilable(Baml::Types::BinaryNode)
      const :right, T.nilable(Baml::Types::BinaryNode)

      def initialize(props)
        super(
          data: props[:data],
          left: props[:left],
          right: props[:right],
        )

        @props = props
      end
    end
    class Blah < T::Struct
      include Baml::Sorbet::Struct
      const :prop4, T.nilable(String)

      def initialize(props)
        super(
          prop4: props[:prop4],
        )

        @props = props
      end
    end
    class BlockConstraint < T::Struct
      include Baml::Sorbet::Struct
      const :foo, Integer
      const :bar, String

      def initialize(props)
        super(
          foo: props[:foo],
          bar: props[:bar],
        )

        @props = props
      end
    end
    class BlockConstraintForParam < T::Struct
      include Baml::Sorbet::Struct
      const :bcfp, Integer
      const :bcfp2, String

      def initialize(props)
        super(
          bcfp: props[:bcfp],
          bcfp2: props[:bcfp2],
        )

        @props = props
      end
    end
    class BookOrder < T::Struct
      include Baml::Sorbet::Struct
      const :orderId, String
      const :title, String
      const :quantity, Integer
      const :price, Float

      def initialize(props)
        super(
          orderId: props[:orderId],
          title: props[:title],
          quantity: props[:quantity],
          price: props[:price],
        )

        @props = props
      end
    end
    class ClassOptionalOutput < T::Struct
      include Baml::Sorbet::Struct
      const :prop1, String
      const :prop2, String

      def initialize(props)
        super(
          prop1: props[:prop1],
          prop2: props[:prop2],
        )

        @props = props
      end
    end
    class ClassOptionalOutput2 < T::Struct
      include Baml::Sorbet::Struct
      const :prop1, T.nilable(String)
      const :prop2, T.nilable(String)
      const :prop3, T.nilable(Baml::Types::Blah)

      def initialize(props)
        super(
          prop1: props[:prop1],
          prop2: props[:prop2],
          prop3: props[:prop3],
        )

        @props = props
      end
    end
    class ClassToRecAlias < T::Struct
      include Baml::Sorbet::Struct
      const :list, Baml::Types::LinkedListAliasNode

      def initialize(props)
        super(
          list: props[:list],
        )

        @props = props
      end
    end
    class ClassWithImage < T::Struct
      include Baml::Sorbet::Struct
      const :myImage, Baml::Image
      const :param2, String
      const :fake_image, Baml::Types::FakeImage

      def initialize(props)
        super(
          myImage: props[:myImage],
          param2: props[:param2],
          fake_image: props[:fake_image],
        )

        @props = props
      end
    end
    class CompoundBigNumbers < T::Struct
      include Baml::Sorbet::Struct
      const :big, Baml::Types::BigNumbers
      const :big_nums, T::Array[Baml::Types::BigNumbers]
      const :another, Baml::Types::BigNumbers

      def initialize(props)
        super(
          big: props[:big],
          big_nums: props[:big_nums],
          another: props[:another],
        )

        @props = props
      end
    end
    class ContactInfo < T::Struct
      include Baml::Sorbet::Struct
      const :primary, T.any(Baml::Types::PhoneNumber, Baml::Types::EmailAddress)
      const :secondary, T.any(Baml::Types::PhoneNumber, Baml::Types::EmailAddress, NilClass)

      def initialize(props)
        super(
          primary: props[:primary],
          secondary: props[:secondary],
        )

        @props = props
      end
    end
    class CustomTaskResult < T::Struct
      include Baml::Sorbet::Struct
      const :bookOrder, T.any(Baml::Types::BookOrder, T.nilable(NilClass))
      const :flightConfirmation, T.any(Baml::Types::FlightConfirmation, T.nilable(NilClass))
      const :groceryReceipt, T.any(Baml::Types::GroceryReceipt, T.nilable(NilClass))

      def initialize(props)
        super(
          bookOrder: props[:bookOrder],
          flightConfirmation: props[:flightConfirmation],
          groceryReceipt: props[:groceryReceipt],
        )

        @props = props
      end
    end
    class DummyOutput < T::Struct
      include Baml::Sorbet::Struct
      const :nonce, String
      const :nonce2, String

      def initialize(props)
        super(
          nonce: props[:nonce],
          nonce2: props[:nonce2],
        )

        @props = props
      end
    end
    class DynInputOutput < T::Struct
      include Baml::Sorbet::Struct
      const :testKey, String

      def initialize(props)
        super(
          testKey: props[:testKey],
        )

        @props = props
      end
    end
    class DynamicClassOne < T::Struct
      include Baml::Sorbet::Struct

      def initialize(props)
        super(
        )

        @props = props
      end
    end
    class DynamicClassTwo < T::Struct
      include Baml::Sorbet::Struct
      const :hi, String
      const :some_class, Baml::Types::SomeClassNestedDynamic
      const :status, T.any(Baml::Types::DynEnumOne, String)

      def initialize(props)
        super(
          hi: props[:hi],
          some_class: props[:some_class],
          status: props[:status],
        )

        @props = props
      end
    end
    class DynamicOutput < T::Struct
      include Baml::Sorbet::Struct

      def initialize(props)
        super(
        )

        @props = props
      end
    end
    class Earthling < T::Struct
      include Baml::Sorbet::Struct
      const :age, Baml::Checked[Integer]

      def initialize(props)
        super(
          age: props[:age],
        )

        @props = props
      end
    end
    class Education < T::Struct
      include Baml::Sorbet::Struct
      const :institution, String
      const :location, String
      const :degree, String
      const :major, T::Array[String]
      const :graduation_date, T.nilable(String)

      def initialize(props)
        super(
          institution: props[:institution],
          location: props[:location],
          degree: props[:degree],
          major: props[:major],
          graduation_date: props[:graduation_date],
        )

        @props = props
      end
    end
    class Email < T::Struct
      include Baml::Sorbet::Struct
      const :subject, String
      const :body, String
      const :from_address, String

      def initialize(props)
        super(
          subject: props[:subject],
          body: props[:body],
          from_address: props[:from_address],
        )

        @props = props
      end
    end
    class EmailAddress < T::Struct
      include Baml::Sorbet::Struct
      const :value, String

      def initialize(props)
        super(
          value: props[:value],
        )

        @props = props
      end
    end
    class Event < T::Struct
      include Baml::Sorbet::Struct
      const :title, String
      const :date, String
      const :location, String
      const :description, String

      def initialize(props)
        super(
          title: props[:title],
          date: props[:date],
          location: props[:location],
          description: props[:description],
        )

        @props = props
      end
    end
    class FakeImage < T::Struct
      include Baml::Sorbet::Struct
      const :url, String

      def initialize(props)
        super(
          url: props[:url],
        )

        @props = props
      end
    end
    class FlightConfirmation < T::Struct
      include Baml::Sorbet::Struct
      const :confirmationNumber, String
      const :flightNumber, String
      const :departureTime, String
      const :arrivalTime, String
      const :seatNumber, String

      def initialize(props)
        super(
          confirmationNumber: props[:confirmationNumber],
          flightNumber: props[:flightNumber],
          departureTime: props[:departureTime],
          arrivalTime: props[:arrivalTime],
          seatNumber: props[:seatNumber],
        )

        @props = props
      end
    end
    class FooAny < T::Struct
      include Baml::Sorbet::Struct
      const :planetary_age, T.any(Baml::Types::Martian, Baml::Types::Earthling)
      const :certainty, Baml::Checked[Integer]
      const :species, Baml::Checked[String]

      def initialize(props)
        super(
          planetary_age: props[:planetary_age],
          certainty: props[:certainty],
          species: props[:species],
        )

        @props = props
      end
    end
    class Forest < T::Struct
      include Baml::Sorbet::Struct
      const :trees, T::Array[Baml::Types::Tree]

      def initialize(props)
        super(
          trees: props[:trees],
        )

        @props = props
      end
    end
    class GroceryReceipt < T::Struct
      include Baml::Sorbet::Struct
      const :receiptId, String
      const :storeName, String
      const :items, T::Array[T.any(String, Integer, Float)]
      const :totalAmount, Float

      def initialize(props)
        super(
          receiptId: props[:receiptId],
          storeName: props[:storeName],
          items: props[:items],
          totalAmount: props[:totalAmount],
        )

        @props = props
      end
    end
    class InnerClass < T::Struct
      include Baml::Sorbet::Struct
      const :prop1, String
      const :prop2, String
      const :inner, Baml::Types::InnerClass2

      def initialize(props)
        super(
          prop1: props[:prop1],
          prop2: props[:prop2],
          inner: props[:inner],
        )

        @props = props
      end
    end
    class InnerClass2 < T::Struct
      include Baml::Sorbet::Struct
      const :prop2, Integer
      const :prop3, Float

      def initialize(props)
        super(
          prop2: props[:prop2],
          prop3: props[:prop3],
        )

        @props = props
      end
    end
    class InputClass < T::Struct
      include Baml::Sorbet::Struct
      const :key, String
      const :key2, String

      def initialize(props)
        super(
          key: props[:key],
          key2: props[:key2],
        )

        @props = props
      end
    end
    class InputClassNested < T::Struct
      include Baml::Sorbet::Struct
      const :key, String
      const :nested, Baml::Types::InputClass

      def initialize(props)
        super(
          key: props[:key],
          nested: props[:nested],
        )

        @props = props
      end
    end
    class LinkedList < T::Struct
      include Baml::Sorbet::Struct
      const :head, T.nilable(Baml::Types::Node)
      const :len, Integer

      def initialize(props)
        super(
          head: props[:head],
          len: props[:len],
        )

        @props = props
      end
    end
    class LinkedListAliasNode < T::Struct
      include Baml::Sorbet::Struct
      const :value, Integer
      const :next, T.nilable(Baml::Types::LinkedListAliasNode)

      def initialize(props)
        super(
          value: props[:value],
          next: props[:next],
        )

        @props = props
      end
    end
    class LiteralClassHello < T::Struct
      include Baml::Sorbet::Struct
      const :prop, String

      def initialize(props)
        super(
          prop: props[:prop],
        )

        @props = props
      end
    end
    class LiteralClassOne < T::Struct
      include Baml::Sorbet::Struct
      const :prop, String

      def initialize(props)
        super(
          prop: props[:prop],
        )

        @props = props
      end
    end
    class LiteralClassTwo < T::Struct
      include Baml::Sorbet::Struct
      const :prop, String

      def initialize(props)
        super(
          prop: props[:prop],
        )

        @props = props
      end
    end
    class MalformedConstraints < T::Struct
      include Baml::Sorbet::Struct
      const :foo, Baml::Checked[Integer]

      def initialize(props)
        super(
          foo: props[:foo],
        )

        @props = props
      end
    end
    class MalformedConstraints2 < T::Struct
      include Baml::Sorbet::Struct
      const :foo, Integer

      def initialize(props)
        super(
          foo: props[:foo],
        )

        @props = props
      end
    end
    # A Martian organism with an age.
    # Such a nice type.
    class Martian < T::Struct
      include Baml::Sorbet::Struct
      # The age of the Martian in Mars years.
      # So many Mars years.
      const :age, Baml::Checked[Integer]

      def initialize(props)
        super(
          age: props[:age],
        )

        @props = props
      end
    end
    class MergeAttrs < T::Struct
      include Baml::Sorbet::Struct
      const :amount, Baml::Checked[Integer]

      def initialize(props)
        super(
          amount: props[:amount],
        )

        @props = props
      end
    end
    class NamedArgsSingleClass < T::Struct
      include Baml::Sorbet::Struct
      const :key, String
      const :key_two, T::Boolean
      const :key_three, Integer

      def initialize(props)
        super(
          key: props[:key],
          key_two: props[:key_two],
          key_three: props[:key_three],
        )

        @props = props
      end
    end
    class Nested < T::Struct
      include Baml::Sorbet::Struct
      const :prop3, T.any(String, T.nilable(NilClass))
      const :prop4, T.any(String, T.nilable(NilClass))
      const :prop20, Baml::Types::Nested2

      def initialize(props)
        super(
          prop3: props[:prop3],
          prop4: props[:prop4],
          prop20: props[:prop20],
        )

        @props = props
      end
    end
    class Nested2 < T::Struct
      include Baml::Sorbet::Struct
      const :prop11, T.any(String, T.nilable(NilClass))
      const :prop12, T.any(String, T.nilable(NilClass))

      def initialize(props)
        super(
          prop11: props[:prop11],
          prop12: props[:prop12],
        )

        @props = props
      end
    end
    class NestedBlockConstraint < T::Struct
      include Baml::Sorbet::Struct
      const :nbc, Baml::Checked[Baml::Types::BlockConstraint]

      def initialize(props)
        super(
          nbc: props[:nbc],
        )

        @props = props
      end
    end
    class NestedBlockConstraintForParam < T::Struct
      include Baml::Sorbet::Struct
      const :nbcfp, Baml::Types::BlockConstraintForParam

      def initialize(props)
        super(
          nbcfp: props[:nbcfp],
        )

        @props = props
      end
    end
    class Node < T::Struct
      include Baml::Sorbet::Struct
      const :data, Integer
      const :next, T.nilable(Baml::Types::Node)

      def initialize(props)
        super(
          data: props[:data],
          next: props[:next],
        )

        @props = props
      end
    end
    class NodeWithAliasIndirection < T::Struct
      include Baml::Sorbet::Struct
      const :value, Integer
      const :next, T.nilable(Baml::Types::NodeWithAliasIndirection)

      def initialize(props)
        super(
          value: props[:value],
          next: props[:next],
        )

        @props = props
      end
    end
    class OptionalListAndMap < T::Struct
      include Baml::Sorbet::Struct
      const :p, T.nilable(T::Array[String])
      const :q, T.nilable(T::Hash[String, String])

      def initialize(props)
        super(
          p: props[:p],
          q: props[:q],
        )

        @props = props
      end
    end
    class OptionalTest_Prop1 < T::Struct
      include Baml::Sorbet::Struct
      const :omega_a, String
      const :omega_b, Integer

      def initialize(props)
        super(
          omega_a: props[:omega_a],
          omega_b: props[:omega_b],
        )

        @props = props
      end
    end
    class OptionalTest_ReturnType < T::Struct
      include Baml::Sorbet::Struct
      const :omega_1, T.nilable(Baml::Types::OptionalTest_Prop1)
      const :omega_2, T.nilable(String)
      const :omega_3, T::Array[T.nilable(T.any(Baml::Types::OptionalTest_CategoryType, String))]

      def initialize(props)
        super(
          omega_1: props[:omega_1],
          omega_2: props[:omega_2],
          omega_3: props[:omega_3],
        )

        @props = props
      end
    end
    class OrderInfo < T::Struct
      include Baml::Sorbet::Struct
      const :order_status, T.any(Baml::Types::OrderStatus, String)
      const :tracking_number, T.nilable(String)
      const :estimated_arrival_date, T.nilable(String)

      def initialize(props)
        super(
          order_status: props[:order_status],
          tracking_number: props[:tracking_number],
          estimated_arrival_date: props[:estimated_arrival_date],
        )

        @props = props
      end
    end
    class OriginalA < T::Struct
      include Baml::Sorbet::Struct
      const :value, Integer

      def initialize(props)
        super(
          value: props[:value],
        )

        @props = props
      end
    end
    class OriginalB < T::Struct
      include Baml::Sorbet::Struct
      const :value, Integer

      def initialize(props)
        super(
          value: props[:value],
        )

        @props = props
      end
    end
    class Person < T::Struct
      include Baml::Sorbet::Struct
      const :name, T.nilable(String)
      const :hair_color, T.nilable(T.any(Baml::Types::Color, String))

      def initialize(props)
        super(
          name: props[:name],
          hair_color: props[:hair_color],
        )

        @props = props
      end
    end
    class PhoneNumber < T::Struct
      include Baml::Sorbet::Struct
      const :value, String

      def initialize(props)
        super(
          value: props[:value],
        )

        @props = props
      end
    end
    class Quantity < T::Struct
      include Baml::Sorbet::Struct
      const :amount, T.any(Integer, Float)
      const :unit, T.nilable(String)

      def initialize(props)
        super(
          amount: props[:amount],
          unit: props[:unit],
        )

        @props = props
      end
    end
    class RaysData < T::Struct
      include Baml::Sorbet::Struct
      const :dataType, T.any(Baml::Types::DataType, String)
      const :value, T.any(Baml::Types::Resume, Baml::Types::Event)

      def initialize(props)
        super(
          dataType: props[:dataType],
          value: props[:value],
        )

        @props = props
      end
    end
    class ReceiptInfo < T::Struct
      include Baml::Sorbet::Struct
      const :items, T::Array[Baml::Types::ReceiptItem]
      const :total_cost, T.nilable(Float)
      const :venue, T.any(String, String)

      def initialize(props)
        super(
          items: props[:items],
          total_cost: props[:total_cost],
          venue: props[:venue],
        )

        @props = props
      end
    end
    class ReceiptItem < T::Struct
      include Baml::Sorbet::Struct
      const :name, String
      const :description, T.nilable(String)
      const :quantity, Integer
      const :price, Float

      def initialize(props)
        super(
          name: props[:name],
          description: props[:description],
          quantity: props[:quantity],
          price: props[:price],
        )

        @props = props
      end
    end
    class Recipe < T::Struct
      include Baml::Sorbet::Struct
      const :ingredients, T::Hash[String, Baml::Types::Quantity]
      const :recipe_type, T.any(String, String)

      def initialize(props)
        super(
          ingredients: props[:ingredients],
          recipe_type: props[:recipe_type],
        )

        @props = props
      end
    end
    class Resume < T::Struct
      include Baml::Sorbet::Struct
      const :name, String
      const :email, String
      const :phone, String
      const :experience, T::Array[Baml::Types::Education]
      const :education, T::Array[String]
      const :skills, T::Array[String]

      def initialize(props)
        super(
          name: props[:name],
          email: props[:email],
          phone: props[:phone],
          experience: props[:experience],
          education: props[:education],
          skills: props[:skills],
        )

        @props = props
      end
    end
    class Schema < T::Struct
      include Baml::Sorbet::Struct
      const :prop1, T.any(String, T.nilable(NilClass))
      const :prop2, T.any(Baml::Types::Nested, String)
      const :prop5, T::Array[T.any(String, T.nilable(NilClass))]
      const :prop6, T.any(String, T::Array[Baml::Types::Nested])
      const :nested_attrs, T::Array[T.any(String, T.nilable(NilClass), Baml::Types::Nested)]
      const :parens, T.any(String, T.nilable(NilClass))
      const :other_group, T.any(String, T.any(Integer, String))

      def initialize(props)
        super(
          prop1: props[:prop1],
          prop2: props[:prop2],
          prop5: props[:prop5],
          prop6: props[:prop6],
          nested_attrs: props[:nested_attrs],
          parens: props[:parens],
          other_group: props[:other_group],
        )

        @props = props
      end
    end
    class SearchParams < T::Struct
      include Baml::Sorbet::Struct
      const :dateRange, T.nilable(Integer)
      const :location, T::Array[String]
      const :jobTitle, T.nilable(Baml::Types::WithReasoning)
      const :company, T.nilable(Baml::Types::WithReasoning)
      const :description, T::Array[Baml::Types::WithReasoning]
      const :tags, T::Array[T.any(T.any(Baml::Types::Tag, String), String)]

      def initialize(props)
        super(
          dateRange: props[:dateRange],
          location: props[:location],
          jobTitle: props[:jobTitle],
          company: props[:company],
          description: props[:description],
          tags: props[:tags],
        )

        @props = props
      end
    end
    class SomeClassNestedDynamic < T::Struct
      include Baml::Sorbet::Struct
      const :hi, String

      def initialize(props)
        super(
          hi: props[:hi],
        )

        @props = props
      end
    end
    class StringToClassEntry < T::Struct
      include Baml::Sorbet::Struct
      const :word, String

      def initialize(props)
        super(
          word: props[:word],
        )

        @props = props
      end
    end
    class TestClassAlias < T::Struct
      include Baml::Sorbet::Struct
      const :key, String
      const :key2, String
      const :key3, String
      const :key4, String
      const :key5, String

      def initialize(props)
        super(
          key: props[:key],
          key2: props[:key2],
          key3: props[:key3],
          key4: props[:key4],
          key5: props[:key5],
        )

        @props = props
      end
    end
    class TestClassNested < T::Struct
      include Baml::Sorbet::Struct
      const :prop1, String
      const :prop2, Baml::Types::InnerClass

      def initialize(props)
        super(
          prop1: props[:prop1],
          prop2: props[:prop2],
        )

        @props = props
      end
    end
    class TestClassWithEnum < T::Struct
      include Baml::Sorbet::Struct
      const :prop1, String
      const :prop2, T.any(Baml::Types::EnumInClass, String)

      def initialize(props)
        super(
          prop1: props[:prop1],
          prop2: props[:prop2],
        )

        @props = props
      end
    end
    class TestOutputClass < T::Struct
      include Baml::Sorbet::Struct
      const :prop1, String
      const :prop2, Integer

      def initialize(props)
        super(
          prop1: props[:prop1],
          prop2: props[:prop2],
        )

        @props = props
      end
    end
    class Tree < T::Struct
      include Baml::Sorbet::Struct
      const :data, Integer
      const :children, Baml::Types::Forest

      def initialize(props)
        super(
          data: props[:data],
          children: props[:children],
        )

        @props = props
      end
    end
    class TwoStoriesOneTitle < T::Struct
      include Baml::Sorbet::Struct
      const :title, String
      const :story_a, String
      const :story_b, String

      def initialize(props)
        super(
          title: props[:title],
          story_a: props[:story_a],
          story_b: props[:story_b],
        )

        @props = props
      end
    end
    class UnionTest_ReturnType < T::Struct
      include Baml::Sorbet::Struct
      const :prop1, T.any(String, T::Boolean)
      const :prop2, T::Array[T.any(Float, T::Boolean)]
      const :prop3, T.any(T::Array[T::Boolean], T::Array[Integer])

      def initialize(props)
        super(
          prop1: props[:prop1],
          prop2: props[:prop2],
          prop3: props[:prop3],
        )

        @props = props
      end
    end
    class WithReasoning < T::Struct
      include Baml::Sorbet::Struct
      const :value, String
      const :reasoning, String

      def initialize(props)
        super(
          value: props[:value],
          reasoning: props[:reasoning],
        )

        @props = props
      end
    end

  end
end