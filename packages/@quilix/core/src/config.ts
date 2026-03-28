export interface QuilixConfig {
  plugins?: PluginConfig[];
  moduleFederation?: ModuleFederationConfig;
}

export interface PluginConfig {
  name: string;
  options?: Record<string, unknown>;
}

export interface ModuleFederationConfig {
  name: string;
  remotes?: Record<string, string>;
  exposes?: Record<string, string>;
  shared?: string[];
}

export function defineConfig(config: QuilixConfig): QuilixConfig {
  return config;
}

export interface ModuleFederationOptions {
  name: string;
  remotes?: Record<string, string>;
  exposes?: Record<string, string>;
  shared?: string[];
}

/**
 * Creates a Module Federation plugin configuration.
 */
export function moduleFederation(options: ModuleFederationOptions): PluginConfig {
  return {
    name: 'module-federation',
    options: options as unknown as Record<string, unknown>,
  };
}
