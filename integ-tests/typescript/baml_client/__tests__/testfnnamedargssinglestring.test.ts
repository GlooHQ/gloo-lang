// This file is auto-generated. Do not edit this file manually.
//
// Disable formatting for this file to avoid linting errors.
// tslint:disable
// @ts-nocheck

import b from '../';

import { traceAsync, FireBamlEvent } from '@boundaryml/baml-core/ffi_layer';


describe('test_case:case1', () => {
  const test_fn = traceAsync('case1', 'null', [['impl', 'string']], 'positional', async (impl) => {
    FireBamlEvent.tags({
      'test_dataset_name': 'TestFnNamedArgsSingleString',
      'test_case_name': 'test',
      'test_case_arg_name': `case1[${impl}]`,
      'test_cycle_id': process.env.BOUNDARY_PROCESS_ID || 'local-run',
    });
    const test_case = { "myString": "hellothere.\n\nSome new lines\n\n\\n\\n\n\n\"\"\"triple quote string\"\"\"\n\nsome json:\n```json\n{\n    \"hi\": \"there\"\n}\n```\nSingle chars\n(\n{\n{}\nXML Tags:\n<hi>hey</hi>\n\n" };
    const result = await b.TestFnNamedArgsSingleString.getImpl(impl).run(
      test_case
    );
  });

  describe('function:TestFnNamedArgsSingleString', () => {
    test('impl:v1', async () => {
      await test_fn('v1');
    });
  });
});
