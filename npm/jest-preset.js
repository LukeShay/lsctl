module.exports = {
    cacheDirectory: '.jest-cache',
    clearMocks: true,
    collectCoverage: true,
    coverageDirectory: '.jest-coverage',
    coverageReporters: ['html', 'lcov', 'text', 'cobertura'],
    roots: ['<rootDir>'],
    testEnvironment: 'node',
    testMatch: ['**/*.{spec,test}.{ts,tsx,js,jsx}'],
    transform: {
        '^.+\\.(t|j)sx?$': '@swc/jest',
    },
};
