import globals from 'globals';
import { fixupConfigRules, fixupPluginRules } from '@eslint/compat';
import react from 'eslint-plugin-react';
import jsxA11Y from 'eslint-plugin-jsx-a11y';
import _import from 'eslint-plugin-import';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import js from '@eslint/js';
import { FlatCompat } from '@eslint/eslintrc';
import tseslint from 'typescript-eslint';
import StylisticPlugin from '@stylistic/eslint-plugin';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const compat = new FlatCompat({
  baseDirectory: __dirname,
  recommendedConfig: js.configs.recommended,
  allConfig: js.configs.all,
});

export default [
  {
    ignores: ['!**/.server', '!**/.client'],
  },
  ...compat.extends('eslint:recommended'), {
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.commonjs,
      },

      ecmaVersion: 'latest',
      sourceType: 'module',

      parserOptions: {
        ecmaFeatures: {
          jsx: true,
        },
      },
    },
  },
  ...fixupConfigRules(compat.extends(
    'plugin:react/recommended',
    'plugin:react/jsx-runtime',
    'plugin:react-hooks/recommended',
    'plugin:jsx-a11y/recommended',
  )).map(config => ({
    ...config,
    files: ['**/*.{js,jsx,ts,tsx}'],
  })),
  {
    files: ['**/*.{js,jsx,ts,tsx}'],

    plugins: {
      'react': fixupPluginRules(react),
      'jsx-a11y': fixupPluginRules(jsxA11Y),
    },

    settings: {
      'react': {
        version: 'detect',
      },

      'formComponents': ['Form'],

      'linkComponents': [{
        name: 'Link',
        linkAttribute: 'to',
      }, {
        name: 'NavLink',
        linkAttribute: 'to',
      }],

      'import/resolver': {
        typescript: {},
      },
    },
  },
  ...fixupConfigRules(compat.extends(
    'plugin:import/recommended',
    'plugin:import/typescript',
  )).map(config => ({
    ...config,
    files: ['**/*.{ts,tsx}'],
  })),
  {
    files: ['**/*.{ts,tsx}'],

    plugins: {
      import: fixupPluginRules(_import),
    },

    languageOptions: {
      parser: tseslint.parser,
    },

    settings: {
      'import/internal-regex': '^~/',

      'import/resolver': {
        node: {
          extensions: ['.ts', '.tsx'],
        },

        typescript: {
          alwaysTryTypes: true,
        },
      },
    },
  },
  {
    files: ['**/.eslintrc.cjs'],

    languageOptions: {
      globals: {
        ...globals.node,
      },
    },
  },
  ...tseslint.configs.recommended,
  StylisticPlugin.configs.customize({
    jsx: true,
    semi: true,
  }),
];
