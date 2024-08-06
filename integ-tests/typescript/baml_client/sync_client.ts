/*************************************************************************************************

Welcome to Baml! To use this generated code, please run one of the following:

$ npm install @boundaryml/baml
$ yarn add @boundaryml/baml
$ pnpm add @boundaryml/baml

*************************************************************************************************/

// This file was generated by BAML: do not edit it. Instead, edit the BAML
// files and re-generate this code.
//
// tslint:disable
// @ts-nocheck
// biome-ignore format: autogenerated code
/* eslint-disable */
import { BamlRuntime, FunctionResult, BamlCtxManager, BamlSyncStream, Image, ClientBuilder } from "@boundaryml/baml"
import {Blah, ClassOptionalOutput, ClassOptionalOutput2, ClassWithImage, DummyOutput, DynInputOutput, DynamicClassOne, DynamicClassTwo, DynamicOutput, Education, Email, Event, FakeImage, InnerClass, InnerClass2, NamedArgsSingleClass, OptionalTest_Prop1, OptionalTest_ReturnType, OrderInfo, Person, Quantity, RaysData, ReceiptInfo, ReceiptItem, Recipe, Resume, SearchParams, SomeClassNestedDynamic, StringToClassEntry, TestClassAlias, TestClassNested, TestClassWithEnum, TestOutputClass, UnionTest_ReturnType, WithReasoning, Category, Category2, Category3, Color, DataType, DynEnumOne, DynEnumTwo, EnumInClass, EnumOutput, Hobby, NamedArgsSingleEnum, NamedArgsSingleEnumList, OptionalTest_CategoryType, OrderStatus, Tag, TestEnum} from "./types"
import TypeBuilder from "./type_builder"
import { DO_NOT_USE_DIRECTLY_UNLESS_YOU_KNOW_WHAT_YOURE_DOING_CTX, DO_NOT_USE_DIRECTLY_UNLESS_YOU_KNOW_WHAT_YOURE_DOING_RUNTIME } from "./globals"

export type RecursivePartialNull<T> = T extends object
  ? {
      [P in keyof T]?: RecursivePartialNull<T[P]>;
    }
  : T | null;

export class BamlSyncClient {
  private runtime: BamlRuntime
  private ctx_manager: BamlCtxManager

  constructor(private runtime: BamlRuntime, private ctx_manager: BamlCtxManager) {}

