import { withBaml } from "@boundaryml/baml-nextjs-plugin";
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  serverExternalPackages: ["@boundaryml/baml", "@boundaryml/baml-darwin-arm64"],
  webpack: (config, { dev, isServer, webpack, nextRuntime }) => {
    // Handle native modules
    if (isServer) {
      // Externalize the native module
      config.externals = [
        ...(Array.isArray(config.externals) ? config.externals : []),
        '@boundaryml/baml-darwin-arm64',
        '@boundaryml/baml',
      ];
    }

    if (!isServer) {
      // Prevent client-side loading of native modules
      config.resolve.fallback = {
        ...config.resolve?.fallback,
        '@boundaryml/baml-darwin-arm64': false,
        '@boundaryml/baml': false,
      };
    }

    config.module = config.module || { rules: [] };
    config.module.rules.push({
      test: /\.node$/,
      use: [
        {
          loader: "nextjs-node-loader",
          options: {
            flags: 'binding',
            outputPath: config.output?.path,
          },
        },
      ],
    });

    // Ensure native addons are processed correctly
    config.resolve = config.resolve || {};
    config.resolve.alias = {
      ...config.resolve.alias,
      "@boundaryml/baml-darwin-arm64": require.resolve("@boundaryml/baml-darwin-arm64"),
    };

    return config;
  },
};

export default withBaml()(nextConfig);
