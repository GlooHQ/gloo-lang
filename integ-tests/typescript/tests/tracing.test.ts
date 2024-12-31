import { traceSync, traceAsync } from '../baml_client/tracing';


function fnWithReturn() : string {
  console.log("fnWithReturn");
  return "Hello, world";
}

function fnWithNoReturn() : void {
  console.log("fnWithNoReturn");
}

async function asyncfnWithReturn() : Promise<string> {
  console.log("asyncfnWithReturn");
  return Promise.resolve("Hello, world");
}

async function asyncfnWithNoReturn() : Promise<void> {
  console.log("asyncfnWithNoReturn");
}


describe('Trace Test', () => {
  test('traceSync with Return', async () => {
      const tracedFnWithReturn = traceSync("tracedFnWithReturn", fnWithReturn);
      const result = tracedFnWithReturn();
      expect(result).toBeDefined();
      expect(result).toBe('Hello, world');
  });
  test('traceSync with no return', async () => {
      const tracedFnWithNoReturn = traceSync("tracedFnWithNoReturn", fnWithNoReturn);
      tracedFnWithNoReturn();
  });
  test('traceAsync with Return', async () => {
      const tracedFnWithReturn = traceAsync("tracedAsyncFnWithReturn", asyncfnWithReturn);
      const result = await tracedFnWithReturn();
      expect(result).toBeDefined();
      expect(result).toBe('Hello, world');
  });
  test('traceAsync with no return', async () => {
      const tracedFnWithNoReturn = traceAsync("tracedAsyncFnWithNoReturn", asyncfnWithNoReturn);
      await tracedFnWithNoReturn();
  });
});