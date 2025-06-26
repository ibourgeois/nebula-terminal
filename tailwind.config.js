/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./components/**/*.{js,vue,ts}",
    "./layouts/**/*.vue",
    "./pages/**/*.vue",
    "./plugins/**/*.{js,ts}",
    "./app.vue",
    "./error.vue"
  ],
  theme: {
    extend: {
      colors: {
        // Nord color palette
        nord: {
          0: '#2E3440',    // Polar Night (darkest)
          1: '#3B4252',    // Polar Night
          2: '#434C5E',    // Polar Night
          3: '#4C566A',    // Polar Night (lightest)
          4: '#D8DEE9',    // Snow Storm (darkest)
          5: '#E5E9F0',    // Snow Storm
          6: '#ECEFF4',    // Snow Storm (lightest)
          7: '#8FBCBB',    // Frost (lightest)
          8: '#88C0D0',    // Frost
          9: '#81A1C1',    // Frost
          10: '#5E81AC',   // Frost (darkest)
          11: '#BF616A',   // Aurora (red)
          12: '#D08770',   // Aurora (orange)
          13: '#EBCB8B',   // Aurora (yellow)
          14: '#A3BE8C',   // Aurora (green)
          15: '#B48EAD',   // Aurora (purple)
        },
        // Terminal-specific colors
        terminal: {
          bg: '#2E3440',      // Nord 0 - main background
          fg: '#ECEFF4',      // Nord 6 - main foreground
          selection: '#434C5E', // Nord 2 - selection background
          cursor: '#D8DEE9',   // Nord 4 - cursor color
          accent: '#88C0D0',   // Nord 8 - accent color
          error: '#BF616A',    // Nord 11 - error color
          warning: '#EBCB8B',  // Nord 13 - warning color
          success: '#A3BE8C',  // Nord 14 - success color
        }
      },
      fontFamily: {
        'mono': ['JetBrains Mono', 'Fira Code', 'Monaco', 'Consolas', 'monospace'],
        'terminal': ['JetBrains Mono', 'Fira Code', 'Monaco', 'Consolas', 'monospace'],
      },
      fontSize: {
        'terminal': ['14px', { lineHeight: '1.4' }],
        'terminal-lg': ['16px', { lineHeight: '1.4' }],
        'terminal-xl': ['18px', { lineHeight: '1.4' }],
      },
      spacing: {
        'terminal': '0.5rem',
        'terminal-lg': '1rem',
      }
    },
  },
  plugins: [],
} 