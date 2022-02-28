import { writeFile, mkdir } from 'fs/promises';
import { dirname } from 'path';

import { transformFile } from '@swc/core';
import fg from 'fast-glob';
import rimraf from 'rimraf';
import type { Options } from '@swc/core';

import type { CommandAdder } from '../types/command-adder';
import { packageJson } from '../utils/file-util';
import { hasDependency } from '../utils/dependency-util';

const action = async (): Promise<void> => {
  const typescript = await hasDependency('typescript');
  const srcFiles = await fg(['src/**/*.{t,j}s', '!src/**/*.d.ts']);
  const swcConfig: Options = {
    sourceMaps: 'inline',
    minify: true,
    jsc: {
      target: packageJson.type === 'module' ? 'es2020' : 'es5',
      parser: {
        syntax: typescript ? 'typescript' : 'ecmascript',
      },
    },
    module: {
      type: packageJson.type === 'module' ? 'es6' : 'commonjs',
      strict: true,
      strictMode: true,
    },
  };

  console.log(`SWC Config: ${JSON.stringify(swcConfig, null, 2)}`);

  rimraf.sync('dist');

  await Promise.all(
    srcFiles.map(async (file) => {
      const distFilePath = file.replace('src/', 'dist/').replace(/\.ts$/u, '.js');

      const transformed = await transformFile(file, swcConfig);

      await mkdir(dirname(distFilePath), { recursive: true });
      await writeFile(distFilePath, transformed.code, {
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
