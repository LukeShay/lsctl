module.exports = {
    cacheDirectory: '.jest-cache',
    clearMocks: true,
    collectCoverage: true,
    coverageDirectory: '.jest-coverage',
    coverageReporters: ['html', 'lcov', 'text', 'cobertura'],
    testEnvironment: 'node',
    testMatch: ['**/*.{spec,test}.{t,j}sx?'],
    transform: {
        '^.+\\.(t|j)s$': '@swc/jest',
    },
};
