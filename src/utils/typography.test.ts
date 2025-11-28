import { describe, it } from "node:test";
import assert from "node:assert";
import { typographyScale } from "./typography.ts";

describe("typographyScale", () => {
  const classic = { base: 1, ratio: 2, notes: 5 };

  it("returns the base size at position 0", () => {
    assert.strictEqual(typographyScale(0, classic), 1);
  });

  it("returns base × ratio at position n (one full interval)", () => {
    assert.strictEqual(typographyScale(5, classic), 2);
  });

  it("returns base × ratio² at position 2n", () => {
    assert.strictEqual(typographyScale(10, classic), 4);
  });

  it("handles negative indices for smaller sizes", () => {
    assert.strictEqual(typographyScale(-5, classic), 0.5);
  });

  it("computes intermediate positions correctly", () => {
    const result = typographyScale(1, classic);
    const expected = Math.pow(2, 1 / 5);
    assert.strictEqual(result, expected);
  });

  it("works with custom base", () => {
    const options = { base: 16, ratio: 2, notes: 5 };
    assert.strictEqual(typographyScale(0, options), 16);
    assert.strictEqual(typographyScale(5, options), 32);
  });

  it("works with custom ratio", () => {
    const options = { base: 1, ratio: 3, notes: 5 };
    assert.strictEqual(typographyScale(5, options), 3);
  });

  it("works with custom notes", () => {
    const options = { base: 1, ratio: 2, notes: 12 };
    assert.strictEqual(typographyScale(12, options), 2);
  });
});
