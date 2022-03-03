import { dirname } from 'path';
import { writeFileSync, mkdirSync } from 'fs';

import { transformFile } from '@swc/core';
import fg from 'fast-glob';
import rimraf from 'rimraf';
import type { Options } from '@swc/core';

import { hasDependency } from '../utils/dependency-util';
import { packageJson } from '../utils/file-util';
import type { CommandAdder } from '../types/command-adder';

const action = async (): Promise<void> => {
  const typescript = hasDependency('typescript');
  const srcFiles = await fg(['src/**/*.{t,j}s', '!src/**/*.d.ts']);
  const swcConfig: Options = {
    jsc: {
      parser: {
        syntax: typescript ? 'typescript' : 'ecmascript',
      },
      target: packageJson.type === 'module' ? 'es2020' : 'es5',
    },
    minify: true,
    module: {
      strict: true,
      strictMode: true,
      type: packageJson.type === 'module' ? 'es6' : 'commonjs',
    },
    sourceMaps: 'inline',
  };

  rimraf.sync('dist');

  await Promise.all(
    srcFiles.map(async (file) => {
      const distFilePath = file.replace('src/', 'dist/').replace(/\.ts$/u, '.js');

      const transformed = await transformFile(file, swcConfig);

      mkdirSync(dirname(distFilePath), { recursive: true });
      writeFileSync(distFilePath, transformed.code, {
        encoding: 'utf8',
      });
    }),
  );
};

const addBuildCommand: CommandAdder = (program): void => {
  program
    .command('build')
    .description('Build the TS and JS files in the `src` directory into the `dist` directory')
    .action(action);
};

export { addBuildCommand };
