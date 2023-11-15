const test = require('node:test');
const assert = require('assert');

const {endpoint} = require('./constants');

test('Can connect to localhost', async () => {
	let root = await fetch(`${endpoint}/basic/hello`);

	let text = await root.text();

	assert.strictEqual(text, "Hello world");
});

test('Can get some JSON', async () => {
	let root = await fetch(`${endpoint}/basic/simple_json`);

	let json = await root.json();

	assert.strictEqual(json.username, "harbo");
	let currentTimestamp = Date.now();
	assert(json.timestamp_ms > (currentTimestamp - 10000));
	assert(json.timestamp_ms < (currentTimestamp + 10000));

	assert(json.active);
});