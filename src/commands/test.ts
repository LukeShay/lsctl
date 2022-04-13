import process from 'node:process';

// eslint-disable-next-line jest/no-jest-import
import jest from 'jest';

import type { CommandAdder } from '../types/command-adder';

const jestConfig = {
  clearMocks: true,
  roots: ['<rootDir>/test'],
  testEnvironment: 'node',
  testMatch: ['**/*.{spec,test}.{t,j}s'],
  transform: {
    '^.+\\.(t|j)sx?$': '@swc-node/jest',
  },
};

const action = async (): Promise<void> => {
  const argv = process.argv.slice(2);

  argv.push('--config', JSON.stringify(jestConfig));

  const [, ...argsToPassToJestCli] = argv;

  await jest.run(argsToPassToJestCli);
};

const addTestCommand: CommandAdder = (program) => {
  program.command('test').description('runs jest').action(action);
};

export { addTestCommand };
