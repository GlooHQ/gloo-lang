"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.EnumBuilder = exports.ClassBuilder = exports.TypeBuilder = void 0;
var native_1 = require("./native");
var TypeBuilder = /** @class */ (function () {
    function TypeBuilder(_a) {
        var classes = _a.classes, enums = _a.enums;
        this.classes = classes;
        this.enums = enums;
        this.tb = new native_1.TypeBuilder();
    }
    TypeBuilder.prototype._tb = function () {
        return this.tb;
    };
    TypeBuilder.prototype.null = function () {
        return this.tb.null();
    };
    TypeBuilder.prototype.string = function () {
        return this.tb.string();
    };
    TypeBuilder.prototype.literalString = function (value) {
        return this.tb.literalString(value);
    };
    TypeBuilder.prototype.literalInt = function (value) {
        return this.tb.literalInt(value);
    };
    TypeBuilder.prototype.literalBool = function (value) {
        return this.tb.literalBool(value);
    };
    TypeBuilder.prototype.int = function () {
        return this.tb.int();
    };
    TypeBuilder.prototype.float = function () {
        return this.tb.float();
    };
    TypeBuilder.prototype.bool = function () {
        return this.tb.bool();
    };
    TypeBuilder.prototype.list = function (type) {
        return this.tb.list(type);
    };
    TypeBuilder.prototype.map = function (keyType, valueType) {
        return this.tb.map(keyType, valueType);
    };
    TypeBuilder.prototype.union = function (types) {
        return this.tb.union(types);
    };
    TypeBuilder.prototype.classBuilder = function (name, properties) {
        return new ClassBuilder(this.tb, name, new Set(properties));
    };
    TypeBuilder.prototype.enumBuilder = function (name, values) {
        return new EnumBuilder(this.tb, name, new Set(values));
    };
    TypeBuilder.prototype.addClass = function (name) {
        if (this.classes.has(name)) {
            throw new Error("Class ".concat(name, " already exists"));
        }
        if (this.enums.has(name)) {
            throw new Error("Enum ".concat(name, " already exists"));
        }
        this.classes.add(name);
        return new ClassBuilder(this.tb, name);
    };
    TypeBuilder.prototype.addEnum = function (name) {
        if (this.classes.has(name)) {
            throw new Error("Class ".concat(name, " already exists"));
        }
        if (this.enums.has(name)) {
            throw new Error("Enum ".concat(name, " already exists"));
        }
        this.enums.add(name);
        return new EnumBuilder(this.tb, name);
    };
    return TypeBuilder;
}());
exports.TypeBuilder = TypeBuilder;
var ClassBuilder = /** @class */ (function () {
    function ClassBuilder(tb, name, properties) {
        if (properties === void 0) { properties = new Set(); }
        this.properties = properties;
        this.bldr = tb.getClass(name);
    }
    ClassBuilder.prototype.type = function () {
        return this.bldr.field();
    };
    ClassBuilder.prototype.listProperties = function () {
        var _this = this;
        return Array.from(this.properties).map(function (name) { return [name, new ClassPropertyBuilder(_this.bldr.property(name))]; });
    };
    ClassBuilder.prototype.addProperty = function (name, type) {
        if (this.properties.has(name)) {
            throw new Error("Property ".concat(name, " already exists."));
        }
        this.properties.add(name);
        return new ClassPropertyBuilder(this.bldr.property(name).setType(type));
    };
    ClassBuilder.prototype.property = function (name) {
        if (!this.properties.has(name)) {
            throw new Error("Property ".concat(name, " not found."));
        }
        return new ClassPropertyBuilder(this.bldr.property(name));
    };
    return ClassBuilder;
}());
exports.ClassBuilder = ClassBuilder;
var ClassPropertyBuilder = /** @class */ (function () {
    function ClassPropertyBuilder(bldr) {
        this.bldr = bldr;
    }
    ClassPropertyBuilder.prototype.alias = function (alias) {
        this.bldr.alias(alias);
        return this;
    };
    ClassPropertyBuilder.prototype.description = function (description) {
        this.bldr.description(description);
        return this;
    };
    return ClassPropertyBuilder;
}());
var EnumBuilder = /** @class */ (function () {
    function EnumBuilder(tb, name, values) {
        if (values === void 0) { values = new Set(); }
        this.values = values;
        this.bldr = tb.getEnum(name);
    }
    EnumBuilder.prototype.type = function () {
        return this.bldr.field();
    };
    EnumBuilder.prototype.value = function (name) {
        if (!this.values.has(name)) {
            throw new Error("Value ".concat(name, " not found."));
        }
        return this.bldr.value(name);
    };
    EnumBuilder.prototype.listValues = function () {
        var _this = this;
        return Array.from(this.values).map(function (name) { return [name, _this.bldr.value(name)]; });
    };
    EnumBuilder.prototype.addValue = function (name) {
        if (this.values.has(name)) {
            throw new Error("Value ".concat(name, " already exists."));
        }
        this.values.add(name);
        return this.bldr.value(name);
    };
    return EnumBuilder;
}());
exports.EnumBuilder = EnumBuilder;
