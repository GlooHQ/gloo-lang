type PartialResponse<Output> = {
  partial?: Output | null
  final?: never
}

type FinalResponse<Output> = {
  final: Output
  partial?: never
}


type StreamingInputProps<Output> = {
  stream: true
  onPartial?: (response: PartialResponse<Output>) => void
  onFinal?: (response: FinalResponse<Output>) => void
}

type NonStreamingInputProps<Output> = {
  stream?: false
  onPartial?: never
  onFinal?: (response: FinalResponse<Output>) => void
}

type UseLLMOptions<Output> = (StreamingInputProps<Output> | NonStreamingInputProps<Output>)
// }

type ServerActionType<Input, Output> = (input: Input) => Output

function isStreamingOptions<Output>(
  options: UseLLMOptions<Output>
): options is StreamingInputProps<Output> {
  return options.stream === true;
}

// Overload signatures
function useLLM<Input, Output>(
  action: ServerActionType<Input, Output>,
  input: Input,
  options: StreamingInputProps<Output>
): PartialResponse<Output>;

function useLLM<Input, Output>(
  action: ServerActionType<Input, Output>,
  input: Input,
  options: NonStreamingInputProps<Output>
): FinalResponse<Output>;

// Implementation
function useLLM<Input, Output>(
  action: ServerActionType<Input, Output>,
  input: Input,
  options: UseLLMOptions<Output>
): PartialResponse<Output> | FinalResponse<Output> {
  if (isStreamingOptions(options)) {
    const retVal = action(input);
    options.onPartial?.({ partial: retVal });
    options.onFinal?.({ final: retVal });
    return { partial: retVal };
  } else {
    const retVal = action(input);
    options.onFinal?.({ final: retVal });
    return { final: retVal };
  }
}

(async () => {
  const action = (input: string) => input
  // Test with streaming - TypeScript will infer this as PartialResponse
  const streamResponse = useLLM(action, "Hello, world!", {
    // prompt: 'Hello, world!',
    stream: true,
    onPartial: (response) => console.log('Partial:', response.partial),
    onFinal: (response) => console.log('Final:', response.final)
  })
  console.log('Streaming response:', streamResponse.partial) // TypeScript knows this is PartialResponse

  // Test without streaming - TypeScript will infer this as FinalResponse
  const nonStreamResponse = useLLM(action, "Hello, world!", {
    // prompt: 'Hello, world!',
    onFinal: (response) => console.log('Final:', response.final)
  })
  console.log('Non-streaming response:', nonStreamResponse.final) // TypeScript knows this is FinalResponse
})()

