import {readFile} from 'node:fs/promises';

describe('jest-preset', () => {
    let swcrc;

    beforeEach(async () => {
        swcrc = JSON.parse(await readFile('.swcrc', 'utf8'));
    });

    test('should match snapshot', () => {
        expect(JSON.stringify(swcrc)).toMatchInlineSnapshot(
            `"{\\"jsc\\":{\\"parser\\":{\\"syntax\\":\\"ecmascript\\"},\\"target\\":\\"es5\\"},\\"minify\\":true,\\"module\\":{\\"strict\\":true,\\"strictMode\\":true,\\"type\\":\\"commonjs\\"},\\"sourceMaps\\":\\"inline\\"}"`
        );
    });
});
