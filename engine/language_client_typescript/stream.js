"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.BamlStream = void 0;
class BamlStream {
    ffiStream;
    partialCoerce;
    finalCoerce;
    ctxManager;
    task = null;
    eventQueue = [];
    constructor(ffiStream, partialCoerce, finalCoerce, ctxManager) {
        this.ffiStream = ffiStream;
        this.partialCoerce = partialCoerce;
        this.finalCoerce = finalCoerce;
        this.ctxManager = ctxManager;
    }
    async driveToCompletion() {
        try {
            this.ffiStream.onEvent((err, data) => {
                if (err) {
                    console.log('errorrr', err);
                    this.eventQueue.push(err);
                }
                else {
                    console.log('data', data);
                    this.eventQueue.push(data);
                }
            });
            try {
                const retval = await this.ffiStream.done(this.ctxManager);
                console.log('retval', retval);
                return retval;
            }
            catch (err) {
                this.eventQueue.push(err);
                throw err;
            }
        }
        finally {
            this.eventQueue.push(null);
        }
    }
    driveToCompletionInBg() {
        if (this.task === null) {
            this.task = this.driveToCompletion();
        }
        return this.task;
    }
    async *[Symbol.asyncIterator]() {
        const backgroundTask = this.driveToCompletionInBg();
        while (true) {
            const event = this.eventQueue.shift();
            if (event === undefined) {
                await new Promise((resolve) => setTimeout(resolve, 100));
                continue;
            }
            if (event === null) {
                break;
            }
            console.log('event', event);
            if (event instanceof Error) {
                throw event;
            }
            else if (event.code === "GenericFailure") {
                console.log('event code', event.code);
                console.log('event', event);
                console.log('event indiex 0', event[0]);
                throw new Error(event[0]);
            }
            else if (event.isOk()) {
                yield this.partialCoerce(event.parsed());
            }
            else {
                throw new Error(event.error());
            }
        }
        // await backgroundTask
    }
    async getFinalResponse() {
        const final = await this.driveToCompletionInBg();
        return this.finalCoerce(final.parsed());
    }
}
exports.BamlStream = BamlStream;
