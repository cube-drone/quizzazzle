// https://stackoverflow.com/questions/72390154/econnrefused-when-making-a-request-to-localhost-using-fetch-in-node-js
const dns = require('dns');
dns.setDefaultResultOrder('ipv4first');

let endpoint = process.env.LOCAL_ENDPOINT ?? 'http://127.0.0.1:3333';

module.exports = {
    endpoint
}