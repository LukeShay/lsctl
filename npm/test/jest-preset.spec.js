describe('jest-preset', () => {
    let jestPreset;

    beforeEach(async () => {
        jestPreset = await import('../jest-preset.js');
    });

    test('should match snapshot', () => {
        expect(JSON.stringify(jestPreset)).toMatchInlineSnapshot(
            '"{\\"cacheDirectory\\":\\".jest-cache\\",\\"clearMocks\\":true,\\"collectCoverage\\":true,\\"coverageDirectory\\":\\".jest-coverage\\",\\"coverageReporters\\":[\\"html\\",\\"lcov\\",\\"text\\",\\"cobertura\\"],\\"roots\\":[\\"<rootDir>\\"],\\"testEnvironment\\":\\"node\\",\\"testMatch\\":[\\"**/*.{spec,test}.{ts,tsx,js,jsx}\\"],\\"transform\\":{\\"^.+\\\\\\\\.(t|j)sx?$\\":\\"@swc/jest\\"},\\"default\\":{\\"cacheDirectory\\":\\".jest-cache\\",\\"clearMocks\\":true,\\"collectCoverage\\":true,\\"coverageDirectory\\":\\".jest-coverage\\",\\"coverageReporters\\":[\\"html\\",\\"lcov\\",\\"text\\",\\"cobertura\\"],\\"roots\\":[\\"<rootDir>\\"],\\"testEnvironment\\":\\"node\\",\\"testMatch\\":[\\"**/*.{spec,test}.{ts,tsx,js,jsx}\\"],\\"transform\\":{\\"^.+\\\\\\\\.(t|j)sx?$\\":\\"@swc/jest\\"}}}"'
        );
    });
});
