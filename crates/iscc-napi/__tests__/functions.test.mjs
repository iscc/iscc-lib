/**
 * Unit tests for text utility and helper napi-rs bindings.
 *
 * Tests the 8 non-gen functions: text_clean, text_remove_newlines, text_trim,
 * text_collapse, encode_base64, iscc_decompose, conformance_selftest,
 * sliding_window.
 */

import { describe, it } from 'node:test';
import { strictEqual, deepStrictEqual, throws } from 'node:assert';

import {
    text_clean,
    text_remove_newlines,
    text_trim,
    text_collapse,
    encode_base64,
    iscc_decompose,
    conformance_selftest,
    sliding_window,
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