  /*
  * @deprecated NOT IMPLEMENTED as streaming must by async. We
  * are not providing an async version as we want to reserve the
  * right to provide a sync version in the future.
  */
  get stream() {
    throw new Error("stream is not available in BamlSyncClient. Use `import { b } from 'baml_client/async_client")
  }  

  
  AaaSamOutputFormat(
      recipe: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): Recipe {
    const raw = this.runtime.callFunctionSync(
      "AaaSamOutputFormat",
      {
        "recipe": recipe
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as Recipe
  }
  
  AudioInput(
      aud: Audio,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "AudioInput",
      {
        "aud": aud
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  ClassifyMessage(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): Category {
    const raw = this.runtime.callFunctionSync(
      "ClassifyMessage",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as Category
  }
  
  ClassifyMessage2(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): Category {
    const raw = this.runtime.callFunctionSync(
      "ClassifyMessage2",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as Category
  }
  
  ClassifyMessage3(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): Category {
    const raw = this.runtime.callFunctionSync(
      "ClassifyMessage3",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as Category
  }
  
  DescribeImage(
      img: Image,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "DescribeImage",
      {
        "img": img
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  DescribeImage2(
      classWithImage: ClassWithImage,img2: Image,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "DescribeImage2",
      {
        "classWithImage": classWithImage,"img2": img2
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  DescribeImage3(
      classWithImage: ClassWithImage,img2: Image,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "DescribeImage3",
      {
        "classWithImage": classWithImage,"img2": img2
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  DescribeImage4(
      classWithImage: ClassWithImage,img2: Image,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "DescribeImage4",
      {
        "classWithImage": classWithImage,"img2": img2
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  DummyOutputFunction(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): DummyOutput {
    const raw = this.runtime.callFunctionSync(
      "DummyOutputFunction",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as DummyOutput
  }
  
  DynamicFunc(
      input: DynamicClassOne,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): DynamicClassTwo {
    const raw = this.runtime.callFunctionSync(
      "DynamicFunc",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as DynamicClassTwo
  }
  
  DynamicInputOutput(
      input: DynInputOutput,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): DynInputOutput {
    const raw = this.runtime.callFunctionSync(
      "DynamicInputOutput",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as DynInputOutput
  }
  
  DynamicListInputOutput(
      input: DynInputOutput[],
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): DynInputOutput[] {
    const raw = this.runtime.callFunctionSync(
      "DynamicListInputOutput",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as DynInputOutput[]
  }
  
  ExpectFailure(
      
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "ExpectFailure",
      {
        
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  ExtractNames(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string[] {
    const raw = this.runtime.callFunctionSync(
      "ExtractNames",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string[]
  }
  
  ExtractPeople(
      text: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): Person[] {
    const raw = this.runtime.callFunctionSync(
      "ExtractPeople",
      {
        "text": text
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as Person[]
  }
  
  ExtractReceiptInfo(
      email: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): ReceiptInfo {
    const raw = this.runtime.callFunctionSync(
      "ExtractReceiptInfo",
      {
        "email": email
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as ReceiptInfo
  }
  
  ExtractResume(
      resume: string,img: Image,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): Resume {
    const raw = this.runtime.callFunctionSync(
      "ExtractResume",
      {
        "resume": resume,"img": img
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as Resume
  }
  
  ExtractResume2(
      resume: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): Resume {
    const raw = this.runtime.callFunctionSync(
      "ExtractResume2",
      {
        "resume": resume
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as Resume
  }
  
  FnClassOptionalOutput(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): ClassOptionalOutput | null {
    const raw = this.runtime.callFunctionSync(
      "FnClassOptionalOutput",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as ClassOptionalOutput | null
  }
  
  FnClassOptionalOutput2(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): ClassOptionalOutput2 | null {
    const raw = this.runtime.callFunctionSync(
      "FnClassOptionalOutput2",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as ClassOptionalOutput2 | null
  }
  
  FnEnumListOutput(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): EnumOutput[] {
    const raw = this.runtime.callFunctionSync(
      "FnEnumListOutput",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as EnumOutput[]
  }
  
  FnEnumOutput(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): EnumOutput {
    const raw = this.runtime.callFunctionSync(
      "FnEnumOutput",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as EnumOutput
  }
  
  FnNamedArgsSingleStringOptional(
      myString: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "FnNamedArgsSingleStringOptional",
      {
        "myString": myString
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  FnOutputBool(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): boolean {
    const raw = this.runtime.callFunctionSync(
      "FnOutputBool",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as boolean
  }
  
  FnOutputClass(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): TestOutputClass {
    const raw = this.runtime.callFunctionSync(
      "FnOutputClass",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as TestOutputClass
  }
  
  FnOutputClassList(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): TestOutputClass[] {
    const raw = this.runtime.callFunctionSync(
      "FnOutputClassList",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as TestOutputClass[]
  }
  
  FnOutputClassNested(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): TestClassNested {
    const raw = this.runtime.callFunctionSync(
      "FnOutputClassNested",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as TestClassNested
  }
  
  FnOutputClassWithEnum(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): TestClassWithEnum {
    const raw = this.runtime.callFunctionSync(
      "FnOutputClassWithEnum",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as TestClassWithEnum
  }
  
  FnOutputStringList(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string[] {
    const raw = this.runtime.callFunctionSync(
      "FnOutputStringList",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string[]
  }
  
  FnTestAliasedEnumOutput(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): TestEnum {
    const raw = this.runtime.callFunctionSync(
      "FnTestAliasedEnumOutput",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as TestEnum
  }
  
  FnTestClassAlias(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): TestClassAlias {
    const raw = this.runtime.callFunctionSync(
      "FnTestClassAlias",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as TestClassAlias
  }
  
  FnTestNamedArgsSingleEnum(
      myArg: NamedArgsSingleEnum,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "FnTestNamedArgsSingleEnum",
      {
        "myArg": myArg
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  GetDataType(
      text: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): RaysData {
    const raw = this.runtime.callFunctionSync(
      "GetDataType",
      {
        "text": text
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as RaysData
  }
  
  GetOrderInfo(
      email: Email,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): OrderInfo {
    const raw = this.runtime.callFunctionSync(
      "GetOrderInfo",
      {
        "email": email
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as OrderInfo
  }
  
  GetQuery(
      query: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): SearchParams {
    const raw = this.runtime.callFunctionSync(
      "GetQuery",
      {
        "query": query
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as SearchParams
  }
  
  MyFunc(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): DynamicOutput {
    const raw = this.runtime.callFunctionSync(
      "MyFunc",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as DynamicOutput
  }
  
  OptionalTest_Function(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): (OptionalTest_ReturnType | null)[] {
    const raw = this.runtime.callFunctionSync(
      "OptionalTest_Function",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as (OptionalTest_ReturnType | null)[]
  }
  
  PromptTestClaude(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "PromptTestClaude",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  PromptTestClaudeChat(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "PromptTestClaudeChat",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  PromptTestClaudeChatNoSystem(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "PromptTestClaudeChatNoSystem",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  PromptTestOpenAI(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "PromptTestOpenAI",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  PromptTestOpenAIChat(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "PromptTestOpenAIChat",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  PromptTestOpenAIChatNoSystem(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "PromptTestOpenAIChatNoSystem",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  PromptTestStreaming(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "PromptTestStreaming",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  TestAnthropic(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "TestAnthropic",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  TestAws(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "TestAws",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  TestAzure(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "TestAzure",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  TestFallbackClient(
      
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "TestFallbackClient",
      {
        
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  TestFnNamedArgsSingleBool(
      myBool: boolean,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "TestFnNamedArgsSingleBool",
      {
        "myBool": myBool
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  TestFnNamedArgsSingleClass(
      myArg: NamedArgsSingleClass,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "TestFnNamedArgsSingleClass",
      {
        "myArg": myArg
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  TestFnNamedArgsSingleEnumList(
      myArg: NamedArgsSingleEnumList[],
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "TestFnNamedArgsSingleEnumList",
      {
        "myArg": myArg
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  TestFnNamedArgsSingleFloat(
      myFloat: number,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "TestFnNamedArgsSingleFloat",
      {
        "myFloat": myFloat
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  TestFnNamedArgsSingleInt(
      myInt: number,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "TestFnNamedArgsSingleInt",
      {
        "myInt": myInt
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  TestFnNamedArgsSingleMapStringToClass(
      myMap: Record<string, StringToClassEntry>,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): Record<string, StringToClassEntry> {
    const raw = this.runtime.callFunctionSync(
      "TestFnNamedArgsSingleMapStringToClass",
      {
        "myMap": myMap
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as Record<string, StringToClassEntry>
  }
  
  TestFnNamedArgsSingleMapStringToMap(
      myMap: Record<string, Record<string, string>>,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): Record<string, Record<string, string>> {
    const raw = this.runtime.callFunctionSync(
      "TestFnNamedArgsSingleMapStringToMap",
      {
        "myMap": myMap
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as Record<string, Record<string, string>>
  }
  
  TestFnNamedArgsSingleMapStringToString(
      myMap: Record<string, string>,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): Record<string, string> {
    const raw = this.runtime.callFunctionSync(
      "TestFnNamedArgsSingleMapStringToString",
      {
        "myMap": myMap
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as Record<string, string>
  }
  
  TestFnNamedArgsSingleString(
      myString: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "TestFnNamedArgsSingleString",
      {
        "myString": myString
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  TestFnNamedArgsSingleStringArray(
      myStringArray: string[],
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "TestFnNamedArgsSingleStringArray",
      {
        "myStringArray": myStringArray
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  TestFnNamedArgsSingleStringList(
      myArg: string[],
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "TestFnNamedArgsSingleStringList",
      {
        "myArg": myArg
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  TestGemini(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "TestGemini",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  TestImageInput(
      img: Image,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "TestImageInput",
      {
        "img": img
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  TestImageListInput(
      imgs: Image[],
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "TestImageListInput",
      {
        "imgs": imgs
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  TestMulticlassNamedArgs(
      myArg: NamedArgsSingleClass,myArg2: NamedArgsSingleClass,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "TestMulticlassNamedArgs",
      {
        "myArg": myArg,"myArg2": myArg2
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  TestOllama(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "TestOllama",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  TestOpenAILegacyProvider(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "TestOpenAILegacyProvider",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  TestRetryConstant(
      
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "TestRetryConstant",
      {
        
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  TestRetryExponential(
      
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "TestRetryExponential",
      {
        
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  TestVertex(
      input: string,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): string {
    const raw = this.runtime.callFunctionSync(
      "TestVertex",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as string
  }
  
  UnionTest_Function(
      input: string | boolean,
      __baml_options__?: { tb?: TypeBuilder, clientRegistry?: ClientRegistry }
  ): UnionTest_ReturnType {
    const raw = this.runtime.callFunctionSync(
      "UnionTest_Function",
      {
        "input": input
      },
      this.ctx_manager.cloneContext(),
      __baml_options__?.tb?.__tb(),
      __baml_options__?.cr,
    )
    return raw.parsed() as UnionTest_ReturnType
  }
  
}

export const b = new BamlSyncClient(DO_NOT_USE_DIRECTLY_UNLESS_YOU_KNOW_WHAT_YOURE_DOING_RUNTIME, DO_NOT_USE_DIRECTLY_UNLESS_YOU_KNOW_WHAT_YOURE_DOING_CTX)