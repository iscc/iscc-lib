/**
 * Unit tests for text utility, helper, and algorithm primitive napi-rs bindings.
 *
 * Tests the 12 non-gen functions: text_clean, text_remove_newlines, text_trim,
 * text_collapse, encode_base64, iscc_decompose, conformance_selftest,
 * sliding_window, alg_simhash, alg_minhash_256, alg_cdc_chunks,
 * soft_hash_video_v0.
 */

import { describe, it } from 'node:test';
import { strictEqual, deepStrictEqual, throws, ok } from 'node:assert';

import {
    text_clean,
    text_remove_newlines,
    text_trim,
    text_collapse,
    encode_base64,
    iscc_decompose,
    conformance_selftest,
    sliding_window,
    alg_simhash,
    alg_minhash_256,
    alg_cdc_chunks,
    soft_hash_video_v0,
} from '../index.js';

describe('text_clean', () => {
    it('applies NFKC normalization', () => {
        // Roman numeral Ⅷ (U+2167) normalizes to "VIII"
        strictEqual(text_clean('\u2167'), 'VIII');
    });

    it('normalizes CRLF to LF', () => {
        strictEqual(text_clean('a\r\nb'), 'a\nb');
    });

    it('removes control characters', () => {
        // \x00 is a control char that should be removed
        strictEqual(text_clean('a\x00b'), 'ab');
    });

    it('handles empty string', () => {
        strictEqual(text_clean(''), '');
    });

    it('strips leading and trailing whitespace', () => {
        strictEqual(text_clean('  hello  '), 'hello');
    });
});

describe('text_remove_newlines', () => {
    it('converts multi-line to single line', () => {
        strictEqual(text_remove_newlines('hello\nworld'), 'hello world');
    });

    it('collapses consecutive spaces', () => {
        strictEqual(text_remove_newlines('hello  \n  world'), 'hello world');
    });

    it('handles empty string', () => {
        strictEqual(text_remove_newlines(''), '');
    });
});

describe('text_trim', () => {
    it('truncates at byte boundary', () => {
        const result = text_trim('Hello World', 5);
        strictEqual(result, 'Hello');
    });

    it('does not split multi-byte characters', () => {
        // 'ä' is 2 bytes in UTF-8; trimming to 3 bytes should keep 'H' + 'ä' (3 bytes)
        const result = text_trim('Hällo', 3);
        strictEqual(result, 'Hä');
    });

    it('trims result whitespace', () => {
        const result = text_trim('ab cd', 3);
        strictEqual(result, 'ab');
    });

    it('handles empty string', () => {
        strictEqual(text_trim('', 10), '');
    });
});

describe('text_collapse', () => {
    it('lowercases and removes whitespace and punctuation', () => {
        strictEqual(text_collapse('Hello, World!'), 'helloworld');
    });

    it('handles empty string', () => {
        strictEqual(text_collapse(''), '');
    });

    it('removes accents via NFD decomposition', () => {
        // 'é' decomposes to 'e' + combining accent (mark), mark is removed
        const result = text_collapse('café');
        strictEqual(result, 'cafe');
    });
});

describe('encode_base64', () => {
    it('encodes known bytes to base64url without padding', () => {
        const buf = Buffer.from([0, 1, 2]);
        strictEqual(encode_base64(buf), 'AAEC');
    });

    it('encodes empty buffer', () => {
        strictEqual(encode_base64(Buffer.alloc(0)), '');
    });

    it('uses url-safe alphabet', () => {
        // Bytes that produce + and / in standard base64 should use - and _
        const buf = Buffer.from([251, 255, 254]);
        const result = encode_base64(buf);
        strictEqual(result.includes('+'), false);
        strictEqual(result.includes('/'), false);
    });
});

describe('iscc_decompose', () => {
    it('decomposes a single unit with prefix', () => {
        const result = iscc_decompose('ISCC:AAAYPXW445FTYNJ3');
        deepStrictEqual(result, ['AAAYPXW445FTYNJ3']);
    });

    it('decomposes a single unit without prefix', () => {
        const result = iscc_decompose('AAAYPXW445FTYNJ3');
        deepStrictEqual(result, ['AAAYPXW445FTYNJ3']);
    });

    it('throws on invalid input', () => {
        throws(() => iscc_decompose('NOT_VALID'), /./);
    });
});

describe('conformance_selftest', () => {
    it('returns true', () => {
        strictEqual(conformance_selftest(), true);
    });
});

describe('sliding_window', () => {
    it('produces expected n-grams', () => {
        const result = sliding_window('Hello', 3);
        deepStrictEqual(result, ['Hel', 'ell', 'llo']);
    });

    it('returns single element when string is shorter than width', () => {
        // Reference implementation returns the full string as a single window
        const result = sliding_window('Hi', 3);
        deepStrictEqual(result, ['Hi']);
    });

    it('returns single element when width equals string length', () => {
        const result = sliding_window('abc', 3);
        deepStrictEqual(result, ['abc']);
    });

    it('throws on width less than 2', () => {
        throws(() => sliding_window('hello', 1), /width must be 2 or bigger/);
    });

    it('throws on width 0', () => {
        throws(() => sliding_window('hello', 0), /width must be 2 or bigger/);
    });
});

