import process from 'node:process';
import { resolve } from 'node:path';
import { readFileSync } from 'node:fs';

import type { PackageJson } from '../types/package-json.js';

const packageJson: PackageJson = JSON.parse(
  readFileSync(resolve(process.cwd(), 'package.json'), 'utf8'),
) as PackageJson;

export { packageJson };
