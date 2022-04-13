import process from 'node:process';
import { resolve } from 'node:path';
import { readFileSync } from 'node:fs';

import type { PackageJson } from '../types/package-json';

const contents = JSON.parse(readFileSync(resolve(process.cwd(), 'package.json'), 'utf8')) as PackageJson;

const packageJson: PackageJson = {
  ...contents,
  isModule: contents.type === 'module',
};

export { packageJson };
