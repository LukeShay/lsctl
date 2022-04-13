type PackageJson = {
  dependencies?: Record<string, string | undefined>;
  description?: string;
  devDependencies?: Record<string, string | undefined>;
  isModule: boolean;
  name?: string;
  type?: 'commonjs' | 'module';
  version?: string;
};

export type { PackageJson };
