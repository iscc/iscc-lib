/**
 * Unit tests for the experimental gen_iscc_id_v1 napi-rs binding.
 *
 * Covers the golden vector, validation-error cases (timestamp / hub_id / realm
 * overflow), and an iscc_decode round-trip of the minted ISCC-IDv1 code.
 */

import { describe, it } from 'node:test';
import { strictEqual, throws } from 'node:assert';

import { gen_iscc_id_v1, iscc_decode } from '../index.js';

describe('gen_iscc_id_v1', () => {
    it('produces the golden vector', () => {
        strictEqual(gen_iscc_id_v1(1751831876325218, 1, 0), 'ISCC:MAIGHFECJMOPMIAB');
    });

    it('accepts realm 1 (operational)', () => {
        strictEqual(typeof gen_iscc_id_v1(1751831876325218, 1, 1), 'string');
    });

    it('throws on timestamp >= 2^52', () => {
        throws(() => gen_iscc_id_v1(2 ** 52, 1, 0));
    });

    it('throws on hub_id >= 2^12', () => {
        throws(() => gen_iscc_id_v1(1751831876325218, 4096, 0));
    });

    it('throws on invalid realm', () => {
        throws(() => gen_iscc_id_v1(1751831876325218, 1, 2));
    });

    it('throws on negative inputs', () => {
        throws(() => gen_iscc_id_v1(-1, 1, 0));
        throws(() => gen_iscc_id_v1(1751831876325218, -1, 0));
        throws(() => gen_iscc_id_v1(1751831876325218, 1, -1));
    });

    it('throws on non-integral inputs', () => {
        throws(() => gen_iscc_id_v1(0.9, 1, 0));
        throws(() => gen_iscc_id_v1(1751831876325218, 0.9, 0));
        throws(() => gen_iscc_id_v1(1751831876325218, 1, 0.9));
    });

    it('throws on non-finite inputs', () => {
        throws(() => gen_iscc_id_v1(NaN, 1, 0));
        throws(() => gen_iscc_id_v1(Infinity, 1, 0));
        throws(() => gen_iscc_id_v1(1751831876325218, NaN, 0));
        throws(() => gen_iscc_id_v1(1751831876325218, 1, NaN));
    });

    it('throws on out-of-range hub_id via a large float', () => {
        throws(() => gen_iscc_id_v1(1751831876325218, 2 ** 32, 0));
    });

    it('round-trips through iscc_decode', () => {
        const decoded = iscc_decode(gen_iscc_id_v1(1751831876325218, 1, 0));
        strictEqual(decoded.maintype, 6);
        strictEqual(decoded.version, 1);
        strictEqual(decoded.subtype, 0);
    });
});
