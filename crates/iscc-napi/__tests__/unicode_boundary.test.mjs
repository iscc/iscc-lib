/**
 * Unicode 16.0.0 boundary conformance tests for the napi binding.
 *
 * Runs the canonical boundary fixture (crates/iscc-lib/tests/unicode_boundary.json)
 * against text_clean and text_collapse, gating the declared-Unicode-version freeze
 * rule: code points unassigned in Unicode 16.0.0 are mapped to the noncharacter
 * sentinel U+FFFF before normalization and removed by the unchanged category-C
 * filter, while assigned code points flow through the regular pipeline. The
 * sequence vectors additionally pin the sentinel map against the superseded
 * delete-filter design.
 */

import { readFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, it } from 'node:test';
import { strictEqual } from 'node:assert';

import { text_clean, text_collapse } from '../index.js';

const __dirname = dirname(fileURLToPath(import.meta.url));

// Path to the canonical Unicode boundary vectors (read in place, no vendored copy).
const boundaryPath = join(__dirname, '..', '..', 'iscc-lib', 'tests', 'unicode_boundary.json');
const boundary = JSON.parse(readFileSync(boundaryPath, 'utf-8'));

describe('unicode_boundary fixture metadata', () => {
    // Guard against a truncated or renamed fixture silently defining zero vector tests.
    it('declares Unicode 16.0.0 and the expected case counts', () => {
        strictEqual(
            boundary._metadata.unicode_data_version,
            '16.0.0',
            'fixture must declare Unicode data version 16.0.0'
        );
        strictEqual(
            Object.keys(boundary.text_clean).length,
            7,
            'expected 7 text_clean boundary cases'
        );
        strictEqual(
            Object.keys(boundary.text_collapse).length,
            5,
            'expected 5 text_collapse boundary cases'
        );
    });
});

// ── text_clean ───────────────────────────────────────────────────────────────

describe('text_clean unicode boundary', () => {
    for (const [name, tc] of Object.entries(boundary.text_clean)) {
        it(name, () => {
            strictEqual(
                text_clean(...tc.inputs),
                tc.outputs.result,
                `text_clean mismatch for ${name}`
            );
        });
    }
});

// ── text_collapse ────────────────────────────────────────────────────────────

describe('text_collapse unicode boundary', () => {
    for (const [name, tc] of Object.entries(boundary.text_collapse)) {
        it(name, () => {
            strictEqual(
                text_collapse(...tc.inputs),
                tc.outputs.result,
                `text_collapse mismatch for ${name}`
            );
        });
    }
});
