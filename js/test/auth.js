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
    let root = await fetch(`${endpoint}/auth/test/generate_invite_code`);

    let json = await root.json();

    assert(json.invite_code);
});

test('Create, verify, and log in as a new user', async () => {
    const fetchCookie = makeFetchHappen(fetch)

    let root = await fetch(`${endpoint}/auth/test/generate_invite_code`);
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
    let password = `${email}-password`;
    registerFormData.append('invite_code', invite_code);
    registerFormData.append('csrf_token', csrf_token);
    registerFormData.append('display_name', testy.name());
    registerFormData.append('email', email);
    registerFormData.append('password', password);
    registerFormData.append('tos', true);
    registerFormData.append('age', true);

    let register_form_response = await fetchCookie(`${endpoint}/auth/register`, {
        method: 'POST',
        body: registerFormData,
    });

    // This should take us to a page that demands we verify our email
    let responseText = await register_form_response.text();

    assert(responseText.includes('ok, user'));

    // there's an endpoint that will give us the email history
    let email_history = await fetchCookie(`${endpoint}/auth/test/get_last_email?email=${email}`);

    let email_code = await email_history.json();

    let url = email_code.email;

    let verify_email = await fetch(url);

    assert.strictEqual(await verify_email.text(), "ok");

    let test_verification = await fetchCookie(`${endpoint}/auth/ok`);

    assert.strictEqual(await test_verification.text(), "ok, verified user");

    const newFetchCookie = makeFetchHappen(fetch)

    let login_form_response = await newFetchCookie(`${endpoint}/auth/login`);

    let login_form_dom = new JSDOM(await login_form_response.text());
    let login_csrf_token = login_form_dom.window.document.querySelector("input[name=\"csrf_token\"]").value;

    const loginFormData = new FormData();
    loginFormData.append('csrf_token', login_csrf_token);
    loginFormData.append('email', email);
    loginFormData.append('password', password);

    let login_final_form_response = await newFetchCookie(`${endpoint}/auth/login`, {
        method: 'POST',
        body: loginFormData,
    });

    console.dir(await login_final_form_response.text())

});