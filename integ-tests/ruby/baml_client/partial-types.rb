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

require_relative "types"

module Baml
  
  module PartialTypes
    class Blah < T::Struct; end
    class ClassOptionalOutput < T::Struct; end
    class ClassOptionalOutput2 < T::Struct; end
    class ClassWithImage < T::Struct; end
    class DummyOutput < T::Struct; end
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
    class Quantity < T::Struct; end
    class RaysData < T::Struct; end
    class ReceiptInfo < T::Struct; end
    class ReceiptItem < T::Struct; end
    class Recipe < T::Struct; end
    class Resume < T::Struct; end
    class SearchParams < T::Struct; end
    class SomeClassNestedDynamic < T::Struct; end
    class StringToClassEntry < T::Struct; end
    class TestClassAlias < T::Struct; end
    class TestClassNested < T::Struct; end
    class TestClassWithEnum < T::Struct; end
    class TestOutputClass < T::Struct; end
    class UnionTest_ReturnType < T::Struct; end
    class WithReasoning < T::Struct; end
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
    class ClassOptionalOutput < T::Struct
      include Baml::Sorbet::Struct
      const :prop1, T.nilable(String)
      const :prop2, T.nilable(String)

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
      const :prop3, Baml::PartialTypes::Blah

      def initialize(props)
        super(
          prop1: props[:prop1],
          prop2: props[:prop2],
          prop3: props[:prop3],
        )

        @props = props
      end
    end
    class ClassWithImage < T::Struct
      include Baml::Sorbet::Struct
      const :myImage, T.nilable(Baml::Image)
      const :param2, T.nilable(String)
      const :fake_image, Baml::PartialTypes::FakeImage

      def initialize(props)
        super(
          myImage: props[:myImage],
          param2: props[:param2],
          fake_image: props[:fake_image],
        )

        @props = props
      end
    end
    class DummyOutput < T::Struct
      include Baml::Sorbet::Struct
      const :nonce, T.nilable(String)
      const :nonce2, T.nilable(String)

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
      const :testKey, T.nilable(String)

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
      const :hi, T.nilable(String)
      const :some_class, Baml::PartialTypes::SomeClassNestedDynamic
      const :status, T.nilable(Baml::Types::DynEnumOne)

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
    class Education < T::Struct
      include Baml::Sorbet::Struct
      const :institution, T.nilable(String)
      const :location, T.nilable(String)
      const :degree, T.nilable(String)
      const :major, T::Array[T.nilable(String)]
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
      const :subject, T.nilable(String)
      const :body, T.nilable(String)
      const :from_address, T.nilable(String)

      def initialize(props)
        super(
          subject: props[:subject],
          body: props[:body],
          from_address: props[:from_address],
        )

        @props = props
      end
    end
    class Event < T::Struct
      include Baml::Sorbet::Struct
      const :title, T.nilable(String)
      const :date, T.nilable(String)
      const :location, T.nilable(String)
      const :description, T.nilable(String)

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
      const :url, T.nilable(String)

      def initialize(props)
        super(
          url: props[:url],
        )

        @props = props
      end
    end
    class InnerClass < T::Struct
      include Baml::Sorbet::Struct
      const :prop1, T.nilable(String)
      const :prop2, T.nilable(String)
      const :inner, Baml::PartialTypes::InnerClass2

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
      const :prop2, T.nilable(Integer)
      const :prop3, T.nilable(Float)

      def initialize(props)
        super(
          prop2: props[:prop2],
          prop3: props[:prop3],
        )

        @props = props
      end
    end
    class NamedArgsSingleClass < T::Struct
      include Baml::Sorbet::Struct
      const :key, T.nilable(String)
      const :key_two, T.nilable(T::Boolean)
      const :key_three, T.nilable(Integer)

      def initialize(props)
        super(
          key: props[:key],
          key_two: props[:key_two],
          key_three: props[:key_three],
        )

        @props = props
      end
    end
    class OptionalTest_Prop1 < T::Struct
      include Baml::Sorbet::Struct
      const :omega_a, T.nilable(String)
      const :omega_b, T.nilable(Integer)

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
      const :omega_1, Baml::PartialTypes::OptionalTest_Prop1
      const :omega_2, T.nilable(String)
      const :omega_3, T::Array[T.nilable(Baml::Types::OptionalTest_CategoryType)]

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
      const :order_status, T.nilable(Baml::Types::OrderStatus)
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
    class Person < T::Struct
      include Baml::Sorbet::Struct
      const :name, T.nilable(String)
      const :hair_color, T.nilable(Baml::Types::Color)

      def initialize(props)
        super(
          name: props[:name],
          hair_color: props[:hair_color],
        )

        @props = props
      end
    end
    class Quantity < T::Struct
      include Baml::Sorbet::Struct
      const :amount, T.nilable(T.any(T.nilable(T.any(T.nilable(Integer))), T.nilable(T.any(T.nilable(Float)))))
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
      const :dataType, T.nilable(Baml::Types::DataType)
      const :value, T.nilable(T.any(T.nilable(T.any(Baml::PartialTypes::Resume)), T.nilable(T.any(Baml::PartialTypes::Event))))

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
      const :items, T::Array[Baml::PartialTypes::ReceiptItem]
      const :total_cost, T.nilable(Float)

      def initialize(props)
        super(
          items: props[:items],
          total_cost: props[:total_cost],
        )

        @props = props
      end
    end
    class ReceiptItem < T::Struct
      include Baml::Sorbet::Struct
      const :name, T.nilable(String)
      const :description, T.nilable(String)
      const :quantity, T.nilable(Integer)
      const :price, T.nilable(Float)

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
      const :ingredients, T::Hash[String, Baml::PartialTypes::Quantity]

      def initialize(props)
        super(
          ingredients: props[:ingredients],
        )

        @props = props
      end
    end
    class Resume < T::Struct
      include Baml::Sorbet::Struct
      const :name, T.nilable(String)
      const :email, T.nilable(String)
      const :phone, T.nilable(String)
      const :experience, T::Array[Baml::PartialTypes::Education]
      const :education, T::Array[T.nilable(String)]
      const :skills, T::Array[T.nilable(String)]

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
    class SearchParams < T::Struct
      include Baml::Sorbet::Struct
      const :dateRange, T.nilable(Integer)
      const :location, T::Array[T.nilable(String)]
      const :jobTitle, Baml::PartialTypes::WithReasoning
      const :company, Baml::PartialTypes::WithReasoning
      const :description, T::Array[Baml::PartialTypes::WithReasoning]
      const :tags, T::Array[T.nilable(T.any(T.nilable(T.any(T.nilable(Baml::Types::Tag))), T.nilable(T.any(T.nilable(String)))))]

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
      const :hi, T.nilable(String)

      def initialize(props)
        super(
          hi: props[:hi],
        )

        @props = props
      end
    end
    class StringToClassEntry < T::Struct
      include Baml::Sorbet::Struct
      const :word, T.nilable(String)

      def initialize(props)
        super(
          word: props[:word],
        )

        @props = props
      end
    end
    class TestClassAlias < T::Struct
      include Baml::Sorbet::Struct
      const :key, T.nilable(String)
      const :key2, T.nilable(String)
      const :key3, T.nilable(String)
      const :key4, T.nilable(String)
      const :key5, T.nilable(String)

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
      const :prop1, T.nilable(String)
      const :prop2, Baml::PartialTypes::InnerClass

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
      const :prop1, T.nilable(String)
      const :prop2, T.nilable(Baml::Types::EnumInClass)

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
      const :prop1, T.nilable(String)
      const :prop2, T.nilable(Integer)

      def initialize(props)
        super(
          prop1: props[:prop1],
          prop2: props[:prop2],
        )

        @props = props
      end
    end
    class UnionTest_ReturnType < T::Struct
      include Baml::Sorbet::Struct
      const :prop1, T.nilable(T.any(T.nilable(T.any(T.nilable(String))), T.nilable(T.any(T.nilable(T::Boolean)))))
      const :prop2, T::Array[T.nilable(T.any(T.nilable(T.any(T.nilable(Float))), T.nilable(T.any(T.nilable(T::Boolean)))))]
      const :prop3, T.nilable(T.any(T.nilable(T.any(T::Array[T.nilable(T::Boolean)])), T.nilable(T.any(T::Array[T.nilable(Integer)]))))

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
      const :value, T.nilable(String)
      const :reasoning, T.nilable(String)

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