"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.default = createParcelBuildRequest;

var _AssetGraphRequest = _interopRequireDefault(require("./AssetGraphRequest"));

var _BundleGraphRequest = _interopRequireDefault(require("./BundleGraphRequest"));

var _WriteBundlesRequest = _interopRequireDefault(require("./WriteBundlesRequest"));

var _utils = require("../utils");

var _dumpGraphToGraphViz = _interopRequireDefault(require("../dumpGraphToGraphViz"));

var _BundleGraph = require("../BundleGraph");

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

function createParcelBuildRequest(input) {
  return {
    type: 'parcel_build_request',
    id: 'parcel_build_request',
    run,
    input
  };
}

async function run({
  input,
  api,
  options
}) {
  let {
    optionsRef,
    requestedAssetIds,
    signal
  } = input;
  let request = (0, _AssetGraphRequest.default)({
    name: 'Main',
    entries: options.entries,
    optionsRef,
    shouldBuildLazily: options.shouldBuildLazily,
    requestedAssetIds
  });
  let {
    assetGraph,
    changedAssets,
    assetRequests
  } = await api.runRequest(request, {
    force: options.shouldBuildLazily && requestedAssetIds.size > 0
  });
  let bundleGraphRequest = (0, _BundleGraphRequest.default)({
    assetGraph,
    optionsRef
  });
  let {
    bundleGraph,
    changedAssets: changedRuntimeAssets
  } = await api.runRequest(bundleGraphRequest);

  for (let [id, asset] of changedRuntimeAssets) {
    changedAssets.set(id, asset);
  } // $FlowFixMe Added in Flow 0.121.0 upgrade in #4381 (Windows only)


  (0, _dumpGraphToGraphViz.default)(bundleGraph._graph, 'BundleGraph', _BundleGraph.bundleGraphEdgeTypes);
  let writeBundlesRequest = (0, _WriteBundlesRequest.default)({
    bundleGraph,
    optionsRef
  });
  let bundleInfo = await api.runRequest(writeBundlesRequest);
  (0, _utils.assertSignalNotAborted)(signal);
  return {
    bundleGraph,
    bundleInfo,
    changedAssets,
    assetRequests
  };
}