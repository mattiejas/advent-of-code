module.exports = {
  env: {
    es6: true,
    node: true,
  },
  extends: ['airbnb-base', 'prettier'],
  globals: {
    Atomics: 'readonly',
    SharedArrayBuffer: 'readonly',
  },
  parserOptions: {
    ecmaVersion: 2018,
    sourceType: 'module',
  },
  rules: {
    'no-plusplus': 0,
    'no-underscore-dangle': 0,
    'arrow-parens': 0,
    'no-console': 0,
    'object-curly-newline': 0,
    'wrap-iife': ['error', 'inside'],
  },
};
