const crypto = require('crypto');
const makeFetchHappen = require('fetch-cookie');
const testy = require('testytesterson');

const { endpoint } = require('./constants');

async function createUser(userCreate){
    const fetchCookie = makeFetchHappen(fetch)

    let email = testy.email();

    let toCreate = {
        user_id: userCreate?.user_id ?? crypto.randomUUID(),
        parent_id: userCreate?.parent_id ?? "00000000-0000-0000-0000-000000000000",
        display_name: userCreate?.display_name ?? testy.name(),
        email: userCreate?.email ?? email,
        password: userCreate?.password  ?? `${email}-password`,
        is_verified: userCreate?.is_verified ?? true,
        is_admin: userCreate?.is_admin ?? false,
    }

    let root = await fetchCookie(`${endpoint}/auth/test/create_user`,{
        method: 'POST',
        body: JSON.stringify(toCreate),
        headers: {
            'Content-Type': 'application/json'
        }
    });

    let returnValue = await root.json();

    return {
        fetch: fetchCookie,
        user: toCreate,
        sessionToken: returnValue.session_token,
        id: returnValue.user_id,
    }
}

module.exports = {
    createUser,
}