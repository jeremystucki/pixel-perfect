(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/pixel_perfect_wasm.js":
/*!************************************!*\
  !*** ../pkg/pixel_perfect_wasm.js ***!
  \************************************/
/*! exports provided: export_normally, force_export */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"export_normally\", function() { return export_normally; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"force_export\", function() { return force_export; });\n/* harmony import */ var _pixel_perfect_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./pixel_perfect_wasm_bg.wasm */ \"../pkg/pixel_perfect_wasm_bg.wasm\");\n\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _pixel_perfect_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_pixel_perfect_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nfunction passArray8ToWasm0(arg, malloc) {\n    const ptr = malloc(arg.length * 1);\n    getUint8Memory0().set(arg, ptr / 1);\n    WASM_VECTOR_LEN = arg.length;\n    return ptr;\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _pixel_perfect_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory0 = new Int32Array(_pixel_perfect_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nfunction getArrayU8FromWasm0(ptr, len) {\n    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);\n}\n/**\n* @param {Uint8Array} input\n* @returns {Uint8Array}\n*/\nfunction export_normally(input) {\n    var ptr0 = passArray8ToWasm0(input, _pixel_perfect_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    _pixel_perfect_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"export_normally\"](8, ptr0, len0);\n    var r0 = getInt32Memory0()[8 / 4 + 0];\n    var r1 = getInt32Memory0()[8 / 4 + 1];\n    var v1 = getArrayU8FromWasm0(r0, r1).slice();\n    _pixel_perfect_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1 * 1);\n    return v1;\n}\n\n/**\n* @param {Uint8Array} input\n* @param {number} pixel_size\n* @returns {Uint8Array}\n*/\nfunction force_export(input, pixel_size) {\n    var ptr0 = passArray8ToWasm0(input, _pixel_perfect_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    _pixel_perfect_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"force_export\"](8, ptr0, len0, pixel_size);\n    var r0 = getInt32Memory0()[8 / 4 + 0];\n    var r1 = getInt32Memory0()[8 / 4 + 1];\n    var v1 = getArrayU8FromWasm0(r0, r1).slice();\n    _pixel_perfect_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1 * 1);\n    return v1;\n}\n\n\n\n//# sourceURL=webpack:///../pkg/pixel_perfect_wasm.js?");

/***/ }),

/***/ "../pkg/pixel_perfect_wasm_bg.wasm":
/*!*****************************************!*\
  !*** ../pkg/pixel_perfect_wasm_bg.wasm ***!
  \*****************************************/
/*! exports provided: memory, export_normally, force_export, __wbindgen_malloc, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/pixel_perfect_wasm_bg.wasm?");

/***/ })

}]);