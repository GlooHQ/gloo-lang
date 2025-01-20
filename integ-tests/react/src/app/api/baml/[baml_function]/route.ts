import { NextRequest, NextResponse } from 'next/server'
import { b } from '../../../../../baml_client'

export async function POST(
  req: NextRequest,
  { params: routeParams }: { params: Promise<{ baml_function: string }> }
) {
  try {
    const { baml_function } = await routeParams
    const functionParams = await req.json()

    console.log('baml_function', baml_function)

    // Type check if the function exists on the BAML client
    if (!(baml_function in b.stream)) {
      return NextResponse.json(
        { error: `Function ${baml_function} not found` },
        { status: 404 }
      )
    }

    // Get the function from the BAML client
    // const bamlFn = b.stream[baml_function as keyof typeof b.stream]
    // if (typeof bamlFn !== 'function') {
    //   return NextResponse.json(
    //     { error: `${baml_function} is not a function` },
    //     { status: 400 }
    //   )
    // }

    // Create a TransformStream for streaming the response
    const stream = new TransformStream()
    const writer = stream.writable.getWriter()
    const encoder = new TextEncoder()

    // Start streaming the response
    // const bamlStream = bamlFn(functionParams)

    // Process the stream in the background
    // ;(async () => {
    //   try {
    //     for await (const chunk of bamlStream) {
    //       // Send partial results
    //       await writer.write(
    //         encoder.encode(
    //           JSON.stringify({ partial: chunk }) + '\n'
    //         )
    //       )
    //     }

    //     // Send final result
    //     const finalResponse = await bamlStream.getFinalResponse()
    //     await writer.write(
    //       encoder.encode(
    //         JSON.stringify({ final: finalResponse }) + '\n'
    //       )
    //     )
    //   } catch (error) {
    //     // Send error if something goes wrong
    //     await writer.write(
    //       encoder.encode(
    //         JSON.stringify({
    //           error: error instanceof Error ? error.message : 'Unknown error'
    //         }) + '\n'
    //       )
    //     )
    //   } finally {
    //     await writer.close()
    //   }
    // })()

    // Return the stream response
    return new NextResponse(stream.readable, {
      headers: {
        'Content-Type': 'application/x-ndjson',
        'Transfer-Encoding': 'chunked',
      },
    })
  } catch (error) {
    console.error('API Error:', error)
    return NextResponse.json(
      {
        error: error instanceof Error ? error.message : 'Internal server error'
      },
      { status: 500 }
    )
  }
}
