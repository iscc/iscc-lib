/**
 * Unit and negative tests for the encode_seq / decode_seq napi-rs bindings.
 *
 * Mirrors tests/test_seq.py: positive spot checks, order and duplicate
 * preservation, and one bad input per assertion for every rejection path.
 */

import { describe, it } from 'node:test';
import { deepStrictEqual, ok, strictEqual, throws } from 'node:assert';

import { encode_seq, decode_seq } from '../index.js';

const UNIT_32 = 'ISCC:GAAAAAAAAA';
const UNIT_32_HEX = '300000000000';
const UNIT_256 = 'ISCC:EAD2RASIYU5IKLENP2OFI4CHZGRWYQCSW2WKX3Y6FJGOCXSYNYGLGBI';
const UNIT_256_HEX = '2007a88248c53a852c8d7e9c547047c9a36c4052b6acabef1e2a4ce15e586e0cb305';

/**
 * Return the hex string of a Buffer or Uint8Array.
 */
function toHex(bytes) {
    return Buffer.from(bytes).toString('hex');
}

describe('encode_seq', () => {
    it('encodes a single 32-bit unit', () => {
        strictEqual(toHex(encode_seq([UNIT_32])), UNIT_32_HEX);
    });

    it('returns a Buffer', () => {
        ok(Buffer.isBuffer(encode_seq([UNIT_32])));
    });

    it('preserves order and duplicates', () => {
        strictEqual(
            toHex(encode_seq([UNIT_256, UNIT_32, UNIT_32])),
            UNIT_256_HEX + UNIT_32_HEX + UNIT_32_HEX,
        );
    });

    it('throws on an empty array', () => {
        throws(() => encode_seq([]), /at least one ISCC-UNIT/);
    });

    it('throws on an empty unit', () => {
        throws(() => encode_seq(['']), /must not be empty/);
    });

    it('throws on non-zero spare bits', () => {
        throws(() => encode_seq(['ISCC:GAAAAAAAAB']));
    });

    it('throws on a non-ASCII case-fold alias', () => {
        throws(() => encode_seq(['gaawdlqng6ſsdpf2']));
    });

    it('throws on a bare string instead of an array', () => {
        throws(() => encode_seq(UNIT_32));
    });
});

describe('decode_seq', () => {
    it('decodes canonical ISCC:-prefixed units in wire order', () => {
        const seq = Buffer.from(UNIT_32_HEX + UNIT_256_HEX, 'hex');
        deepStrictEqual(decode_seq(seq), [UNIT_32, UNIT_256]);
    });

    it('throws on empty bytes', () => {
        throws(() => decode_seq(Buffer.alloc(0)), /Empty ISCC-SEQ/);
    });

    it('throws on a truncated unit', () => {
        const seq = Buffer.from(UNIT_256_HEX, 'hex');
        throws(() => decode_seq(seq.subarray(0, seq.length - 1)));
    });
});
