const test = require('node:test');
const assert = require('assert');
const jsdom = require("jsdom");
const testy = require('testytesterson');
const makeFetchHappen = require('fetch-cookie');
const { JSDOM } = jsdom;

const { endpoint } = require('./constants');
const { createUser } = require('./generator');

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

    // check our status
    let status_response = await fetchCookie(`${endpoint}/auth/status`);
    let status_text = await status_response.text();
    assert(status_text.includes("ok, not logged in"));

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
    let responseText = (await register_form_response.text()).toLowerCase();

    assert(responseText.includes('verify') && responseText.includes('email'));

    // check our status
    status_response = await fetchCookie(`${endpoint}/auth/status`);
    status_text = await status_response.text();
    assert(status_text.includes("ok, user"));

    // we didn't get an email, because we're running this in test mode and we don't have an email server
    // so we have to cheat:
    // there's an endpoint that will give us the email history
    let email_history = await fetchCookie(`${endpoint}/auth/test/get_last_email?email=${email}`);
    let email_code = await email_history.json();
    let url = email_code.email;
    let verify_email = await fetch(url);
    assert((await verify_email.text()).toLowerCase().includes("home"));

    // check our status
    status_response = await fetchCookie(`${endpoint}/auth/status`);
    status_text = await status_response.text();
    assert(status_text.includes("ok, verified user"));

    // once we've done that, if we hit /auth/ok, it will take us to the home page
    let test_verification = await fetchCookie(`${endpoint}/auth/ok`);
    let homeText = (await test_verification.text()).toLowerCase();
    assert(homeText.includes("home"));

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
    homeText = (await login_final_form_response.text()).toLowerCase();
    assert(homeText.includes("home"));
});

test('Quickly create a new user', async () => {
    let {fetch, user} = await createUser();

    let root = await fetch(`${endpoint}/auth/status`);

    let html = await root.text();

    assert(html.includes("ok, verified user"));
});

test('Quickly create a hundred new users', async () => {
    for(let i = 0; i < 100; i++){
        let {fetch, user} = await createUser();

        let root = await fetch(`${endpoint}/auth/status`);

        let html = await root.text();

        assert(html.includes("ok, verified user"));
    }
});