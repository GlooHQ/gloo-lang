const cors = require('cors')
const { createProxyMiddleware } = require('http-proxy-middleware')
const assert = require('assert')
const express = require('express');
const app = express()
const { SignatureV4 } = require('@smithy/signature-v4');
const { HttpRequest } = require('@aws-sdk/protocol-http');
const { Sha256 } = require('@aws-crypto/sha256-browser');
const { URL } = require('url');
require('dotenv').config()

app.use(cors())
app.use(express.json());

// From https://nodejs.org/api/url.html#url-strings-and-url-objects:
// ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
// │                                              href                                              │
// ├──────────┬──┬─────────────────────┬────────────────────────┬───────────────────────────┬───────┤
// │ protocol │  │        auth         │          host          │           path            │ hash  │
// │          │  │                     ├─────────────────┬──────┼──────────┬────────────────┤       │
// │          │  │                     │    hostname     │ port │ pathname │     search     │       │
// │          │  │                     │                 │      │          ├─┬──────────────┤       │
// │          │  │                     │                 │      │          │ │    query     │       │
// "  https:   //    user   :   pass   @ sub.example.com : 8080   /p/a/t/h  ?  query=string   #hash "
// │          │  │          │          │    hostname     │ port │          │                │       │
// │          │  │          │          ├─────────────────┴──────┤          │                │       │
// │ protocol │  │ username │ password │          host          │          │                │       │
// ├──────────┴──┼──────────┴──────────┼────────────────────────┤          │                │       │
// │   origin    │                     │         origin         │ pathname │     search     │ hash  │
// ├─────────────┴─────────────────────┴────────────────────────┴──────────┴────────────────┴───────┤
// │                                              href                                              │
// └────────────────────────────────────────────────────────────────────────────────────────────────┘

// These are the origins which we may "leak" our API keys to.
//
// We inject our API keys into requests to these domains so that promptfiddle users are not
// required to provide their own API keys, but we must make sure that these API keys cannot be
// leaked to third parties.
//
// Since all we do is blindly proxy requests from the WASM runtime, and promptfiddle users may
// override the base_url of any client, this allowlist guarantees that we only inject API keys
// in requests to these model providers.
const API_KEY_INJECTION_ALLOWED = {
  'https://api.openai.com': { Authorization: `Bearer ${process.env.OPENAI_API_KEY}` },
  'https://api.anthropic.com': { 'x-api-key': process.env.ANTHROPIC_API_KEY },
  'https://generativelanguage.googleapis.com': { 'x-goog-api-key': process.env.GOOGLE_API_KEY },
  'https://openrouter.ai': { Authorization: `Bearer ${process.env.OPENROUTER_API_KEY}` },
  'https://amazonaws.com': null, // AWS requests will be signed dynamically.
  'https://bedrock-runtime.us-east-1.amazonaws.com': null, // AWS requests will be signed dynamically.
}

// Consult sam@ before changing this.
for (const url of Object.keys(API_KEY_INJECTION_ALLOWED)) {
  assert(
    url === new URL(url).origin && new URL(url).protocol === 'https:',
    `Keys of API_KEY_INJECTION_ALLOWED must be HTTPS origins for model providers, got ${url}`,
  )
}

