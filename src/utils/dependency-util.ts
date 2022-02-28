import { packageJson } from './file-util';

const hasDependency = (dependency: string): boolean =>
  Boolean(packageJson.dependencies?.[dependency]) || Boolean(packageJson.devDependencies?.[dependency]);

export { hasDependency };
