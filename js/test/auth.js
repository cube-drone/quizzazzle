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

    let json = await fetch(`${endpoint}/auth/user`);
    let userJson = await json.json();

    assert.strictEqual(userJson.user_id, user.user_id);
    assert.strictEqual(userJson.display_name, user.display_name);
    assert.strictEqual(userJson.is_admin, user.is_admin);
    assert(userJson.thumbnail_url);
});

test('IP verification', async () => {
    let {fetch, user} = await createUser();

    let root = await fetch(`${endpoint}/auth/status`);
    let html = await root.text();
    assert(html.includes("ok, verified user"));

    // this is a test-only endpoint that will forget our IP address and log us out
    await fetch(`${endpoint}/auth/test/forget_ip`);

    let login_form_response = await fetch(`${endpoint}/auth/login`);
    let login_form_dom = new JSDOM(await login_form_response.text());
    let login_csrf_token = login_form_dom.window.document.querySelector("input[name=\"csrf_token\"]").value;

    const loginFormData = new FormData();
    loginFormData.append('csrf_token', login_csrf_token);
    loginFormData.append('email', user.email);
    loginFormData.append('password', user.password);

    let login_verify_response = await fetch(`${endpoint}/auth/login`, {
        method: 'POST',
        body: loginFormData,
    });
    homeText = (await login_verify_response.text()).toLowerCase();
    // oh no! we need to verify our location!
    assert(homeText.includes("verify") && homeText.includes("location"));

    // we didn't get an email, because we're running this in test mode and we don't have an email server
    // so we have to cheat:
    // there's an endpoint that will give us the email history (again)
    let email_history = await fetch(`${endpoint}/auth/test/get_last_email?email=${user.email}`);
    let email_code = await email_history.json();
    let url = email_code.email;
    let verify_email = await fetch(url);
    assert((await verify_email.text()).toLowerCase().includes("home"));

    // check our status
    status_response = await fetch(`${endpoint}/auth/status`);
    status_text = await status_response.text();
    assert(status_text.includes("ok, verified user"));
});

test('Password reset', async () => {
    let {fetch, user} = await createUser();

    let root = await fetch(`${endpoint}/auth/status`);
    let html = await root.text();
    assert(html.includes("ok, verified user"));

    await fetch(`${endpoint}/auth/logout`);

    let reset_form_response = await fetch(`${endpoint}/auth/password_reset`);
    let reset_form_dom = new JSDOM(await reset_form_response.text());
    let reset_csrf_token = reset_form_dom.window.document.querySelector("input[name=\"csrf_token\"]").value;
    assert(reset_csrf_token);

    const resetFormData = new FormData();
    resetFormData.append('csrf_token', reset_csrf_token);
    resetFormData.append('email', user.email);
    let reset_response = await fetch(`${endpoint}/auth/password_reset`, {
        method: 'POST',
        body: resetFormData,
    });

    let reset_text = (await reset_response.text()).toLowerCase();
    assert(reset_text.includes("email") && reset_text.includes("sent"));

    let email_history = await fetch(`${endpoint}/auth/test/get_last_email?email=${user.email}`);
    let email_code = await email_history.json();
    let url = email_code.email;
    let verify_email = await fetch(url);
    let verify_text = await verify_email.text();
    // this is a new form, with a new CSRF token
    let verify_form_dom = new JSDOM(verify_text);
    let verify_csrf_token = verify_form_dom.window.document.querySelector("input[name=\"csrf_token\"]").value;
    assert(verify_csrf_token);
    let verify_password_token = verify_form_dom.window.document.querySelector("input[name=\"password_token\"]").value;
    let new_password = `${user.email}-new-password`;

    const resetFormDataStage2 = new FormData();
    resetFormDataStage2.append('csrf_token', verify_csrf_token);
    resetFormDataStage2.append('password_token', verify_password_token);
    resetFormDataStage2.append('password', new_password);

    let reset_response_stage2 = await fetch(`${endpoint}/auth/password_reset/stage_2`, {
        method: 'POST',
        body: resetFormDataStage2,
    });
    // this SHOULD take you to home
    let reset_text_stage2 = await reset_response_stage2.text();
    assert(reset_text_stage2.includes("home"));

    // logout one more time, this time we log in with the updated password
    await fetch(`${endpoint}/auth/logout`);

    let login_form_response = await fetch(`${endpoint}/auth/login`);
    let login_form_dom = new JSDOM(await login_form_response.text());
    let login_csrf_token = login_form_dom.window.document.querySelector("input[name=\"csrf_token\"]").value;

    const loginFormData = new FormData();
    loginFormData.append('csrf_token', login_csrf_token);
    loginFormData.append('email', user.email);
    loginFormData.append('password', new_password);

    let login_final_form_response = await fetch(`${endpoint}/auth/login`, {
        method: 'POST',
        body: loginFormData,
    });
    // home page once more
    homeText = (await login_final_form_response.text()).toLowerCase();
    assert(homeText.includes("home"));

})