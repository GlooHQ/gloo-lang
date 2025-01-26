const assert = require('assert')
const cors = require('cors');
const express = require('express');
const http2 = require('http2');
const { URL } = require('url');
require('dotenv').config();

const app = express();
app.use(cors());

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
  'https://generativelanguage.googleapis.com': { Authorization: `Bearer ${process.env.GOOGLE_API_KEY}` },
  'https://openrouter.ai': { Authorization: `Bearer ${process.env.OPENROUTER_API_KEY}` },
};

// Consult sam@ before changing this.
for (const url of Object.keys(API_KEY_INJECTION_ALLOWED)) {
  assert(
    url === new URL(url).origin && new URL(url).protocol === 'https:',
    `Keys of API_KEY_INJECTION_ALLOWED must be HTTPS origins for model providers, got ${url}`,
  )
}

// Middleware to handle proxy requests.
app.use(async (req, res) => {
  const originalUrl = req.headers['baml-original-url'];
  if (!originalUrl) {
    res.status(400).send('Missing baml-original-url header');
    return;
  }

  try {
    // Parse the original URL and append the request path.
    const targetUrl = new URL(originalUrl);

    const removeTrailingSlash = req.path.endsWith('/')
      ? req.path.slice(0, -1) // Remove trailing slash
      : req.path;

    targetUrl.pathname = `${targetUrl.pathname}${removeTrailingSlash}`;

    const proxyReqHeaders = { ...req.headers }; // Clone incoming headers
    delete proxyReqHeaders.host; // Remove host header for upstream requests
    delete proxyReqHeaders.origin; // Remove origin header for upstream requests

    // It is very important that we ONLY resolve against API_KEY_INJECTION_ALLOWED
    // by using the URL origin! (i.e. NOT using str.startsWith - the latter can still
    // leak API keys to malicious subdomains e.g. https://api.openai.com.evil.com)
    const allowedHeaders = API_KEY_INJECTION_ALLOWED[targetUrl.origin];

    if (allowedHeaders) {
      // Override headers.
      for ([header, value] of Object.entries(allowedHeaders)) {
        proxyReqHeaders[header.toLowerCase()] = value;
      }
    }

    // Establish HTTP/2 connection
    const client = http2.connect(targetUrl.origin);

    const proxyReq = client.request({
      ':method': req.method,
      ':path': `${targetUrl.pathname}${targetUrl.search}`,
      ...proxyReqHeaders,
    });

    // Pipe the request body to the upstream server.
    req.pipe(proxyReq);

    // Handle the response from the upstream server.
    proxyReq.on('response', (headers) => {
      // Set response headers
      for (const [key, value] of Object.entries(headers)) {
        if (key.startsWith(':')) continue; // Skip pseudo-headers
        res.setHeader(key, value);
      }
      res.statusCode = headers[':status'];
    });

    proxyReq.on('data', (chunk) => {
      res.write(chunk); // Forward the data to the client
    });

    proxyReq.on('end', () => {
      res.end(); // End the response
      client.close(); // Close the HTTP/2 connection
    });

    proxyReq.on('error', (err) => {
      console.error('Proxy request error:', err);
      res.status(500).send('Internal Server Error');
      client.close();
    });
  } catch (err) {
    console.error('Proxy error:', err);
    res.status(500).send('Failed to process request');
  }
});

// Start the server
app.listen(3000, () => {
  console.log('Server is listening on port 3000');
});
