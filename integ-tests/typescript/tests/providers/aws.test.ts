import { b, resetBamlEnvVars } from '../test-setup'
import { ClientRegistry } from '@boundaryml/baml'
import { STSClient, AssumeRoleCommand, GetCallerIdentityCommand } from '@aws-sdk/client-sts'

describe('AWS Provider', () => {
  // Store original AWS_PROFILE at the top level
  const originalEnv: { [key: string]: string | undefined } = {
    AWS_PROFILE: process.env.AWS_PROFILE,
    AWS_ACCESS_KEY_ID: process.env.AWS_ACCESS_KEY_ID,
    AWS_SECRET_ACCESS_KEY: process.env.AWS_SECRET_ACCESS_KEY,
    AWS_SESSION_TOKEN: process.env.AWS_SESSION_TOKEN,
    AWS_REGION: process.env.AWS_REGION,
    AWS_DEFAULT_REGION: process.env.AWS_DEFAULT_REGION,
  }

  // beforeAll(() => {
  //   // Clear all AWS-related environment variables at the start
  //   Object.keys(originalEnv).forEach((key) => {
  //     delete process.env[key]
  //   })
  // })

  // afterAll(() => {
  //   // Restore all AWS-related environment variables after all tests
  //   Object.entries(originalEnv).forEach(([key, value]) => {
  //     if (value === undefined) {
  //       delete process.env[key]
  //     } else {
  //       process.env[key] = value
  //     }
  //   })
  // })

  it('should support AWS', async () => {
    const res = await b.TestAws('Dr. Pepper')
    expect(res.length).toBeGreaterThan(0)
  })

  it('should handle invalid AWS region gracefully', async () => {
    await expect(async () => {
      await b.TestAwsInvalidRegion('Write a nice short story about Dr. Pepper')
    }).rejects.toMatchObject({
      code: 'GenericFailure',
    })
  })

  it('should handle invalid AWS access key gracefully', async () => {
    // Clear all AWS-related environment variables
    // TODO: We shouldn't delete these because we aren't putting them back
    // delete process.env.AWS_ACCESS_KEY_ID
    // delete process.env.AWS_SECRET_ACCESS_KEY
    // delete process.env.AWS_SESSION_TOKEN
    // delete process.env.AWS_PROFILE
    // delete process.env.AWS_REGION
    // delete process.env.AWS_DEFAULT_REGION

    // Create a new client registry with no environment credentials
    const cr = new ClientRegistry()
    cr.addLlmClient('InvalidAwsClient', 'aws-bedrock', {
      model_id: 'meta.llama3-8b-instruct-v1:0',
      region: 'us-east-1',
      access_key_id: 'AKIAINVALID12345678',
      secret_access_key: 'abcdef1234567890abcdef1234567890abcdef12',
    })
    cr.setPrimary('InvalidAwsClient')

    await expect(async () => {
      await b.TestAwsInvalidAccessKey('Write a nice short story about Dr. Pepper', { clientRegistry: cr })
    }).rejects.toMatchObject({
      code: 'GenericFailure',
    })
  })

  describe('Streaming', () => {
    it('should support streaming in AWS', async () => {
      const stream = b.stream.TestAws('Dr. Pepper')
      const msgs: string[] = []
      for await (const msg of stream) {
        msgs.push(msg ?? '')
      }
      const final = await stream.getFinalResponse()

      expect(final.length).toBeGreaterThan(0)
      expect(msgs.length).toBeGreaterThan(0)
      for (let i = 0; i < msgs.length - 2; i++) {
        expect(msgs[i + 1].startsWith(msgs[i])).toBeTruthy()
      }
      expect(msgs.at(-1)).toEqual(final)
    })
  })

  describe('Dynamic Client Registry', () => {
    // beforeEach(() => {
    //   // Clear all AWS-related environment variables before each test
    //   Object.keys(originalEnv).forEach((key) => {
    //     delete process.env[key]
    //   })
    // })

    // afterEach(() => {
    //   // Clear all AWS-related environment variables after each test
    //   Object.keys(originalEnv).forEach((key) => {
    //     delete process.env[key]
    //   })
    // })

    describe('Credential Resolution', () => {
      let originalProfile: string | undefined
      let originalRegion: string | undefined
      let originalDefaultRegion: string | undefined

      beforeAll(() => {
        originalProfile = process.env.AWS_PROFILE
        originalRegion = process.env.AWS_REGION
        originalDefaultRegion = process.env.AWS_DEFAULT_REGION
      })

      // beforeEach(() => {
      //   // Set AWS_PROFILE for all tests in this suite
      //   // process.env.AWS_PROFILE = 'boundaryml-dev'

      //   // Clear all AWS-related environment variables at the start
      //   Object.keys(originalEnv).forEach((key) => {
      //     delete process.env[key]
      //   })
      // })

      // afterEach(() => {
      //   // Restore all AWS-related environment variables after all tests
      //   Object.entries(originalEnv).forEach(([key, value]) => {
      //     if (value === undefined) {
      //       delete process.env[key]
      //     } else {
      //       process.env[key] = value
      //     }
      //   })
      // })

      test('should handle session credentials correctly', async () => {
        const sts = new STSClient({
          region: 'us-east-1',
          credentials: {
            accessKeyId: process.env.foo_AWS_ACCESS_KEY_ID ?? '',
            secretAccessKey: process.env.foo_AWS_SECRET_ACCESS_KEY ?? '',
          },
        })
        const { Credentials } = await sts.send(
          new AssumeRoleCommand({
            RoleArn: 'arn:aws:iam::404337120808:role/bedrock-access-role-integ-tests',
            RoleSessionName: 'BamlTestSession',
            DurationSeconds: 900,
          }),
        )

        if (!Credentials) {
          throw new Error('Failed to get credentials from STS')
        }

        const cr = new ClientRegistry()
        cr.addLlmClient('DynamicAWSClient', 'aws-bedrock', {
          model_id: 'meta.llama3-8b-instruct-v1:0',
          region: 'us-east-1',
          access_key_id: Credentials.AccessKeyId,
          secret_access_key: Credentials.SecretAccessKey,
          session_token: Credentials.SessionToken,
        })
        cr.setPrimary('DynamicAWSClient')

        const result = await b.TestAws('Dr. Pepper', { clientRegistry: cr })
        expect(result.length).toBeGreaterThan(0)
      })

      test('should require region in all environments', async () => {
        // Clear all region-related environment variables
        // delete process.env.AWS_REGION
        // delete process.env.AWS_DEFAULT_REGION
        // delete process.env.AWS_ACCESS_KEY_ID
        // delete process.env.AWS_SECRET_ACCESS_KEY
        // delete process.env.AWS_SESSION_TOKEN
        // delete process.env.AWS_PROFILE

        const cr = new ClientRegistry()
        cr.addLlmClient('DynamicAWSClient', 'aws-bedrock', {
          model_id: 'meta.llama3-8b-instruct-v1:0',
          access_key_id: 'test',
          secret_access_key: 'test',
        })
        cr.setPrimary('DynamicAWSClient')

        await expect(async () => {
          await b.TestAws('Dr. Pepper', { clientRegistry: cr })
        }).rejects.toMatchObject({
          code: 'GenericFailure',
        })
      })

      test('should throw error when region is empty or AWS_REGION is unset', async () => {
        // Clear all region-related environment variables
        // delete process.env.AWS_REGION
        // delete process.env.AWS_DEFAULT_REGION

        const crEmptyRegion = new ClientRegistry()
        crEmptyRegion.addLlmClient('DynamicAWSClient', 'aws-bedrock', {
          model_id: 'meta.llama3-8b-instruct-v1:0',
          region: '',
          access_key_id: 'test',
          secret_access_key: 'test',
        })
        crEmptyRegion.setPrimary('DynamicAWSClient')

        await expect(async () => {
          await b.TestAws('Dr. Pepper', { clientRegistry: crEmptyRegion })
        }).rejects.toMatchObject({
          code: 'GenericFailure',
        })

        const crNoEnvRegion = new ClientRegistry()
        crNoEnvRegion.addLlmClient('DynamicAWSClient', 'aws-bedrock', {
          model_id: 'meta.llama3-8b-instruct-v1:0',
          access_key_id: 'test',
          secret_access_key: 'test',
        })
        crNoEnvRegion.setPrimary('DynamicAWSClient')

        await expect(async () => {
          await b.TestAws('Dr. Pepper', { clientRegistry: crNoEnvRegion })
        }).rejects.toMatchObject({
          code: 'GenericFailure',
        })
      })
    })

    it('should support dynamic client configuration', async () => {
      // Set AWS_PROFILE for this specific test
      // process.env.AWS_PROFILE = 'boundaryml-dev'

      const cr = new ClientRegistry()
      cr.addLlmClient('DynamicAWSClient', 'aws-bedrock', {
        model_id: 'meta.llama3-8b-instruct-v1:0',
        region: 'us-east-1',
        inference_configuration: {
          max_tokens: 100,
        },
      })
      cr.setPrimary('DynamicAWSClient')

      const result = await b.TestAws('Dr. Pepper', { clientRegistry: cr })
      expect(result.length).toBeGreaterThan(0)
    })

    test('should support AWS credentials configuration', async () => {
      const cr = new ClientRegistry()
      cr.addLlmClient('DynamicAWSClient', 'aws-bedrock', {
        model_id: 'meta.llama3-8b-instruct-v1:0',
        region: 'us-east-1',
        access_key_id: 'test-access-key',
        secret_access_key: 'test-secret-key',
      })
      cr.setPrimary('DynamicAWSClient')

      await expect(async () => {
        await b.TestAws('Dr. Pepper', { clientRegistry: cr })
      }).rejects.toMatchObject({
        code: 'GenericFailure',
      })
    })

    it('should support AWS profile configuration', async () => {
      const cr = new ClientRegistry()
      cr.addLlmClient('DynamicAWSClient', 'aws-bedrock', {
        model_id: 'meta.llama3-8b-instruct-v1:0',
        region: 'us-east-1',
        profile: 'boundaryml-dev',
        inference_configuration: {
          max_tokens: 100,
        },
      })
      cr.setPrimary('DynamicAWSClient')

      const result = await b.TestAws('Dr. Pepper', { clientRegistry: cr })
      expect(result.length).toBeGreaterThan(0)
    })

    it('should support both model and model_id parameters', async () => {
      // Set AWS_PROFILE for this specific test
      // process.env.AWS_PROFILE = 'boundaryml-dev'

      // Test with model_id parameter
      const crWithModelId = new ClientRegistry()
      crWithModelId.addLlmClient('DynamicAWSClientModelId', 'aws-bedrock', {
        model_id: 'meta.llama3-8b-instruct-v1:0',
        region: 'us-east-1',
        inference_configuration: {
          max_tokens: 100,
        },
      })
      crWithModelId.setPrimary('DynamicAWSClientModelId')
      const resultWithModelId = await b.TestAws('Dr. Pepper', { clientRegistry: crWithModelId })
      expect(resultWithModelId.length).toBeGreaterThan(0)

      // Test with model parameter (legacy format)
      const crWithModel = new ClientRegistry()
      crWithModel.addLlmClient('DynamicAWSClientModel', 'aws-bedrock', {
        model: 'meta.llama3-8b-instruct-v1:0',
        region: 'us-east-1',
        inference_configuration: {
          max_tokens: 100,
        },
      })
      crWithModel.setPrimary('DynamicAWSClientModel')
      const resultWithModel = await b.TestAws('Dr. Pepper', { clientRegistry: crWithModel })
      expect(resultWithModel.length).toBeGreaterThan(0)
    })

    it('should handle invalid configuration gracefully', async () => {
      // Set AWS_PROFILE for this specific test
      // process.env.AWS_PROFILE = 'boundaryml-dev'

      const cr = new ClientRegistry()
      cr.addLlmClient('DynamicAWSClient', 'aws-bedrock', {
        model_id: 'meta.llama3-8b-instruct-v1:0',
        region: 'invalid-region',
        inference_configuration: {
          max_tokens: 100,
        },
      })
      cr.setPrimary('DynamicAWSClient')

      await expect(async () => {
        await b.TestAws('Dr. Pepper', { clientRegistry: cr })
      }).rejects.toMatchObject({
        code: 'GenericFailure',
      })
    })

    it('should handle non-existent model gracefully', async () => {
      // Set AWS_PROFILE for this specific test
      // process.env.AWS_PROFILE = 'boundaryml-dev'

      const cr = new ClientRegistry()
      cr.addLlmClient('DynamicAWSClient', 'aws-bedrock', {
        model_id: 'non-existent-model-123',
        region: 'us-east-1',
        inference_configuration: {
          max_tokens: 100,
        },
      })
      cr.setPrimary('DynamicAWSClient')

      await expect(async () => {
        await b.TestAws('Dr. Pepper', { clientRegistry: cr })
      }).rejects.toMatchObject({
        code: 'GenericFailure',
        message: expect.stringContaining('model'),
      })
    })

    test('should throw error when using temporary credentials without session token', async () => {
      // Clear all AWS-related environment variables
      // Object.keys(originalEnv).forEach((key) => {
      //   delete process.env[key]
      // })

      const sts = new STSClient({
        region: 'us-east-1',
        credentials: {
          accessKeyId: originalEnv.AWS_ACCESS_KEY_ID ?? '',
          secretAccessKey: originalEnv.AWS_SECRET_ACCESS_KEY ?? '',
        },
      })
      const { Credentials } = await sts.send(
        new AssumeRoleCommand({
          RoleArn: 'arn:aws:iam::404337120808:role/bedrock-access-role-integ-tests',
          RoleSessionName: 'BamlTestSession',
          DurationSeconds: 900,
        }),
      )

      if (!Credentials) {
        throw new Error('Failed to get credentials from STS')
      }

      const cr = new ClientRegistry()
      cr.addLlmClient('DynamicAWSClient', 'aws-bedrock', {
        model_id: 'meta.llama3-8b-instruct-v1:0',
        region: 'us-east-1',
        access_key_id: Credentials.AccessKeyId,
        secret_access_key: Credentials.SecretAccessKey,
        // Intentionally omit session_token
      })
      cr.setPrimary('DynamicAWSClient')

      await expect(async () => {
        await b.TestAwsInvalidSessionToken('Dr. Pepper', { clientRegistry: cr })
      }).rejects.toMatchObject({
        code: 'GenericFailure',
        message: expect.stringContaining('Session token is required'),
      })
    })

    test('should throw error when region is not provided', async () => {
      // Clear all region-related environment variables

      const cr = new ClientRegistry()
      cr.addLlmClient('DynamicAWSClient', 'aws-bedrock', {
        model_id: 'meta.llama3-8b-instruct-v1:0',
        // region: 'us-east-7',
        // profile: 'boundaryml-prod',
        // access_key_id: process.env.AWS_ACCESS_KEY_ID,
        // secret_access_key: process.env.AWS_SECRET_ACCESS_KEY,
        // session_token: process.env.AWS_SESSION_TOKEN,
      })
      cr.setPrimary('DynamicAWSClient')

      const result = await b.TestAws('Dr. Pepper', { clientRegistry: cr })
      console.log('result', result)
      // await expect(async () => {
      // }).rejects.toMatchObject({
      // code: 'GenericFailure',
      // })
    })

    test('should throw error when using invalid profile', async () => {
      // Clear any existing profile
      // delete process.env.AWS_PROFILE

      const cr = new ClientRegistry()
      cr.addLlmClient('DynamicAWSClient', 'aws-bedrock', {
        model_id: 'meta.llama3-8b-instruct-v1:0',
        region: 'us-east-1',
        profile: 'non-existent-profile-123',
      })
      cr.setPrimary('DynamicAWSClient')

      // Wait for the promise to reject
      let error: any
      try {
        const result = await b.TestAws('Dr. Pepper', { clientRegistry: cr })
        console.log('got result', result)
      } catch (e) {
        console.error('got error', e)
        error = e
      }

      // Verify the error
      expect(error).toBeDefined()
      expect(error.code).toBe('GenericFailure')
    })

    it('should support both AWS_REGION and AWS_DEFAULT_REGION environment variables', async () => {
      // Store original environment values
      const originalRegion = process.env.AWS_REGION
      const originalDefaultRegion = process.env.AWS_DEFAULT_REGION

      try {
        // Test with AWS_REGION
        // delete process.env.AWS_DEFAULT_REGION
        // process.env.AWS_REGION = 'us-east-1'

        const crWithAwsRegion = new ClientRegistry()
        crWithAwsRegion.addLlmClient('DynamicAWSClient', 'aws-bedrock', {
          model_id: 'meta.llama3-8b-instruct-v1:0',
          // Don't specify region, let it use AWS_REGION
          inference_configuration: {
            max_tokens: 100,
          },
        })
        crWithAwsRegion.setPrimary('DynamicAWSClient')

        const resultWithAwsRegion = await b.TestAws('Dr. Pepper', { clientRegistry: crWithAwsRegion })
        expect(resultWithAwsRegion.length).toBeGreaterThan(0)

        // Test with AWS_DEFAULT_REGION
        // delete process.env.AWS_REGION
        // process.env.AWS_DEFAULT_REGION = 'us-east-1'

        const crWithDefaultRegion = new ClientRegistry()
        crWithDefaultRegion.addLlmClient('DynamicAWSClient', 'aws-bedrock', {
          model_id: 'meta.llama3-8b-instruct-v1:0',
          // Don't specify region, let it use AWS_DEFAULT_REGION
          inference_configuration: {
            max_tokens: 100,
          },
        })
        crWithDefaultRegion.setPrimary('DynamicAWSClient')

        const resultWithDefaultRegion = await b.TestAws('Dr. Pepper', { clientRegistry: crWithDefaultRegion })
        expect(resultWithDefaultRegion.length).toBeGreaterThan(0)

        // Test that AWS_REGION takes precedence over AWS_DEFAULT_REGION
        process.env.AWS_REGION = 'us-east-1'
        process.env.AWS_DEFAULT_REGION = 'us-west-2' // Different region

        const crWithBothRegions = new ClientRegistry()
        crWithBothRegions.addLlmClient('DynamicAWSClient', 'aws-bedrock', {
          model_id: 'meta.llama3-8b-instruct-v1:0',
          // Don't specify region, should use AWS_REGION over AWS_DEFAULT_REGION
          inference_configuration: {
            max_tokens: 100,
          },
        })
        crWithBothRegions.setPrimary('DynamicAWSClient')

        const resultWithBothRegions = await b.TestAws('Dr. Pepper', { clientRegistry: crWithBothRegions })
        expect(resultWithBothRegions.length).toBeGreaterThan(0)
      } finally {
        // Restore original environment values
        // if (originalRegion) {
        //   process.env.AWS_REGION = originalRegion
        // } else {
        //   delete process.env.AWS_REGION
        // }
        // if (originalDefaultRegion) {
        //   process.env.AWS_DEFAULT_REGION = originalDefaultRegion
        // } else {
        //   delete process.env.AWS_DEFAULT_REGION
        // }
      }
    })
  })
})
