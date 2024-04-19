// This file is auto-generated. Do not edit this file manually.
//
// Disable formatting for this file to avoid linting errors.
// tslint:disable
// @ts-nocheck
/* eslint-disable */


import { GPT4 } from '../client';
import { GetOrderInfo } from '../function';
import { schema } from '../json_schema';
import { InternalEmail } from '../types_internal';
import { Deserializer } from '@boundaryml/baml-core/deserializer/deserializer';


// Impl: default_config
// Client: GPT4
// An implementation for GetOrderInfo


const prompt_template = `Given the email below:

\`\`\`
from: {{email.from_address}}
Email Subject: {{email.subject}}
Email Body: {{email.body}}
\`\`\`

Extract this info from the email in JSON format:
{{ ctx.output_format }}

Before you output the JSON, please explain your
reasoning step-by-step. Here is an example on how to do this:
'If we think step by step we can see that ...
 therefore the output JSON is:
{
  ... the json schema ...
}'`;
const output_format = `{
  "order_status": "OrderStatus as string",
  "tracking_number": string | null,
  "estimated_arrival_date": string | null
}

OrderStatus
---
ORDERED
SHIPPED
DELIVERED
CANCELLED`;

const template_macros = [
]

const deserializer = new Deserializer<OrderInfo>(schema, {
  $ref: '#/definitions/GetOrderInfo_output'
});

GetOrderInfo.registerImpl('default_config', async (
  args: {
    email: Email
  }
): Promise<OrderInfo> => {
    const result = await GPT4.run_jinja_template(
      prompt_template,
      args,
      output_format,
      template_macros,
    );

    return deserializer.coerce(result.generated);
  }
);


