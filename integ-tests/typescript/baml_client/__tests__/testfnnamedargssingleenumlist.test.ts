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
      'test_dataset_name': 'TestFnNamedArgsSingleEnumList',
      'test_case_name': 'test',
      'test_case_arg_name': `case1[${impl}]`,
      'test_cycle_id': process.env.BOUNDARY_PROCESS_ID || 'local-run',
    });
    const test_case = { "myArg": ["ONE", "ONE", "TWO"] };
    const result = await b.TestFnNamedArgsSingleEnumList.getImpl(impl).run(
      test_case
    );
  });

  describe('function:TestFnNamedArgsSingleEnumList', () => {
    test('impl:v1', async () => {
      await test_fn('v1');
    });
  });
});
