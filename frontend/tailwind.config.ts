import type { Config } from 'tailwindcss';

export default {
  content: ['./app/**/*.{js,jsx,ts,tsx}'],
  theme: {
    extend: {
      fontSize: {
        'sm': '0.750rem',
        'base': '1rem',
        'xl': '1.333rem',
        '2xl': '1.777rem',
        '3xl': '2.369rem',
        '4xl': '3.158rem',
        '5xl': '4.210rem',
      },
      fontFamily: {
        // https://systemfontstack.com/
        heading: 'Playfair Display Variable, Iowan Old Style, Apple Garamond, Baskerville, Times New Roman, Droid Serif, Times, Source Serif Pro, serif, Apple Color Emoji, Segoe UI Emoji, Segoe UI Symbol',
        body: 'Raleway Variable, -apple-system, BlinkMacSystemFont, avenir next, avenir, segoe ui, helvetica neue, helvetica, Cantarell, Ubuntu, roboto, noto, arial, sans-serif',
      },
      fontWeight: {
        normal: '400',
        bold: '700',
      },
      colors: {
        text: {
          50: 'hsl(120, 12%, 5%)',
          100: 'hsl(120, 14%, 10%)',
          200: 'hsl(120, 12%, 20%)',
          300: 'hsl(120, 12%, 30%)',
          400: 'hsl(120, 12%, 40%)',
          500: 'hsl(120, 12%, 50%)',
          600: 'hsl(120, 12%, 60%)',
          700: 'hsl(120, 12%, 70%)',
          800: 'hsl(120, 12%, 80%)',
          900: 'hsl(120, 14%, 90%)',
          950: 'hsl(120, 12%, 95%)',
        },
        background: {
          50: 'hsl(113, 31%, 5%)',
          100: 'hsl(109, 33%, 10%)',
          200: 'hsl(108, 33%, 20%)',
          300: 'hsl(108, 33%, 30%)',
          400: 'hsl(108, 33%, 40%)',
          500: 'hsl(108, 33%, 50%)',
          600: 'hsl(108, 33%, 60%)',
          700: 'hsl(108, 33%, 70%)',
          800: 'hsl(108, 33%, 80%)',
          900: 'hsl(109, 33%, 90%)',
          950: 'hsl(105, 31%, 95%)',
        },
        primary: {
          50: 'hsl(115, 52%, 5%)',
          100: 'hsl(118, 49%, 10%)',
          200: 'hsl(118, 51%, 20%)',
          300: 'hsl(118, 50%, 30%)',
          400: 'hsl(118, 50%, 40%)',
          500: 'hsl(118, 50%, 50%)',
          600: 'hsl(118, 50%, 60%)',
          700: 'hsl(118, 50%, 70%)',
          800: 'hsl(119, 50%, 80%)',
          900: 'hsl(118, 49%, 90%)',
          950: 'hsl(120, 52%, 95%)',
        },
        secondary: {
          50: 'hsl(120, 69%, 5%)',
          100: 'hsl(117, 73%, 10%)',
          200: 'hsl(118, 71%, 20%)',
          300: 'hsl(118, 71%, 30%)',
          400: 'hsl(118, 71%, 40%)',
          500: 'hsl(118, 71%, 50%)',
          600: 'hsl(118, 71%, 60%)',
          700: 'hsl(118, 71%, 70%)',
          800: 'hsl(118, 71%, 80%)',
          900: 'hsl(117, 73%, 90%)',
          950: 'hsl(117, 69%, 95%)',
        },
        accent: {
          50: 'hsl(118, 92%, 5%)',
          100: 'hsl(119, 92%, 10%)',
          200: 'hsl(118, 92%, 20%)',
          300: 'hsl(118, 93%, 30%)',
          400: 'hsl(118, 93%, 40%)',
          500: 'hsl(118, 93%, 50%)',
          600: 'hsl(118, 93%, 60%)',
          700: 'hsl(118, 93%, 70%)',
          800: 'hsl(118, 92%, 80%)',
          900: 'hsl(119, 92%, 90%)',
          950: 'hsl(118, 92%, 95%)',
        },
      },

    },
  },
  plugins: [],
} satisfies Config;
