/**
 * Conformance tests for all 9 gen_*_v0 napi-rs bindings against data.json vectors.
 *
 * Mirrors the Python conformance tests in tests/test_conformance.py.
 * Uses Node.js built-in test runner (node:test) — no extra dependencies.
 */

import { readFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, it } from 'node:test';
import { strictEqual } from 'node:assert';

import {
    gen_meta_code_v0,
    gen_text_code_v0,
    gen_image_code_v0,
    gen_audio_code_v0,
    gen_video_code_v0,
    gen_mixed_code_v0,
    gen_data_code_v0,
    gen_instance_code_v0,
    gen_iscc_code_v0,
} from '../index.js';

const __dirname = dirname(fileURLToPath(import.meta.url));
const dataPath = join(__dirname, '..', '..', 'iscc-lib', 'tests', 'data.json');
const data = JSON.parse(readFileSync(dataPath, 'utf-8'));

/**
 * Stringify an object with sorted keys (matching Python json.dumps(sort_keys=True)).
 */
function jsonSortedStringify(obj) {
    return JSON.stringify(obj, Object.keys(obj).sort());
}

/**
 * Convert meta input from JSON value to string argument for gen_meta_code_v0.
 */
function prepareMetaArg(metaVal) {
    if (metaVal === null || metaVal === undefined) {
        return undefined;
    }
    if (typeof metaVal === 'string') {
        return metaVal;
    }
    if (typeof metaVal === 'object') {
        return jsonSortedStringify(metaVal);
    }
    throw new Error(`unexpected meta type: ${typeof metaVal}`);
}

/**
 * Decode 'stream:<hex>' input to Buffer.
 */
function decodeStream(streamStr) {
    const hexData = streamStr.replace(/^stream:/, '');
    if (hexData.length === 0) {
        return Buffer.alloc(0);
    }
    return Buffer.from(hexData, 'hex');
}

// ── gen_meta_code_v0 ─────────────────────────────────────────────────────────

describe('gen_meta_code_v0', () => {
    for (const [name, tc] of Object.entries(data.gen_meta_code_v0)) {
        it(name, () => {
            const inputs = tc.inputs;
            const nameArg = inputs[0];
            const description = inputs[1] || undefined;
            const meta = prepareMetaArg(inputs[2]);
            const bits = inputs[3];

            const result = gen_meta_code_v0(nameArg, description, meta, bits);
            strictEqual(result, tc.outputs.iscc);
        });
    }
});

// ── gen_text_code_v0 ─────────────────────────────────────────────────────────

describe('gen_text_code_v0', () => {
    for (const [name, tc] of Object.entries(data.gen_text_code_v0)) {
        it(name, () => {
            const inputs = tc.inputs;
            const result = gen_text_code_v0(inputs[0], inputs[1]);
            strictEqual(result, tc.outputs.iscc);
        });
    }
});

// ── gen_image_code_v0 ────────────────────────────────────────────────────────

describe('gen_image_code_v0', () => {
    for (const [name, tc] of Object.entries(data.gen_image_code_v0)) {
        it(name, () => {
            const inputs = tc.inputs;
            const pixels = Buffer.from(inputs[0]);
            const result = gen_image_code_v0(pixels, inputs[1]);
            strictEqual(result, tc.outputs.iscc);
        });
    }
});

// ── gen_audio_code_v0 ────────────────────────────────────────────────────────

describe('gen_audio_code_v0', () => {
    for (const [name, tc] of Object.entries(data.gen_audio_code_v0)) {
        it(name, () => {
            const inputs = tc.inputs;
            const result = gen_audio_code_v0(inputs[0], inputs[1]);
            strictEqual(result, tc.outputs.iscc);
        });
    }
});

// ── gen_video_code_v0 ────────────────────────────────────────────────────────

describe('gen_video_code_v0', () => {
    for (const [name, tc] of Object.entries(data.gen_video_code_v0)) {
        it(name, () => {
            const inputs = tc.inputs;
            const result = gen_video_code_v0(inputs[0], inputs[1]);
            strictEqual(result, tc.outputs.iscc);
        });
    }
});

// ── gen_mixed_code_v0 ────────────────────────────────────────────────────────

describe('gen_mixed_code_v0', () => {
    for (const [name, tc] of Object.entries(data.gen_mixed_code_v0)) {
        it(name, () => {
            const inputs = tc.inputs;
            const result = gen_mixed_code_v0(inputs[0], inputs[1]);
            strictEqual(result, tc.outputs.iscc);
        });
    }
});

// ── gen_data_code_v0 ─────────────────────────────────────────────────────────

describe('gen_data_code_v0', () => {
    for (const [name, tc] of Object.entries(data.gen_data_code_v0)) {
        it(name, () => {
            const inputs = tc.inputs;
            const buf = decodeStream(inputs[0]);
            const result = gen_data_code_v0(buf, inputs[1]);
            strictEqual(result, tc.outputs.iscc);
        });
    }
});

// ── gen_instance_code_v0 ─────────────────────────────────────────────────────

describe('gen_instance_code_v0', () => {
    for (const [name, tc] of Object.entries(data.gen_instance_code_v0)) {
        it(name, () => {
            const inputs = tc.inputs;
            const buf = decodeStream(inputs[0]);
            const result = gen_instance_code_v0(buf, inputs[1]);
            strictEqual(result, tc.outputs.iscc);
        });
    }
});

// ── gen_iscc_code_v0 ─────────────────────────────────────────────────────────

describe('gen_iscc_code_v0', () => {
    for (const [name, tc] of Object.entries(data.gen_iscc_code_v0)) {
        it(name, () => {
            const inputs = tc.inputs;
            const result = gen_iscc_code_v0(inputs[0]);
            strictEqual(result, tc.outputs.iscc);
        });
    }
});
