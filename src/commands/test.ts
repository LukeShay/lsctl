import process from 'node:process';

import jest from 'jest';

import type { CommandAdder } from '../types/command-adder';

const jestConfig = {
  clearMocks: true,
  roots: ['<rootDir>/test'],
  testEnvironment: 'node',
  testMatch: ['**/*.{spec,test}.{t,j}s'],
  transform: {
    '^.+\\.(t|j)s$': '@swc/jest',
  },
};

const action = async (): Promise<void> => {
  const argv = process.argv.slice(2);

  argv.push('--config', JSON.stringify(jestConfig));

  const [, ...argsToPassToJestCli] = argv;

  await jest.run(argsToPassToJestCli);
};

const addTestCommand: CommandAdder = (program): void => {
  program.command('test').description('Runs jest').action(action);
};

export { addTestCommand };
