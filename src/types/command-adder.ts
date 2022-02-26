import type { Command } from 'commander';

type CommandAdder = (program: Command) => void | Promise<void>;

export type { CommandAdder };
