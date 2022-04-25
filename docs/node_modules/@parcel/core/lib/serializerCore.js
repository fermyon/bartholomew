"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.serializeRaw = exports.deserializeRaw = void 0;

function _v() {
  const data = _interopRequireDefault(require("v8"));

  _v = function () {
    return data;
  };

  return data;
}

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

// $FlowFixMe - Flow doesn't know about this method yet
let serializeRaw = _v().default.serialize; // $FlowFixMe - Flow doesn't know about this method yet


exports.serializeRaw = serializeRaw;

let deserializeRaw = _v().default.deserialize;

exports.deserializeRaw = deserializeRaw;