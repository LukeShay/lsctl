import type { Options } from '@swc/core';

const generateSwcConfig = ({ typescript, isModule }: { typescript?: boolean; isModule?: boolean }): Options => ({
  jsc: {
    parser: {
      syntax: typescript ? 'typescript' : 'ecmascript',
    },
    target: isModule ? 'es2020' : 'es5',
  },
  minify: true,
  module: {
    strict: true,
    strictMode: true,
    type: isModule ? 'es6' : 'commonjs',
  },
  sourceMaps: 'inline',
});

export { generateSwcConfig };
