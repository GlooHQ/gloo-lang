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
import { Image } from "@boundaryml/baml"

export interface Checked<T,CheckName extends string = string> {
    value: T,
    checks: Record<CheckName, Check>,
}

export interface Check {
    name: string,
    expr: string
    status: "succeeded" | "failed"
}

export function all_succeeded<CheckName extends string>(checks: Record<CheckName, Check>): boolean {
    return get_checks(checks).every(check => check.status === "succeeded")
}

export function get_checks<CheckName extends string>(checks: Record<CheckName, Check>): Check[] {
    return Object.values(checks)
}
export enum AliasedEnum {
  KEY_ONE = "KEY_ONE",
  KEY_TWO = "KEY_TWO",
}

export enum Category {
  Refund = "Refund",
  CancelOrder = "CancelOrder",
  TechnicalSupport = "TechnicalSupport",
  AccountIssue = "AccountIssue",
  Question = "Question",
}

export enum Category2 {
  Refund = "Refund",
  CancelOrder = "CancelOrder",
  TechnicalSupport = "TechnicalSupport",
  AccountIssue = "AccountIssue",
  Question = "Question",
}

export enum Category3 {
  Refund = "Refund",
  CancelOrder = "CancelOrder",
  TechnicalSupport = "TechnicalSupport",
  AccountIssue = "AccountIssue",
  Question = "Question",
}

export enum Color {
  RED = "RED",
  BLUE = "BLUE",
  GREEN = "GREEN",
  YELLOW = "YELLOW",
  BLACK = "BLACK",
  WHITE = "WHITE",
}

export enum DataType {
  Resume = "Resume",
  Event = "Event",
}

export enum DynEnumOne {
}

export enum DynEnumTwo {
}

export enum EnumInClass {
  ONE = "ONE",
  TWO = "TWO",
}

/**
 * An enum with three values,
 * ONE, TWO and THREE.
 */
export enum EnumOutput {
  /**
   * The first enum.
   */
  ONE = "ONE",
  /**
   * The second enum.
   */
  TWO = "TWO",
  THREE = "THREE",
}

export enum Hobby {
  SPORTS = "SPORTS",
  MUSIC = "MUSIC",
  READING = "READING",
}

export enum NamedArgsSingleEnum {
  ONE = "ONE",
  TWO = "TWO",
}

export enum NamedArgsSingleEnumList {
  ONE = "ONE",
  TWO = "TWO",
}

export enum OptionalTest_CategoryType {
  Aleph = "Aleph",
  Beta = "Beta",
  Gamma = "Gamma",
}

export enum OrderStatus {
  ORDERED = "ORDERED",
  SHIPPED = "SHIPPED",
  DELIVERED = "DELIVERED",
  CANCELLED = "CANCELLED",
}

export enum Tag {
  Security = "Security",
  AI = "AI",
  Blockchain = "Blockchain",
}

export enum TestEnum {
  A = "A",
  B = "B",
  C = "C",
  D = "D",
  E = "E",
  F = "F",
  G = "G",
}

export interface BigNumbers {
  a: number
  b: number
  
}

export interface BinaryNode {
  data: number
  left?: BinaryNode | null
  right?: BinaryNode | null
  
}

export interface Blah {
  prop4?: string | null
  
}

export interface BlockConstraint {
  foo: number
  bar: string
  
}

export interface BlockConstraintForParam {
  bcfp: number
  bcfp2: string
  
}

export interface BookOrder {
  orderId: string
  title: string
  quantity: number
  price: number
  
}

export interface ClassOptionalOutput {
  prop1: string
  prop2: string
  
}

export interface ClassOptionalOutput2 {
  prop1?: string | null
  prop2?: string | null
  prop3?: Blah | null
  
}

export interface ClassWithImage {
  myImage: Image
  param2: string
  fake_image: FakeImage
  
}

export interface CompoundBigNumbers {
  big: BigNumbers
  big_nums: BigNumbers[]
  another: BigNumbers
  
}

export interface ContactInfo {
  primary: PhoneNumber | EmailAddress
  secondary?: PhoneNumber | EmailAddress | null
  
}

export interface CustomTaskResult {
  bookOrder?: BookOrder | null | null
  flightConfirmation?: FlightConfirmation | null | null
  groceryReceipt?: GroceryReceipt | null | null
  
}

export interface DummyOutput {
  nonce: string
  nonce2: string
  
  [key: string]: any;
}

export interface DynInputOutput {
  testKey: string
  
  [key: string]: any;
}

export interface DynamicClassOne {
  
  [key: string]: any;
}

export interface DynamicClassTwo {
  hi: string
  some_class: SomeClassNestedDynamic
  status: (string | DynEnumOne)
  
  [key: string]: any;
}

export interface DynamicOutput {
  
  [key: string]: any;
}

export interface Earthling {
  age: Checked<number,"earth_aged" | "no_infants">
  
}

export interface Education {
  institution: string
  location: string
  degree: string
  major: string[]
  graduation_date?: string | null
  
}

export interface Email {
  subject: string
  body: string
  from_address: string
  
}