// ── Algorithm primitives ─────────────────────────────────────────────────────

describe('alg_simhash', () => {
    it('returns 32 zero bytes for empty input', () => {
        const result = alg_simhash([]);
        strictEqual(Buffer.isBuffer(result), true);
        strictEqual(result.length, 32);
        ok(result.every((b) => b === 0));
    });

    it('returns the same bytes for a single digest', () => {
        const digest = Buffer.from([0xaa, 0xbb, 0xcc, 0xdd]);
        const result = alg_simhash([digest]);
        strictEqual(Buffer.isBuffer(result), true);
        deepStrictEqual([...result], [0xaa, 0xbb, 0xcc, 0xdd]);
    });

    it('returns output length matching input digest length', () => {
        const d1 = Buffer.alloc(8, 0xff);
        const d2 = Buffer.alloc(8, 0x00);
        const result = alg_simhash([d1, d2]);
        strictEqual(result.length, 8);
    });

    it('produces expected result for two complementary digests', () => {
        // All-ones and all-zeros: half frequency → all-ones (tie breaks to 1)
        const d1 = Buffer.alloc(4, 0xff);
        const d2 = Buffer.alloc(4, 0x00);
        const result = alg_simhash([d1, d2]);
        // With 2 inputs, threshold is 1; bit set when count >= 1, so ties go to 1
        deepStrictEqual([...result], [0xff, 0xff, 0xff, 0xff]);
    });
});

describe('alg_minhash_256', () => {
    it('returns a 32-byte Buffer', () => {
        const result = alg_minhash_256([1, 2, 3, 4, 5]);
        strictEqual(Buffer.isBuffer(result), true);
        strictEqual(result.length, 32);
    });

    it('produces deterministic output', () => {
        const features = [100, 200, 300, 400, 500];
        const r1 = alg_minhash_256(features);
        const r2 = alg_minhash_256(features);
        deepStrictEqual([...r1], [...r2]);
    });

    it('produces different output for different features', () => {
        const r1 = alg_minhash_256([1, 2, 3]);
        const r2 = alg_minhash_256([100, 200, 300]);
        // Extremely unlikely to be equal
        ok(!r1.equals(r2));
    });
});

describe('alg_cdc_chunks', () => {
    it('returns one empty chunk for empty input', () => {
        const result = alg_cdc_chunks(Buffer.alloc(0), false);
        strictEqual(result.length, 1);
        strictEqual(result[0].length, 0);
    });

    it('chunks concatenate back to original', () => {
        const data = Buffer.from('The quick brown fox jumps over the lazy dog. '.repeat(100));
        const chunks = alg_cdc_chunks(data, false);
        const reassembled = Buffer.concat(chunks);
        ok(data.equals(reassembled));
    });

    it('returns at least one chunk for small input', () => {
        const data = Buffer.from('hello');
        const chunks = alg_cdc_chunks(data, false);
        ok(chunks.length >= 1);
        ok(Buffer.concat(chunks).equals(data));
    });

    it('respects utf32 mode', () => {
        // utf32=true aligns cut points to 4-byte boundaries
        const data = Buffer.alloc(4096, 0x41); // 4096 'A' bytes
        const chunks = alg_cdc_chunks(data, true);
        for (const chunk of chunks) {
            strictEqual(chunk.length % 4, 0, 'chunk length should be 4-byte aligned');
        }
    });

    it('accepts custom avg_chunk_size', () => {
        const data = Buffer.from('x'.repeat(8192));
        const chunks = alg_cdc_chunks(data, false, 512);
        ok(Buffer.concat(chunks).equals(data));
    });
});

describe('soft_hash_video_v0', () => {
    it('returns a Buffer of length bits/8 for default bits', () => {
        // Single frame of 380 zeros (matching conformance vector structure)
        const frame = new Array(380).fill(0);
        const result = soft_hash_video_v0([frame]);
        strictEqual(Buffer.isBuffer(result), true);
        strictEqual(result.length, 8); // 64 bits / 8
    });

    it('returns correct length for custom bits', () => {
        const frame = new Array(380).fill(0);
        const result = soft_hash_video_v0([frame], 128);
        strictEqual(result.length, 16); // 128 bits / 8
    });

    it('throws on empty frame_sigs', () => {
        throws(() => soft_hash_video_v0([]), /must not be empty/);
    });

    it('produces deterministic output', () => {
        const frames = [
            new Array(380).fill(1),
            new Array(380).fill(2),
        ];
        const r1 = soft_hash_video_v0(frames);
        const r2 = soft_hash_video_v0(frames);
        deepStrictEqual([...r1], [...r2]);
    });
});
