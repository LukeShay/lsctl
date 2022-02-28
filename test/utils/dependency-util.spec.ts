import { hasDependency } from '../../src/utils/dependency-util';
import { chance } from '../test-helpers/chance';

describe('dependency util', () => {
  describe('hasDependency', () => {
    test('should return true when there is a dev dependency', () => {
      expect(hasDependency('eslint')).toBe(true);
    });

    test('should return true when there is a dependency', () => {
      expect(hasDependency('@swc/core')).toBe(true);
    });

    test("should return false when there isn't a dependency or dev dependency", () => {
      expect(hasDependency(chance.sentence())).toBe(false);
    });
  });
});
