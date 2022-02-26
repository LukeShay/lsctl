type PackageJson = {
  name?: string;
  version?: string;
  description?: string;
  type?: 'module' | 'commonjs';
  dependencies?: Record<string, string | undefined>;
  devDependencies?: Record<string, string | undefined>;
};

export type { PackageJson };
