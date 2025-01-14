import type { Configuration } from 'webpack';

function getNextJsVersion(): string | null {
  try {
    // Try to find Next.js in the project's dependencies first
    const projectNextPath = require.resolve('next/package.json', {
      paths: [process.cwd()]
    });
    const nextPackageJson = require(projectNextPath);
    return nextPackageJson.version || null;
  } catch (error) {
    try {
      // Fallback to checking in the plugin's dependencies
      const nextPackageJson = require('next/package.json');
      return nextPackageJson.version || null;
    } catch (error) {
      console.warn('Warning: Could not determine Next.js version, defaulting to latest config');
      return null;
    }
  }
}

type GenericNextConfig = {
  experimental?: {
    serverComponentsExternalPackages?: string[];
  };
  serverExternalPackages?: string[];
  webpack?: (config: Configuration, context: any) => Configuration;
};

export interface BamlNextConfig {
  webpack?: (config: Configuration, context: any) => Configuration;
}

export function withBaml(bamlConfig: BamlNextConfig = {}) {
  return function withBamlConfig(nextConfig: GenericNextConfig = {}): GenericNextConfig {
    const nextVersion = getNextJsVersion();
    // Default to new config (>= 14) if version can't be determined
    const majorVersion = nextVersion ? parseInt(nextVersion.split('.')[0], 10) : 14;
    const useNewConfig = majorVersion >= 14;

    return {
      ...nextConfig,
      ...(useNewConfig
        ? {
            serverExternalPackages: [
              ...((nextConfig as any)?.serverExternalPackages || []),
              "@boundaryml/baml"
            ],
          }
        : {
            experimental: {
              ...nextConfig.experimental,
              serverComponentsExternalPackages: [
                ...((nextConfig.experimental as any)?.serverComponentsExternalPackages || []),
                "@boundaryml/baml"
              ],
            },
          }),
      webpack: (config: Configuration, context: any) => {
        if (typeof nextConfig.webpack === 'function') {
          config = nextConfig.webpack(config, context);
        }
        if (typeof bamlConfig.webpack === 'function') {
          config = bamlConfig.webpack(config, context);
        }

        config.module = config.module || {};
        config.module.rules = config.module.rules || [];
        config.module.rules.push({
          test: /\.node$/,
          use: [
            {
              loader: "nextjs-node-loader",
              options: {
                outputPath: config.output?.path,
              },
            },
          ],
        });

        return config;
      },
    };
  };
}
