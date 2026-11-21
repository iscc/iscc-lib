/**
 * Shared edge-case vectors (crates/iscc-lib/tests/data-edge.json) run against the napi bindings.
 *
 * Every vector runs, `except.nodejs` is the only deviation, and a function section
 * this harness does not handle fails. A rejection is asserted on the raw call alone;
 * the result is converted to its fixture form only after a successful call.
 */

import { readFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, it } from 'node:test';
import { deepStrictEqual, ok, strictEqual } from 'node:assert';

import {
    iscc_decode,
    encode_seq,
    gen_text_code_v0,
    gen_mixed_code_v0,
    gen_iscc_code_v0,
    alg_simhash,
    gen_meta_code_v0,
    gen_video_code_v0,
    alg_cdc_chunks,
} from '../index.js';

const __dirname = dirname(fileURLToPath(import.meta.url));
const dataPath = join(__dirname, '..', '..', 'iscc-lib', 'tests', 'data-edge.json');

/** Surface id of this harness in the `except` map. */
const SURFACE = 'nodejs';

/** Encode bytes as a `bytes:<hex>` fixture value. */
function bytesValue(data) {
    return `bytes:${Buffer.from(data).toString('hex')}`;
}

/** Decode a `bytes:<hex>` fixture value into a Buffer; throw on any other value. */
function decodeBytes(value) {
    const match = /^bytes:([0-9a-f]*)$/.exec(value);
    if (!match) throw new TypeError(`expected a bytes:<hex> input, got ${JSON.stringify(value)}`);
    return Buffer.from(match[1], 'hex');
}

/** Decode the `alg_simhash` digest list from `bytes:<hex>` strings into Buffers. */
function prepareSimhash([digests, ...rest]) {
    return [digests.map(decodeBytes), ...rest];
}

/** Decode the `alg_cdc_chunks` data from a `bytes:<hex>` string into a Buffer. */
function prepareCdc([data, ...rest]) {
    return [decodeBytes(data), ...rest];
}

/** Convert `alg_cdc_chunks` chunks to a list of `bytes:<hex>` values. */
function convertChunks(chunks) {
    return chunks.map(bytesValue);
}

/**
 * Prepare `gen_meta_code_v0` inputs: an empty description becomes `undefined` and a
 * meta object is serialized to the JSON string the binding takes.
 */
function prepareMeta([name, description, meta, ...rest]) {
    const metaArg = typeof meta === 'string' ? meta : JSON.stringify(meta);
    return [name, description || undefined, metaArg, ...rest];
}

/** Return a vector's raw JSON inputs unchanged. */
function identity(inputs) {
    return inputs;
}

/** Convert an `iscc_decode` result to `[mt, st, vs, length, "bytes:<hex>"]`. */
function convertDecode(r) {
    return [r.maintype, r.subtype, r.version, r.length, bytesValue(r.digest)];
}

/** Convert a bare ISCC string result (gen_* functions) to an `{iscc}` object. */
function convertIscc(iscc) {
    return { iscc };
}

/**
 * Section name -> [function, converter, input preparer]. The raw call is always
 * `fn(...prepare(inputs))`; preparing and converting stay outside the raw call so
 * neither error ever counts as a rejection. `bytes:` inputs are decoded only by a
 * section's preparer.
 */
const SECTIONS = {
    iscc_decode: [iscc_decode, convertDecode, identity],
    encode_seq: [encode_seq, bytesValue, identity],
    gen_text_code_v0: [gen_text_code_v0, convertIscc, identity],
    gen_mixed_code_v0: [gen_mixed_code_v0, convertIscc, identity],
    gen_iscc_code_v0: [gen_iscc_code_v0, convertIscc, identity],
    alg_simhash: [alg_simhash, bytesValue, prepareSimhash],
    gen_meta_code_v0: [gen_meta_code_v0, convertIscc, prepareMeta],
    gen_video_code_v0: [gen_video_code_v0, convertIscc, identity],
    alg_cdc_chunks: [alg_cdc_chunks, convertChunks, prepareCdc],
};

/** Load and parse data-edge.json. */
function loadDoc() {
    return JSON.parse(readFileSync(dataPath, 'utf-8'));
}

/**
 * Section name -> expected output keys its napi result does not carry, which are
 * therefore not compared (an absent section compares every key). The gen functions
 * here return a bare ISCC string, so text drops `characters`, mixed drops `parts` and
 * meta drops `name`, `description`, `meta` and `metahash`.
 */
const UNCOMPARED = {
    gen_text_code_v0: ['characters'],
    gen_mixed_code_v0: ['parts'],
    gen_meta_code_v0: ['name', 'description', 'meta', 'metahash'],
};

/**
 * Compare an actual output with the expected one and return a failure, or null.
 * For an object, every expected key outside the section's static uncompared set must
 * be present in the result and equal; a missing key fails.
 */
function compare(section, expected, actual) {
    if (expected === null || typeof expected !== 'object' || Array.isArray(expected)) {
        const same = JSON.stringify(expected) === JSON.stringify(actual);
        return same ? null : `expected ${JSON.stringify(expected)}, got ${JSON.stringify(actual)}`;
    }
    if (actual === null || typeof actual !== 'object' || Array.isArray(actual)) {
        return `expected an object, got ${JSON.stringify(actual)}`;
    }
    const uncompared = UNCOMPARED[section] ?? [];
    for (const key of Object.keys(expected)) {
        if (uncompared.includes(key)) continue;
        if (!Object.hasOwn(actual, key)) return `missing field ${key} in ${JSON.stringify(actual)}`;
        if (JSON.stringify(expected[key]) !== JSON.stringify(actual[key])) {
            return `field ${key}: expected ${JSON.stringify(expected[key])}, got ${JSON.stringify(actual[key])}`;
        }
    }
    return null;
}

