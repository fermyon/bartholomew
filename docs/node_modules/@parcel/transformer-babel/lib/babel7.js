"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.default = babel7;

function _assert() {
  const data = _interopRequireDefault(require("assert"));

  _assert = function () {
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

var _remapAstLocations = require("./remapAstLocations");

var _package = _interopRequireDefault(require("../package.json"));

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

const transformerVersion = _package.default.version;
(0, _assert().default)(typeof transformerVersion === 'string');

async function babel7(opts) {
  var _babelOptions$config$, _babelOptions$config$2, _babelOptions$syntaxP;

  let {
    asset,
    options,
    babelOptions,
    additionalPlugins = []
  } = opts;
  const babelCore = await options.packageManager.require('@babel/core', asset.filePath, {
    range: '^7.12.0',
    saveDev: true,
    shouldAutoInstall: options.shouldAutoInstall
  });
  let config = { ...babelOptions.config,
    plugins: additionalPlugins.concat(babelOptions.config.plugins),
    code: false,
    ast: true,
    filename: asset.filePath,
    babelrc: false,
    configFile: false,
    parserOpts: { ...babelOptions.config.parserOpts,
      sourceFilename: (0, _utils().relativeUrl)(options.projectRoot, asset.filePath),
      allowReturnOutsideFunction: true,
      strictMode: false,
      sourceType: 'module',
      plugins: [...((_babelOptions$config$ = (_babelOptions$config$2 = babelOptions.config.parserOpts) === null || _babelOptions$config$2 === void 0 ? void 0 : _babelOptions$config$2.plugins) !== null && _babelOptions$config$ !== void 0 ? _babelOptions$config$ : []), ...((_babelOptions$syntaxP = babelOptions.syntaxPlugins) !== null && _babelOptions$syntaxP !== void 0 ? _babelOptions$syntaxP : []), // Applied by preset-env
      'classProperties', 'classPrivateProperties', 'classPrivateMethods', 'exportDefaultFrom' // 'topLevelAwait'
      ]
    },
    caller: {
      name: 'parcel',
      version: transformerVersion,
      targets: JSON.stringify(babelOptions.targets),
      outputFormat: asset.env.outputFormat
    }
  };
  let ast = await asset.getAST();
  let res;

  if (ast) {
    res = await babelCore.transformFromAstAsync(ast.program, asset.isASTDirty() ? undefined : await asset.getCode(), config);
  } else {
    res = await babelCore.transformAsync(await asset.getCode(), config);

    if (res.ast) {
      let map = await asset.getMap();

      if (map) {
        (0, _remapAstLocations.remapAstLocations)(babelCore.types, res.ast, map);
      }
    }
  }

  if (res.ast) {
    asset.setAST({
      type: 'babel',
      version: '7.0.0',
      program: res.ast
    });
  }
}