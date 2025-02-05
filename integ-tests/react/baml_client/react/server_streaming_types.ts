/*************************************************************************************************

Welcome to Baml! To use this generated code, please run one of the following:

$ npm install @boundaryml/baml
$ yarn add @boundaryml/baml
$ pnpm add @boundaryml/baml

*************************************************************************************************/

// This file was generated by BAML: do not edit it. Instead, edit the BAML
// files and re-generate this code.
//
/* eslint-disable */
// tslint:disable
// @ts-nocheck
// biome-ignore format: autogenerated code
import type { Check, Checked  } from "../types";
import type { Image, Audio } from "@boundaryml/baml";

import type {  AnotherObject,  BigNumbers,  BinaryNode,  Blah,  BlockConstraint,  BlockConstraintForParam,  BookOrder,  ClassForNullLiteral,  ClassOptionalOutput,  ClassOptionalOutput2,  ClassToRecAlias,  ClassWithBlockDone,  ClassWithImage,  ClassWithoutDone,  ComplexMemoryObject,  CompoundBigNumbers,  ContactInfo,  CustomTaskResult,  DummyOutput,  DynInputOutput,  DynamicClassOne,  DynamicClassTwo,  DynamicOutput,  Earthling,  Education,  Email,  EmailAddress,  Event,  FakeImage,  FlightConfirmation,  FooAny,  Forest,  FormatterTest0,  FormatterTest1,  FormatterTest2,  FormatterTest3,  GroceryReceipt,  InnerClass,  InnerClass2,  InputClass,  InputClassNested,  LinkedList,  LinkedListAliasNode,  LiteralClassHello,  LiteralClassOne,  LiteralClassTwo,  MalformedConstraints,  MalformedConstraints2,  Martian,  MemoryObject,  MergeAttrs,  NamedArgsSingleClass,  Nested,  Nested2,  NestedBlockConstraint,  NestedBlockConstraintForParam,  Node,  NodeWithAliasIndirection,  OptionalListAndMap,  OptionalTest_Prop1,  OptionalTest_ReturnType,  OrderInfo,  OriginalA,  OriginalB,  Person,  PhoneNumber,  Quantity,  RaysData,  ReceiptInfo,  ReceiptItem,  Recipe,  RecursiveAliasDependency,  Resume,  Schema,  SearchParams,  SemanticContainer,  SimpleTag,  SmallThing,  SomeClassNestedDynamic,  StringToClassEntry,  TestClassAlias,  TestClassNested,  TestClassWithEnum,  TestMemoryOutput,  TestOutputClass,  Tree,  TwoStoriesOneTitle,  UnionTest_ReturnType,  UniverseQuestion,  UniverseQuestionInput,  WithReasoning,  AliasedEnum,  Category,  Category2,  Category3,  Color,  DataType,  DynEnumOne,  DynEnumTwo,  EnumInClass,  EnumOutput,  Hobby,  MapKey,  NamedArgsSingleEnum,  NamedArgsSingleEnumList,  OptionalTest_CategoryType,  OrderStatus,  Tag,  TestEnum,  JsonEntry,  JsonTemplate,  RecursiveMapAlias,  RecursiveListAlias,  RecAliasOne,  RecAliasTwo,  RecAliasThree,  JsonValue,  JsonObject,  JsonArray } from "../types"

import type * as types from "../types"
import type { partial_types }from "../partial_types";

