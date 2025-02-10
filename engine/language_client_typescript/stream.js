"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g = Object.create((typeof Iterator === "function" ? Iterator : Object).prototype);
    return g.next = verb(0), g["throw"] = verb(1), g["return"] = verb(2), typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
var __await = (this && this.__await) || function (v) { return this instanceof __await ? (this.v = v, this) : new __await(v); }
var __asyncGenerator = (this && this.__asyncGenerator) || function (thisArg, _arguments, generator) {
    if (!Symbol.asyncIterator) throw new TypeError("Symbol.asyncIterator is not defined.");
    var g = generator.apply(thisArg, _arguments || []), i, q = [];
    return i = Object.create((typeof AsyncIterator === "function" ? AsyncIterator : Object).prototype), verb("next"), verb("throw"), verb("return", awaitReturn), i[Symbol.asyncIterator] = function () { return this; }, i;
    function awaitReturn(f) { return function (v) { return Promise.resolve(v).then(f, reject); }; }
    function verb(n, f) { if (g[n]) { i[n] = function (v) { return new Promise(function (a, b) { q.push([n, v, a, b]) > 1 || resume(n, v); }); }; if (f) i[n] = f(i[n]); } }
    function resume(n, v) { try { step(g[n](v)); } catch (e) { settle(q[0][3], e); } }
    function step(r) { r.value instanceof __await ? Promise.resolve(r.value.v).then(fulfill, reject) : settle(q[0][2], r); }
    function fulfill(value) { resume("next", value); }
    function reject(value) { resume("throw", value); }
    function settle(f, v) { if (f(v), q.shift(), q.length) resume(q[0][0], q[0][1]); }
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.BamlStream = void 0;
var index_1 = require("./index");
var BamlStream = /** @class */ (function () {
    function BamlStream(ffiStream, partialCoerce, finalCoerce, ctxManager) {
        this.ffiStream = ffiStream;
        this.partialCoerce = partialCoerce;
        this.finalCoerce = finalCoerce;
        this.ctxManager = ctxManager;
        this.task = null;
        this.eventQueue = [];
        this.isCancelled = false;
    }
    /**
     * Cancels the stream and stops processing
     */
    BamlStream.prototype.cancel = function () {
        if (this.isCancelled) {
            console.log('[TS] Stream already cancelled');
            return;
        }
        console.log('[TS] Cancelling stream...');
        this.isCancelled = true;
        this.ctxManager.setCancelled();
        console.log('[TS] Context cancelled');
        // Clear the queue to stop iteration
        this.eventQueue = [];
        this.eventQueue.push(null);
        console.log('[TS] Event queue cleared');
    };
    BamlStream.prototype.driveToCompletion = function () {
        return __awaiter(this, void 0, void 0, function () {
            var retval;
            var _this = this;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        _a.trys.push([0, , 2, 3]);
                        console.log('[TS] Setting up event handler');
                        this.ffiStream.onEvent(function (err, data) {
                            if (err) {
                                console.log('[TS] Event error:', err);
                                return;
                            }
                            else {
                                console.log('[TS] Received event data');
                                _this.eventQueue.push(data);
                            }
                        });
                        console.log('[TS] Waiting for stream completion');
                        return [4 /*yield*/, this.ffiStream.done(this.ctxManager)];
                    case 1:
                        retval = _a.sent();
                        console.log('[TS] Stream completed');
                        return [2 /*return*/, retval];
                    case 2:
                        console.log('[TS] Pushing final null to queue');
                        this.eventQueue.push(null);
                        return [7 /*endfinally*/];
                    case 3: return [2 /*return*/];
                }
            });
        });
    };
    BamlStream.prototype.driveToCompletionInBg = function () {
        if (this.task === null) {
            console.log('[TS] Starting background task');
            this.task = this.driveToCompletion();
        }
        return this.task;
    };
    BamlStream.prototype[Symbol.asyncIterator] = function () {
        return __asyncGenerator(this, arguments, function _a() {
            var event_1;
            return __generator(this, function (_b) {
                switch (_b.label) {
                    case 0:
                        this.driveToCompletionInBg();
                        _b.label = 1;
                    case 1:
                        if (!!this.isCancelled) return [3 /*break*/, 7];
                        event_1 = this.eventQueue.shift();
                        if (!(event_1 === undefined)) return [3 /*break*/, 3];
                        return [4 /*yield*/, __await(new Promise(function (resolve) { return setTimeout(resolve, 100); }))];
                    case 2:
                        _b.sent();
                        return [3 /*break*/, 1];
                    case 3:
                        if (event_1 === null) {
                            console.log('[TS] Iterator received null event, breaking');
                            return [3 /*break*/, 7];
                        }
                        if (!event_1.isOk()) return [3 /*break*/, 6];
                        console.log('[TS] Yielding event data');
                        return [4 /*yield*/, __await(this.partialCoerce(event_1.parsed(true)))];
                    case 4: return [4 /*yield*/, _b.sent()];
                    case 5:
                        _b.sent();
                        _b.label = 6;
                    case 6: return [3 /*break*/, 1];
                    case 7:
                        if (this.isCancelled) {
                            console.log('[TS] Iterator throwing cancellation error');
                            throw new index_1.BamlClientFinishReasonError('', '', 'Stream was cancelled', 'Stream was cancelled');
                        }
                        return [2 /*return*/];
                }
            });
        });
    };
    BamlStream.prototype.getFinalResponse = function () {
        return __awaiter(this, void 0, void 0, function () {
            var final;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        if (this.isCancelled) {
                            console.log('[TS] getFinalResponse throwing cancellation error');
                            throw new index_1.BamlClientFinishReasonError('', '', 'Stream was cancelled', 'Stream was cancelled');
                        }
                        console.log('[TS] Getting final response');
                        return [4 /*yield*/, this.driveToCompletionInBg()];
                    case 1:
                        final = _a.sent();
                        return [2 /*return*/, this.finalCoerce(final.parsed(false))];
                }
            });
        });
    };
    return BamlStream;
}());
exports.BamlStream = BamlStream;
