// This file is auto-generated. Do not edit this file manually.
//
// Disable formatting for this file to avoid linting errors.
// tslint:disable
// @ts-nocheck
/* eslint-disable */


import { GPT4 } from '../client';
import { ClassifyMessage } from '../function';
import { schema } from '../json_schema';
import { Deserializer } from '@boundaryml/baml-core/deserializer/deserializer';


// Impl: default_config
// Client: GPT4
// An implementation for ClassifyMessage


const prompt_template = `Classify the following INPUT into ONE
of the following categories:

INPUT: {{ input }}

{{ ctx.output_schema }}

Response:`;
const output_schema = `"Category as string"

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
  $ref: '#/definitions/ClassifyMessage_output'
});

ClassifyMessage.registerImpl('default_config', async (
  args: {
    input: string
  }
): Promise<Category> => {
    const result = await GPT4.run_jinja_template(
      prompt_template,
      args,
      output_schema,
      template_macros,
    );

    return deserializer.coerce(result.generated);
  }
);


