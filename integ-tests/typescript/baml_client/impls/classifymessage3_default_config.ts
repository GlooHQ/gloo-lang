// This file is auto-generated. Do not edit this file manually.
//
// Disable formatting for this file to avoid linting errors.
// tslint:disable
// @ts-nocheck
/* eslint-disable */


import { GPT4 } from '../client';
import { ClassifyMessage3 } from '../function';
import { schema } from '../json_schema';
import { Deserializer } from '@boundaryml/baml-core/deserializer/deserializer';


// Impl: default_config
// Client: GPT4
// An implementation for ClassifyMessage3


const prompt_template = `Classify the following INPUT into ONE
of the following categories:

INPUT: {{ input }}

{{ ctx.output_format }}

Response:`;
const output_format = `"Category as string"

Category
---
Refund
CancelOrder
TechnicalSupport
AccountIssue
Question`;

const template_macros = [
]

const deserializer = new Deserializer<Category>(schema, {
  $ref: '#/definitions/ClassifyMessage3_output'
});

ClassifyMessage3.registerImpl('default_config', async (
  args: {
    input: string
  }
): Promise<Category> => {
    const result = await GPT4.run_jinja_template(
      prompt_template,
      args,
      output_format,
      template_macros,
    );

    return deserializer.coerce(result.generated);
  }
);


