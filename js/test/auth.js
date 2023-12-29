const test = require('node:test');
const assert = require('assert');
const jsdom = require("jsdom");
const testy = require('testytesterson');
const makeFetchHappen = require('fetch-cookie');
const { JSDOM } = jsdom;

const { endpoint } = require('./constants');

test('Can connect to localhost', async () => {
    let root = await fetch(`${endpoint}/auth/invite`);

    let html = await root.text();

    assert(html);
});

test('Get a valid invite code from the bin', async () => {
    let root = await fetch(`${endpoint}/auth/generate_invite_code`);

    let json = await root.json();

    assert(json.invite_code);
});

test('Use the invite code to create a new user', async () => {
    const fetchCookie = makeFetchHappen(fetch)

    let root = await fetch(`${endpoint}/auth/generate_invite_code`);
    let json = await root.json();
    let invite_code = json.invite_code;

    const formData = new FormData();
    formData.append('invite_code', invite_code);

    let invite_form_response = await fetchCookie(`${endpoint}/auth/invite`, {
        method: 'POST',
        body: formData,
    });


    const formDom = new JSDOM(await invite_form_response.text());
    const csrf_token = formDom.window.document.querySelector("input[name=\"csrf_token\"]").value;
    const invite_code_again = formDom.window.document.querySelector("input[name=\"invite_code\"]").value;
    assert.strictEqual(invite_code, invite_code_again);

    const registerFormData = new FormData();
    let email = testy.email();
    registerFormData.append('invite_code', invite_code);
    registerFormData.append('csrf_token', csrf_token);
    registerFormData.append('display_name', testy.name());
    registerFormData.append('email', email);
    registerFormData.append('password', `${email}-password`);
    registerFormData.append('tos', true);
    registerFormData.append('age', true);

    let register_form_response = await fetchCookie(`${endpoint}/auth/register`, {
        method: 'POST',
        body: registerFormData,
    });

    // This should take us to a page that demands we verify our email
    let responseText = await register_form_response.text();
    console.warn(responseText);

    assert(responseText.includes('ok, user'));

    // there's an endpoint that will give us the email history


});