/** Run the raw call and return `{ error }` or `{ result }`. */
function rawCall(fn, inputs) {
    try {
        return { result: fn(...inputs) };
    } catch (error) {
        if (!(error instanceof Error)) throw error;
        return { error };
    }
}

/** Check one vector against the surface's effective expectation; return a failure or null. */
function checkVector(section, [fn, convert, prepare], vector) {
    const exception = vector.except?.[SURFACE];
    if (exception === 'skip') return null;
    const expectError = exception === 'error' || 'error' in vector;
    const { result, error } = rawCall(fn, prepare(vector.inputs));
    if (expectError) {
        return error ? null : `expected an error, got ${JSON.stringify(convert(result))}`;
    }
    if (error) return `expected an output, got error: ${error.message}`;
    if (!('outputs' in vector)) return 'vector has neither outputs nor error';
    return compare(section, vector.outputs, convert(result));
}

/** Run every vector of a parsed document; return one `section/vector: reason` per failure. */
function checkDoc(doc) {
    const failures = [];
    for (const [section, vectors] of Object.entries(doc)) {
        const entry = SECTIONS[section];
        if (!entry) {
            failures.push(`${section}: unknown section`);
            continue;
        }
        for (const [name, vector] of Object.entries(vectors)) {
            const reason = checkVector(section, entry, vector);
            if (reason) failures.push(`${section}/${name}: ${reason}`);
        }
    }
    return failures;
}

describe('data-edge.json', () => {
    it('passes every vector', () => {
        const doc = loadDoc();
        for (const section of Object.keys(SECTIONS)) {
            ok(Object.keys(doc[section] ?? {}).length > 0, `section ${section} has no vectors`);
        }
        deepStrictEqual(checkDoc(doc), []);
    });

    it('fails an unknown section', () => {
        const doc = {
            gen_unknown_v0: { test_0001_x: { inputs: ['x'], outputs: 'x', note: 'n' } },
        };
        const failures = checkDoc(doc);
        strictEqual(failures.length, 1);
        ok(failures[0].includes('unknown section'), failures[0]);
    });

    it('fails an expected error that succeeds', () => {
        // Valid inputs marked `error` prove each rejection-only caller reaches the library.
        const doc = {
            gen_mixed_code_v0: {
                test_0001_x: { inputs: [['EUA6GIKXN42IQV3S', 'EIAUKMOUIOYZCKA5'], 64], error: 'e', note: 'n' },
            },
            gen_iscc_code_v0: {
                test_0001_x: {
                    inputs: [['GABVVC5DMJJGYKZ4ZBYVNYABFFYXG', 'IADWIK7A7JTUAQ2D6QARX7OBEIK3OOUAM42LOBLCZ4ZOGDLRHMDL6TQ']],
                    error: 'e',
                    note: 'n',
                },
            },
        };
        const failures = checkDoc(doc);
        strictEqual(failures.length, 2, failures.join('\n'));
        ok(failures.every((f) => f.includes('expected an error')), failures.join('\n'));
    });

    it('fails an output mismatch', () => {
        const doc = {
            encode_seq: {
                test_0001_x: { inputs: [['ISCC:GAAAAAAAAA']], outputs: 'bytes:300000000001', note: 'n' },
            },
        };
        const failures = checkDoc(doc);
        strictEqual(failures.length, 1);
        ok(failures[0].includes('expected "bytes:300000000001"'), failures[0]);
    });

    it('honours except', () => {
        const outputs = [3, 0, 0, 0, 'bytes:00000000'];
        const doc = {
            iscc_decode: {
                test_0001_error: { inputs: ['ISCC:GAAAAAAAAA'], outputs, note: 'n', except: { nodejs: 'error' } },
                test_0002_skip: { inputs: ['ISCC:GAAAAAAAAB'], outputs, note: 'n', except: { nodejs: 'skip' } },
            },
        };
        const failures = checkDoc(doc);
        strictEqual(failures.length, 1);
        ok(failures[0].startsWith('iscc_decode/test_0001_error: expected an error'), failures[0]);
    });

    it('compares an object output', () => {
        // The data.json Hello World vector: `characters` is uncompared, `iscc` matches.
        const outputs = { iscc: 'ISCC:EAASKDNZNYGUUF5A', characters: 10 };
        const doc = { gen_text_code_v0: { test_0001_x: { inputs: ['Hello World', 64], outputs, note: 'n' } } };
        deepStrictEqual(checkDoc(doc), []);
    });

    it('fails an expected key outside the uncompared set', () => {
        // `parts` is uncompared for mixed only, so a text result lacking it fails.
        const outputs = { iscc: 'ISCC:EAASKDNZNYGUUF5A', parts: [] };
        const doc = { gen_text_code_v0: { test_0001_x: { inputs: ['Hello World', 64], outputs, note: 'n' } } };
        const failures = checkDoc(doc);
        strictEqual(failures.length, 1, failures.join('\n'));
        ok(failures[0].includes('missing field parts'), failures[0]);
    });

    it('fails a result without iscc', () => {
        const failure = compare(
            'gen_text_code_v0',
            { iscc: 'ISCC:EAASKDNZNYGUUF5A', characters: 10 },
            { characters: 10 },
        );
        ok(failure?.includes('missing field iscc'), String(failure));
    });
});
