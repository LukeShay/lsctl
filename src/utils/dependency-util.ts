import { packageJson } from './file-util.js';

const hasDependency = async (dependency: string): Promise<boolean> => {
  return !!packageJson.dependencies?.[dependency] || !!packageJson.devDependencies?.[dependency];
};

export { hasDependency };
