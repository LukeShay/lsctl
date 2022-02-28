import { Command } from 'commander';

import { addBuildCommand } from './commands/build';
import { addTestCommand } from './commands/test';

const run = async (): Promise<void> => {
  const program = new Command();

  program.name('lsctl').description('CLI simplify building server side Node.js applications').version('0.0.1');

  await addBuildCommand(program);
  await addTestCommand(program);

  program.parse();
};

export { run };
