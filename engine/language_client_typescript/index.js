"use strict";
var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        if (typeof b !== "function" && b !== null)
            throw new TypeError("Class extends value " + String(b) + " is not a constructor or null");
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.BamlValidationError = exports.BamlClientFinishReasonError = exports.BamlCtxManager = exports.BamlStream = exports.BamlLogEvent = exports.ClientRegistry = exports.invoke_runtime_cli = exports.Audio = exports.ClientBuilder = exports.Image = exports.FunctionResultStream = exports.FunctionResult = exports.BamlRuntime = void 0;
exports.createBamlValidationError = createBamlValidationError;
var native_1 = require("./native");
Object.defineProperty(exports, "BamlRuntime", { enumerable: true, get: function () { return native_1.BamlRuntime; } });
Object.defineProperty(exports, "FunctionResult", { enumerable: true, get: function () { return native_1.FunctionResult; } });
Object.defineProperty(exports, "FunctionResultStream", { enumerable: true, get: function () { return native_1.FunctionResultStream; } });
Object.defineProperty(exports, "Image", { enumerable: true, get: function () { return native_1.BamlImage; } });
Object.defineProperty(exports, "ClientBuilder", { enumerable: true, get: function () { return native_1.ClientBuilder; } });
Object.defineProperty(exports, "Audio", { enumerable: true, get: function () { return native_1.BamlAudio; } });
Object.defineProperty(exports, "invoke_runtime_cli", { enumerable: true, get: function () { return native_1.invoke_runtime_cli; } });
Object.defineProperty(exports, "ClientRegistry", { enumerable: true, get: function () { return native_1.ClientRegistry; } });
Object.defineProperty(exports, "BamlLogEvent", { enumerable: true, get: function () { return native_1.BamlLogEvent; } });
var stream_1 = require("./stream");
Object.defineProperty(exports, "BamlStream", { enumerable: true, get: function () { return stream_1.BamlStream; } });
var async_context_vars_1 = require("./async_context_vars");
Object.defineProperty(exports, "BamlCtxManager", { enumerable: true, get: function () { return async_context_vars_1.BamlCtxManager; } });
var BamlClientFinishReasonError = /** @class */ (function (_super) {
    __extends(BamlClientFinishReasonError, _super);
    function BamlClientFinishReasonError(prompt, raw_output, message, finish_reason) {
        var _this = _super.call(this, message) || this;
        _this.name = 'BamlClientFinishReasonError';
        _this.prompt = prompt;
        _this.raw_output = raw_output;
        _this.finish_reason = finish_reason;
        Object.setPrototypeOf(_this, BamlClientFinishReasonError.prototype);
        return _this;
    }
    BamlClientFinishReasonError.prototype.toJSON = function () {
        return JSON.stringify({
            name: this.name,
            message: this.message,
            raw_output: this.raw_output,
            prompt: this.prompt,
            finish_reason: this.finish_reason,
        }, null, 2);
    };
    BamlClientFinishReasonError.from = function (error) {
        if (error.message.includes('BamlClientFinishReasonError')) {
            try {
                var errorData = JSON.parse(error.message);
                if (errorData.type === 'BamlClientFinishReasonError') {
                    return new BamlClientFinishReasonError(errorData.prompt || '', errorData.raw_output || '', errorData.message || error.message, errorData.finish_reason);
                }
                else {
                    console.warn('Not a BamlClientFinishReasonError:', error);
                }
            }
            catch (parseError) {
                // If JSON parsing fails, fall back to the original error
                console.warn('Failed to parse BamlClientFinishReasonError:', parseError);
            }
        }
        return undefined;
    };
    return BamlClientFinishReasonError;
}(Error));
exports.BamlClientFinishReasonError = BamlClientFinishReasonError;
var BamlValidationError = /** @class */ (function (_super) {
    __extends(BamlValidationError, _super);
    function BamlValidationError(prompt, raw_output, message) {
        var _this = _super.call(this, message) || this;
        _this.name = 'BamlValidationError';
        _this.prompt = prompt;
        _this.raw_output = raw_output;
        Object.setPrototypeOf(_this, BamlValidationError.prototype);
        return _this;
    }
    BamlValidationError.prototype.toJSON = function () {
        return JSON.stringify({
            name: this.name,
            message: this.message,
            raw_output: this.raw_output,
            prompt: this.prompt,
        }, null, 2);
    };
    BamlValidationError.from = function (error) {
        if (error.message.includes('BamlValidationError')) {
            try {
                var errorData = JSON.parse(error.message);
                if (errorData.type === 'BamlValidationError') {
                    return new BamlValidationError(errorData.prompt || '', errorData.raw_output || '', errorData.message || error.message);
                }
            }
            catch (parseError) {
                console.warn('Failed to parse BamlValidationError:', parseError);
            }
        }
        return undefined;
    };
    return BamlValidationError;
}(Error));
exports.BamlValidationError = BamlValidationError;
// Helper function to safely create a BamlValidationError
function createBamlValidationError(error) {
    var bamlValidationError = BamlValidationError.from(error);
    if (bamlValidationError) {
        return bamlValidationError;
    }
    var bamlClientFinishReasonError = BamlClientFinishReasonError.from(error);
    if (bamlClientFinishReasonError) {
        return bamlClientFinishReasonError;
    }
    // otherwise return the original error
    return error;
}
// No need for a separate throwBamlValidationError function in TypeScript
