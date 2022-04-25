"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.default = void 0;

function _path() {
  const data = _interopRequireDefault(require("path"));

  _path = function () {
    return data;
  };

  return data;
}

function _process() {
  const data = _interopRequireDefault(require("process"));

  _process = function () {
    return data;
  };

  return data;
}

function _plugin() {
  const data = require("@parcel/plugin");

  _plugin = function () {
    return data;
  };

  return data;
}

function _utils() {
  const data = require("@parcel/utils");

  _utils = function () {
    return data;
  };

  return data;
}

function _diagnostic() {
  const data = require("@parcel/diagnostic");

  _diagnostic = function () {
    return data;
  };

  return data;
}

var _native = require("../native");

function _workers() {
  const data = _interopRequireDefault(require("@parcel/workers"));

  _workers = function () {
    return data;
  };

  return data;
}

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

var _default = new (_plugin().Optimizer)({
  async optimize({
    bundle,
    contents,
    logger
  }) {
    if (!bundle.env.shouldOptimize) {
      return {
        contents
      };
    }

    await loadOnMainThreadIfNeeded();
    let buffer = await (0, _utils().blobToBuffer)(contents); // Attempt to optimize it, if the optimize fails we log a warning...

    try {
      let optimized = (0, _native.optimize)(bundle.type, buffer);
      return {
        contents: optimized.length < buffer.length ? optimized : buffer
      };
    } catch (err) {
      var _bundle$getMainEntry;

      const filepath = (_bundle$getMainEntry = bundle.getMainEntry()) === null || _bundle$getMainEntry === void 0 ? void 0 : _bundle$getMainEntry.filePath;
      const filename = filepath ? _path().default.relative(_process().default.cwd(), filepath) : 'unknown';
      logger.warn({
        message: (0, _diagnostic().md)`Could not optimize image ${filename}: ${err.message}`,
        stack: err.stack
      });
    }

    return {
      contents: buffer
    };
  }

}); // On linux with older versions of glibc (e.g. CentOS 7), we encounter a segmentation fault
// when worker threads exit due to thread local variables in Rust. A workaround is to
// also load the native module on the main thread, so that it is not unloaded until process exit.
// See https://github.com/rust-lang/rust/issues/91979.


exports.default = _default;
let isLoadedOnMainThread = false;

async function loadOnMainThreadIfNeeded() {
  if (!isLoadedOnMainThread && _process().default.platform === 'linux' && _workers().default.isWorker()) {
    let {
      family,
      version
    } = require('detect-libc');

    if (family === 'glibc' && parseFloat(version) <= 2.17) {
      let api = _workers().default.getWorkerApi();

      await api.callMaster({
        location: __dirname + '/loadNative.js',
        args: []
      });
      isLoadedOnMainThread = true;
    }
  }
}