// Add this middleware before the proxy middleware
app.use(async (req, res, next) => {
  if (req.path.includes('converse')) {
    try {
      
      const awsAccessKeyId = process.env.AWS_ACCESS_KEY_ID;
      const awsSecretAccessKey = process.env.AWS_SECRET_ACCESS_KEY;
      if (!awsAccessKeyId || !awsSecretAccessKey) {
        console.error('Missing AWS credentials');
        return res.status(500).send('Missing AWS credentials');
      }
      console.log('awsAccessKeyId', awsAccessKeyId);
      console.log('awsSecretAccessKey', awsSecretAccessKey);

      const awsRegion = 'us-east-1';
      const signer = new SignatureV4({
        service: 'bedrock',
        region: awsRegion,
        credentials: {
          accessKeyId: awsAccessKeyId,
          secretAccessKey: awsSecretAccessKey,
        },
        applyChecksum: true,
        sha256: Sha256,
      });

      const originalUrl = new URL('https://bedrock-runtime.us-east-1.amazonaws.com');
      console.log("body", req.body);
      console.log("originalHeaders", req.headers);
       // Clear existing headers and only set host and content-type
       req.headers = {
        host: originalUrl.host,
        'content-type': 'application/json',
        'accept': 'application/json',
      };

      const request = new HttpRequest({
        method: req.method,
        protocol: originalUrl.protocol,
        path: req.path,
        headers: {
          ...req.headers,

          // host: originalUrl.hostname,
          // 'content-type': 'application/json'
        },
        hostname: originalUrl.hostname,

        body: JSON.stringify(req.body)
      });

     

      const signedRequest = await signer.sign(request, {
        signingDate: new Date(),
        
      });
      
      // Add signed headers to the request
      for (const [header, value] of Object.entries(signedRequest.headers)) {
        console.log('signedRequest', header, value);
        req.headers[header] = value;
      }
      req.headers['baml-original-url'] = req.headers['baml-original-url'] || 'https://bedrock-runtime.us-east-1.amazonaws.com';

      const curlCommand = `curl -X ${req.method} \\
      '${req.headers['baml-original-url']}${req.path}' \\
      -H 'Content-Type: application/json' \\
    ${Object.entries(req.headers)
      .map(([key, value]) => `  -H '${key}: ${value}' \\`)
      .join('\n')}
      -d '${JSON.stringify(req.body, null, 2)}'`;
    
          console.log('\nCURL Command for testing:');
          console.log(curlCommand);
    } catch (err) {
      console.error('Error signing AWS request:', err);
      return res.status(500).send('Error signing AWS request');
    }
  }
  try {
    next();
  } catch (err) {
    console.error('Error in middleware:', err);
    return res.status(500).send('Error in middleware');
  }
});

// Modify the proxy middleware to remove the AWS signing logic
app.use(
  createProxyMiddleware({
    changeOrigin: true,
    pathRewrite: (path, req) => {
      if (path.endsWith('/')) {
        return path.slice(0, -1);
      }
      return path;
    },
    router: (req) => {
      console.log('router', req.body);
      const originalUrl = req.headers['baml-original-url'];
      if (typeof originalUrl === 'string') {
        return originalUrl;
      } else {
        console.log('error with baml-original-url', originalUrl);
        // return 'https://bedrock-runtime.us-east-1.amazonaws.com';
       throw new Error('baml-original-url header is missing or invalid');
      }
    },
    logger: console,
    on: {
      proxyReq: (proxyReq, req, res) => {
        try {
          console.log('proxyReq headers', req.headers);
          console.log('proxyReq path', req.path);
          const bamlOriginalUrl = req.headers['baml-original-url']
          proxyReq.removeHeader('baml-original-url')

          console.log('bamlOriginalUrl', bamlOriginalUrl);
          if (bamlOriginalUrl === undefined) {
            return
          }
          const proxyOrigin = new URL(bamlOriginalUrl).origin
          // It is very important that we ONLY resolve against API_KEY_INJECTION_ALLOWED
          // by using the URL origin! (i.e. NOT using str.startsWith - the latter can still
          // leak API keys to malicious subdomains e.g. https://api.openai.com.evil.com)
          const headers = API_KEY_INJECTION_ALLOWED[proxyOrigin]
          if (headers === undefined || headers === null) {
            
            return
          }
          for (const [header, value] of Object.entries(headers)) {
            proxyReq.setHeader(header, value)
            }
          proxyReq.removeHeader('origin')
          proxyReq.removeHeader('baml-original-url')
        } catch (err) {
          // This is not console.warn because it's not important
          console.log('baml-original-url is not parsable', err)
        }
      },
      proxyRes: (proxyRes, req, res) => {
        console.log('proxyRes....')
        console.log('proxyRes', proxyRes.body)
        proxyRes.headers['Access-Control-Allow-Origin'] = '*'
      },
      error: (error) => {
        console.error('proxy error:', error);
      },
    },
  })
);
// Start web server on port 3000
app.listen(3000, () => {
  console.log('Server is listening on port 3000')
})
