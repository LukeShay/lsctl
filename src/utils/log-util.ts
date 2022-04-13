import process from 'node:process';

import chalk from 'chalk';

const LogLevels = {
  DEBUG: 'debug',
  ERROR: 'error',
  INFO: 'info',
  WARN: 'warn',
} as const;

type LogLevel = typeof LogLevels[keyof typeof LogLevels];

const silent = process.argv.includes('--silent');
const debug = process.argv.includes('--debug');

const log = (level: LogLevel, message: string): void => {
  if (silent || (level === LogLevels.DEBUG && !debug)) {
    return;
  }

  // eslint-disable-next-line no-console
  console.log(message);
};

const logDebug = (message: string): void => {
  log(LogLevels.DEBUG, `${chalk.blue('[LSCTL]')} ${message}`);
};

const logInfo = (message: string): void => {
  log(LogLevels.INFO, `${chalk.green('[LSCTL]')} ${message}`);
};

const logWarn = (message: string): void => {
  log(LogLevels.WARN, `${chalk.yellow('[LSCTL]')} ${message}`);
};

const logError = (message: string): void => {
  log(LogLevels.ERROR, `${chalk.red('[LSCTL]')} ${message}`);
};
const logErrorAndExit = (message: string): void => {
  log(LogLevels.ERROR, `${chalk.red('[LSCTL]')} ${message}`);
};

export type { LogLevel };
export { log, logDebug, logInfo, logWarn, logError, LogLevels };
