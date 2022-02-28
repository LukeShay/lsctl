module.exports = {
  clearMocks: true,
  roots: ['<rootDir>'],
  testEnvironment: 'node',
  testMatch: ['**/*.{spec,test}.{t,j}s'],
  transform: {
    '^.+\\.(t|j)s$': '@swc/jest',
  },
};