export type StreamingServerTypes = {
  ClassThatPointsToRecursiveClassThroughAlias: partial_types.ClassToRecAlias,
  DescribeImage4: string,
  TakeRecAliasDep: partial_types.RecursiveAliasDependency,
  TestMulticlassNamedArgs: string,
  FnOutputStringList: (string | null)[],
  TestOpenAILegacyProvider: string,
  TestFnNamedArgsSingleMapStringToClass: (Record<string, (partial_types.StringToClassEntry | null)> ),
  StreamBigNumbers: partial_types.BigNumbers,
  StreamOneBigNumber: number,
  MakeBlockConstraint: Checked<partial_types.BlockConstraint,"cross_field">,
  StreamingCompoundNumbers: partial_types.CompoundBigNumbers,
  BuildTree: partial_types.Tree,
  TestAzure: string,
  FnOutputBool: boolean,
  LiteralUnionsTest: (1 | true | "string output"),
  TestFnNamedArgsSingleMapStringToMap: (Record<string, (Record<string, (string | null)> | null)> ),
  TestAzureO3NoMaxTokens: string,
  TestFnNamedArgsSingleFloat: string,
  TestGemini: string,
  TestNamedArgsLiteralInt: string,
  TestImageInput: string,
  TestSingleFallbackClient: string,
  FnEnumListOutput: (EnumOutput | null)[],
  UnionTest_Function: partial_types.UnionTest_ReturnType,
  FnOutputClassWithEnum: partial_types.TestClassWithEnum,
  MakeNestedBlockConstraint: partial_types.NestedBlockConstraint,
  ExtractNames: (string | null)[],
  TestGeminiSystemAsChat: string,
  TestImageListInput: string,
  StreamUnionIntegers: ((number | null) | (string | null) | null)[],
  AliasThatPointsToRecursiveType: partial_types.LinkedListAliasNode,
  TestFallbackToShorthand: string,
  AllowedOptionals: partial_types.OptionalListAndMap,
  BuildLinkedList: partial_types.LinkedList,
  DynamicInputOutput: partial_types.DynInputOutput,
  ExtractPeople: (partial_types.Person | null)[],
  InOutLiteralStringUnionMapKey: (Record<"one" | "two" | "three" | "four", (string | null)> ),
  InOutSingleLiteralStringMapKey: (Record<"key", (string | null)> ),
  PredictAgeBare: Checked<number,"too_big">,
  TestAzureO3WithMaxCompletionTokens: string,
  TestOpenAIO1NoMaxTokens: string,
  AaaSamOutputFormat: partial_types.Recipe,
  TestGeminiSystem: string,
  TestUniverseQuestion: partial_types.UniverseQuestion,
  Completion: string,
  TestAwsInvalidRegion: string,
  DummyOutputFunction: partial_types.DummyOutput,
  InOutEnumMapKey: (Record<MapKey, (string | null)> ),
  TestAzureWithMaxTokens: string,
  TestNamedArgsLiteralBool: string,
  FnOutputClassNested: partial_types.TestClassNested,
  ExtractContactInfo: partial_types.ContactInfo,
  NestedAlias: (((number | null) | (string | null) | (boolean | null) | (number | null) | null) | (string | null)[] | (Record<string, (string | null)[]> | null)),
  PredictAge: partial_types.FooAny,
  FnNamedArgsSingleStringOptional: string,
  GetQuery: partial_types.SearchParams,
  UseBlockConstraint: number,
  FnClassOptionalOutput2: ((partial_types.ClassOptionalOutput2 | null) | null),
  RecursiveClassWithAliasIndirection: partial_types.NodeWithAliasIndirection,
  ClassifyMessage2: types.Category,
  FnEnumOutput: types.EnumOutput,
  AliasedInputList: string,
  PromptTestOpenAIChatNoSystem: string,
  TestAnthropicShorthand: string,
  TestAzureFailure: string,
  TestCaching: string,
  TestFnNamedArgsSingleStringArray: string,
  AliasWithMultipleAttrs: Checked<number,"gt_ten">,
  ExpectFailure: string,
  FnLiteralClassInputOutput: partial_types.LiteralClassHello,
  DynamicFunc: partial_types.DynamicClassTwo,
  TestOpenAIWithNullMaxTokens: string,
  PromptTestOpenAIChat: string,
  FnOutputInt: number,
  AliasedInputEnum: string,
  TestFnNamedArgsSingleString: string,
  ReturnMalformedConstraints: partial_types.MalformedConstraints,
  ClassifyDynEnumTwo: (string | DynEnumTwo),
  AliasedInputClass2: string,
  FnLiteralUnionClassInputOutput: ((partial_types.LiteralClassOne | null) | (partial_types.LiteralClassTwo | null)),
  SchemaDescriptions: partial_types.Schema,
  RecursiveAliasCycle: RecAliasOne,
  TestAwsInvalidProfile: string,
  SimpleRecursiveMapAlias: RecursiveMapAlias,
  StreamFailingAssertion: partial_types.TwoStoriesOneTitle,
  TestAzureO1WithMaxTokens: string,
  TestFnNamedArgsSingleEnumList: string,
  TestOpenAIShorthand: string,
  TestRetryConstant: string,
  TestVertex: string,
  UseNestedBlockConstraint: number,
  PromptTestClaudeChatNoSystem: string,
  FnTestAliasedEnumOutput: types.TestEnum,
  TestAzureO1WithMaxCompletionTokens: string,
  ExtractHobby: (string | Hobby | null)[],
  AssertFn: number,
  TestOpenAIWithMaxTokens: string,
  FnOutputClassList: (partial_types.TestOutputClass | null)[],
  MyFunc: partial_types.DynamicOutput,
  TestImageInputAnthropic: string,
  TestFnNamedArgsSingleStringList: string,
  ExtractResume: partial_types.Resume,
  ReturnJsonEntry: JsonTemplate,
  MapAlias: (Record<string, (string | null)[]> ),
  NullLiteralClassHello: partial_types.ClassForNullLiteral,
  TestVertexWithSystemInstructions: string,
  TestAwsInvalidSessionToken: string,
  DynamicListInputOutput: (partial_types.DynInputOutput | null)[],
  PromptTestStreaming: string,
  OptionalTest_Function: ((partial_types.OptionalTest_ReturnType | null) | null)[],
  ExtractResume2: partial_types.Resume,
  ExtractReceiptInfo: partial_types.ReceiptInfo,
  TestRetryExponential: string,
  PromptTestClaudeChat: string,
  TestAwsInvalidAccessKey: string,
  ClassifyMessage3: types.Category,
  FnOutputClass: partial_types.TestOutputClass,
  TestFallbackClient: string,
  TestAnthropic: string,
  PromptTestOpenAI: string,
  FnOutputLiteralInt: 5,
  DescribeImage3: string,
  ReturnAliasWithMergedAttributes: Checked<number,"gt_ten">,
  JsonTypeAliasCycle: JsonValue,
  AliasedInputClassNested: string,
  FnOutputLiteralString: "example output",
  DescribeImage: string,
  TestOllama: string,
  CustomTask: ((partial_types.BookOrder | null) | (partial_types.FlightConfirmation | null) | (partial_types.GroceryReceipt | null)),
  MakeSemanticContainer: partial_types.SemanticContainer,
  AudioInput: string,
  PromptTestClaude: string,
  ReturnFailingAssert: number,
  TestFnNamedArgsSingleBool: string,
  GetDataType: partial_types.RaysData,
  TestFnNamedArgsSingleClass: string,
  AliasedInputClass: string,
  DifferentiateUnions: ((partial_types.OriginalA | null) | (partial_types.OriginalB | null)),
  UseMalformedConstraints: number,
  TestAws: string,
  TestFnNamedArgsSingleMapStringToString: (Record<string, (string | null)> ),
  TestNamedArgsLiteralString: string,
  FnTestClassAlias: partial_types.TestClassAlias,
  MergeAliasAttributes: partial_types.MergeAttrs,
  GetOrderInfo: partial_types.OrderInfo,
  PrimitiveAlias: ((number | null) | (string | null) | (boolean | null) | (number | null)),
  SimpleRecursiveListAlias: RecursiveListAlias,
  TestFnNamedArgsSingleInt: string,
  TestOpenAIO1WithMaxCompletionTokens: string,
  TestAzureO1NoMaxTokens: string,
  FnClassOptionalOutput: ((partial_types.ClassOptionalOutput | null) | null),
  TestMemory: partial_types.TestMemoryOutput,
  TestGeminiOpenAiGeneric: string,
  DescribeImage2: string,
  FnOutputLiteralBool: false,
  FnTestNamedArgsSingleEnum: string,
  TestOpenAI: string,
  TestOpenAIO1WithMaxTokens: string,
  ClassifyMessage: types.Category,
}