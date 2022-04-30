"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.empty = exports.default = void 0;

function _module() {
  const data = require("module");

  _module = function () {
    return data;
  };

  return data;
}

// $FlowFixMe this is untyped
const empty = require.resolve('./_empty.js'); // $FlowFixMe


exports.empty = empty;
let builtins = Object.create(null); // use definite (current) list of Node builtins

for (let key of _module().builtinModules) {
  builtins[key] = empty;
}

builtins.assert = 'assert/';
builtins.buffer = 'buffer/';
builtins.console = 'console-browserify';
builtins.constants = 'constants-browserify';
builtins.crypto = 'crypto-browserify';
builtins.domain = 'domain-browser';
builtins.events = 'events/';
builtins.http = 'stream-http';
builtins.https = 'https-browserify';
builtins.os = 'os-browserify/browser.js';
builtins.path = 'path-browserify';
builtins.process = 'process/browser.js';
builtins.punycode = 'punycode/';
builtins.querystring = 'querystring-es3/';
builtins.stream = 'stream-browserify';
builtins.string_decoder = 'string_decoder/';
builtins.sys = 'util/util.js';
builtins.timers = 'timers-browserify';
builtins.tty = 'tty-browserify';
builtins.url = 'url/';
builtins.util = 'util/util.js';
builtins.vm = 'vm-browserify';
builtins.zlib = 'browserify-zlib';
var _default = builtins;
exports.default = _default;