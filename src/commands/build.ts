import { dirname } from 'node:path';
import { writeFile, mkdir } from 'node:fs/promises';

import { transformFile } from '@swc/core';
import fg from 'fast-glob';
import rimraf from 'rimraf';

import { hasDependency } from '../utils/dependency-util';
import { logDebug, logError, logInfo } from '../utils/log-util';
import { packageJson } from '../utils/file-util';
import type { CommandAdder } from '../types/command-adder';
import { generateSwcConfig } from '../utils/swc-util';

const action = async (): Promise<void> => {
  logInfo('Building files found in src/');

  const typescript = hasDependency('typescript');
  const srcFiles = await fg(['src/**/*.{t,j}{s,sx}', '!src/**/*.d.ts']);
  const swcConfig = generateSwcConfig({
    isModule: packageJson.isModule,
    typescript,
  });

  rimraf.sync('dist');

  await Promise.all(
    srcFiles.map(async (file) => {
      try {
        const distFilePath = file.replace('src/', 'dist/').replace(/\.tsx?$/u, '.js');

        const transformed = await transformFile(file, swcConfig);

        await mkdir(dirname(distFilePath), { recursive: true });
        await writeFile(distFilePath, transformed.code, {
          encoding: 'utf8',
        });
      } catch (error) {
        logError(`Error building ${file}: ${error.message}`);
      }
    }),
  );

  logDebug('Built the following files:');

  srcFiles.forEach((file) => {
    logDebug(`    - ${file}`);
  });
};

const addBuildCommand: CommandAdder = (program) => {
  program
    .command('build')
    .description('build the TS and JS files in the `src` directory into the `dist` directory')
    .action(action);
};

export { addBuildCommand };