export interface EmailAddress {
  value: string
  
}

export interface Event {
  title: string
  date: string
  location: string
  description: string
  
}

export interface FakeImage {
  url: string
  
}

export interface FlightConfirmation {
  confirmationNumber: string
  flightNumber: string
  departureTime: string
  arrivalTime: string
  seatNumber: string
  
}

export interface FooAny {
  planetary_age: Martian | Earthling
  certainty: Checked<number,"unreasonably_certain">
  species: Checked<string,"regex_bad" | "regex_good" | "trivial">
  
}

export interface Forest {
  trees: Tree[]
  
}

export interface GroceryReceipt {
  receiptId: string
  storeName: string
  items: (string | number | number)[]
  totalAmount: number
  
}

export interface InnerClass {
  prop1: string
  prop2: string
  inner: InnerClass2
  
}

export interface InnerClass2 {
  prop2: number
  prop3: number
  
}

export interface InputClass {
  key: string
  key2: string
  
}

export interface InputClassNested {
  key: string
  nested: InputClass
  
}

export interface LinkedList {
  head?: Node | null
  len: number
  
}

export interface LiteralClassHello {
  prop: "hello"
  
}

export interface LiteralClassOne {
  prop: "one"
  
}

export interface LiteralClassTwo {
  prop: "two"
  
}

export interface MalformedConstraints {
  foo: Checked<number,"foo_check">
  
}

export interface MalformedConstraints2 {
  foo: number
  
}

/**
 * A Martian organism with an age.
 * Such a nice type.
 */
export interface Martian {
  /**
   * The age of the Martian in Mars years.
   * So many Mars years.
   */
  age: Checked<number,"young_enough">
  
}

export interface NamedArgsSingleClass {
  key: string
  key_two: boolean
  key_three: number
  
}

export interface Nested {
  prop3?: string | null | null
  prop4?: string | null | null
  prop20: Nested2
  
}

export interface Nested2 {
  prop11?: string | null | null
  prop12?: string | null | null
  
}

export interface NestedBlockConstraint {
  nbc: Checked<BlockConstraint,"cross_field">
  
}

export interface NestedBlockConstraintForParam {
  nbcfp: BlockConstraintForParam
  
}

export interface Node {
  data: number
  next?: Node | null
  
}

export interface OptionalTest_Prop1 {
  omega_a: string
  omega_b: number
  
}

export interface OptionalTest_ReturnType {
  omega_1?: OptionalTest_Prop1 | null
  omega_2?: string | null
  omega_3: (OptionalTest_CategoryType | null)[]
  
}

export interface OrderInfo {
  order_status: OrderStatus
  tracking_number?: string | null
  estimated_arrival_date?: string | null
  
}

export interface OriginalA {
  value: number
  
}

export interface OriginalB {
  value: number
  
  [key: string]: any;
}

export interface Person {
  name?: string | null
  hair_color?: (string | Color) | null
  
  [key: string]: any;
}

export interface PhoneNumber {
  value: string
  
}

export interface Quantity {
  amount: number | number
  unit?: string | null
  
}

export interface RaysData {
  dataType: DataType
  value: Resume | Event
  
}

export interface ReceiptInfo {
  items: ReceiptItem[]
  total_cost?: number | null
  venue: "barisa" | "ox_burger"
  
}

export interface ReceiptItem {
  name: string
  description?: string | null
  quantity: number
  price: number
  
}

export interface Recipe {
  ingredients: Record<string, Quantity>
  recipe_type: "breakfast" | "dinner"
  
}

export interface Resume {
  name: string
  email: string
  phone: string
  experience: Education[]
  education: string[]
  skills: string[]
  
}

export interface Schema {
  prop1?: string | null | null
  prop2: Nested | string
  prop5: (string | null | null)[]
  prop6: string | Nested[]
  nested_attrs: (string | null | null | Nested)[]
  parens?: string | null | null
  other_group: string | number | string
  
}

export interface SearchParams {
  dateRange?: number | null
  location: string[]
  jobTitle?: WithReasoning | null
  company?: WithReasoning | null
  description: WithReasoning[]
  tags: (Tag | string)[]
  
}

export interface SomeClassNestedDynamic {
  hi: string
  
  [key: string]: any;
}

export interface StringToClassEntry {
  word: string
  
}

export interface TestClassAlias {
  key: string
  key2: string
  key3: string
  key4: string
  key5: string
  
}

export interface TestClassNested {
  prop1: string
  prop2: InnerClass
  
}

export interface TestClassWithEnum {
  prop1: string
  prop2: EnumInClass
  
}

export interface TestOutputClass {
  prop1: string
  prop2: number
  
}

export interface Tree {
  data: number
  children: Forest
  
}

export interface TwoStoriesOneTitle {
  title: string
  story_a: string
  story_b: string
  
}

export interface UnionTest_ReturnType {
  prop1: string | boolean
  prop2: (number | boolean)[]
  prop3: boolean[] | number[]
  
}

export interface WithReasoning {
  value: string
  reasoning: string
  
}
