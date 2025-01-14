
'use server'

import { b, UniverseQuestion, UniverseQuestionInput } from '../../baml_client'
import { ServerAction } from '../../baml_client/react/types'

/**
 * Server action for testing AWS functionality
 */
export const CustomTestAwsAction = (async (
  input: string,
  options?: { stream?: boolean }
) => {
  if (options?.stream) {
    const stream = b.stream.TestAws(input);
    return stream.toStreamable();
  }
  return b.TestAws(input);
}) as ServerAction<string, string>;

/**
 * Server action for testing universe questions
 */
export const CustomTestUniverseQuestionAction = (async (
  input: UniverseQuestionInput,
  options?: { stream?: boolean }
) => {
  if (options?.stream) {
    const stream = b.stream.TestUniverseQuestion(input);
    return stream.toStreamable();
  }
  return b.TestUniverseQuestion(input);
}) as ServerAction<UniverseQuestionInput, UniverseQuestion>;
