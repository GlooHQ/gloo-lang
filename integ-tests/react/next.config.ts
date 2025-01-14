// import { withBaml } from "@boundaryml/baml-nextjs-plugin";
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  serverExternalPackages: ["@boundaryml/baml"],
  webpack: (config, { dev, isServer, webpack, nextRuntime }) => {
    config.module.rules.push({
      test: /\.node$/,
      use: [
        {
          loader: "nextjs-node-loader",
          options: {
            outputPath: config.output.path,
          },
        },
      ],
    });
    return config;
  },

};

export default nextConfig;
// export default withBaml()(nextConfig);
