const test = require('node:test');
const assert = require('assert');

const {endpoint} = require('./constants');

test('Can connect to localhost', async () => {
	let root = await fetch(`${endpoint}/basic/hello`);

	let text = await root.text();

	assert.strictEqual(text, "Hello world");
});