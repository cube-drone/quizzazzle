const test = require('node:test');
const assert = require('assert');

const { endpoint } = require('./constants');

test('Can connect to localhost', async () => {
    let root = await fetch(`${endpoint}/basic/hello`);

    let text = await root.text();

    assert.strictEqual(text, "Hello world");
});

test('Can get some JSON', async () => {
    let root = await fetch(`${endpoint}/basic/json`);

    let json = await root.json();

    assert.strictEqual(json.username, "harbo");
    let currentTimestamp = Date.now();
    assert(json.timestamp_ms > (currentTimestamp - 10000));
    assert(json.timestamp_ms < (currentTimestamp + 10000));

    assert(json.active);
});

test('Can create a Basic Thing', async () => {

    const response = await fetch(`${endpoint}/basic/thing`, {
        method: 'POST',
        body: JSON.stringify({
            name: "Test Thing 1",
        }),
        headers: {
            'Content-Type': 'application/json'
        }
    });

    assert.strictEqual(response.status, 200);
    const json = await response.json();
    assert(json.id != null);
    assert.strictEqual(json.name, "Test Thing 1");

    const getResponse = await fetch(`${endpoint}/basic/thing/${json.id}`);

    assert.strictEqual(getResponse.status, 200);
    const getJson = await getResponse.json();
    assert.strictEqual(getJson.id, json.id);
    assert.strictEqual(getJson.name, "Test Thing 1");
    assert.notEqual(new Date(getJson.created_at), "Invalid Date");
});

test('Validation Troubles: name too short', async () => {

    const response = await fetch(`${endpoint}/basic/thing`, {
        method: 'POST',
        body: JSON.stringify({
            name: "sht",
        }),
        headers: {
            'Content-Type': 'application/json'
        }
    });

    assert.strictEqual(response.status, 400);
});

test('Validation Troubles: name too long', async () => {

    const response = await fetch(`${endpoint}/basic/thing`, {
        method: 'POST',
        body: JSON.stringify({
            name: "tubatubatubatubatubatubatubatubatubatubatubatubatubatubatubatubatubatuba\
                    tubatubatubatubatubatubatubatubatubatubatubatubatubatubatubatubatubat\
                    tubatubatubatubatubatubatubatubatubatubatubatubatubatubatubatubatubat\
                    tubatubatubatubatubatubatubatubatubatubatubatubatubatubatubatubatubat\
                    tubatubatubatubatubatubatubatubatubatubatubatubatubatubatubatubatubat",
        }),
        headers: {
            'Content-Type': 'application/json'
        }
    });

    assert.strictEqual(response.status, 400);
});

test("Let's get some config", async () => {
    const response = await fetch(`${endpoint}/config/public`);

    assert.strictEqual(response.status, 200);

    let publicConfig = await response.json();
    assert.strictEqual(publicConfig.public_key, "public_value");

    const responsePriv = await fetch(`${endpoint}/config/private`);

    assert.strictEqual(responsePriv.status, 200);

    let privateConfig = await responsePriv.json();
    assert.strictEqual(privateConfig.private_key, "private_value");
});