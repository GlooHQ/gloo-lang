// import the native typebuilder that was compiled from rust code using napi-rs
// this provides the core functionality that our typescript wrapper will use
import { TypeBuilder as NativeTypeBuilder } from '../../native';

export class TypeBuilder {
  // holds the instance of the native rust implementation
  // the native instance handles all the actual type building logic
  // we just provide a nice typescript interface on top
  private native: NativeTypeBuilder;

  constructor() {
    // instantiate a new native typebuilder when this wrapper is created
    // this sets up the underlying rust state that we'll delegate to
    this.native = new NativeTypeBuilder();
  }

  // creates a new class definition or returns an existing one with the given name
  // we renamed this from addclass/class_ to getclass to better match what the native api expects
  // this is used to define the structure and properties of classes in the type system
  getClass(name: string) {
    // pass the class creation request through to the native rust implementation
    // the rust code handles all the details of managing the class definition
    return this.native.getClass(name);
  }

  // creates a new enum definition or returns an existing one with the given name
  // we renamed this from addenum/enum to getenum to better match what the native api expects
  // this is used to define enums with their possible values and metadata
  getEnum(name: string) {
    // delegate enum creation to the native rust implementation
    // the rust code manages the enum definition and its allowed values
    return this.native.getEnum(name);
  }

  // converts the entire type definition to a human-readable string representation
  // useful for debugging and seeing the full structure of defined types
  toString(): string {
    // let the native rust code generate the string representation
    // it will include all classes and enums with their properties, values and metadata
    // formatted in a consistent way for easy reading
    return this.native.toString();
  }
}