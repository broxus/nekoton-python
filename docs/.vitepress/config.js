import vue from '@vitejs/plugin-vue';

const HELP_URL = 'https://t.me/everdev';
const FEEDBACK_URL = '';
const GITHUB_URL = '';

module.exports = {
  title: 'Nekoton Python Docs',
  base: '/',
  description: 'nekoton-python',

  plugins: [vue()],
  rewrites: {
    'src/pages/index.md': 'index.md',
    'src/pages/concepts/data-representation.md': 'concepts/data-representation.md',
    'src/pages/concepts/abi.md': 'concepts/abi.md',
    'src/pages/concepts/accounts.md': 'concepts/accounts.md',
    'src/pages/concepts/messages.md': 'concepts/messages.md',
    'src/pages/concepts/transactions.md': 'concepts/transactions.md',
    'src/pages/guides/installation-and-quick-start.md': 'installation-and-quick-start.md',
    'src/pages/guides/keys-and-signatures.md': 'guides/keys-and-signatures.md',
    'src/pages/guides/working-with-cells.md': 'guides/working-with-cells.md',
    'src/pages/guides/working-with-abi.md': 'guides/working-with-abi.md',
    'src/pages/guides/working-with-accounts.md': 'guides/working-with-accounts.md',
    'src/pages/guides/working-with-messages.md': 'guides/working-with-messages.md',
    'src/pages/guides/working-with-transactions.md': 'guides/working-with-transactions.md',
    'src/pages/guides/working-with-transport.md': 'guides/working-with-transport.md',
  },
  themeConfig: {
    search: {
      provider: 'local',
    },
    nav: [
      { text: 'Feedback', link: FEEDBACK_URL },
      { text: 'Community', link: HELP_URL },
    ],
    sidebar: [
      { text: 'Overview', link: '/' },
      {
        text: 'Installation & Quick Start',
        link: '/installation-and-quick-start.md',
      },

      {
        text: 'Concepts',
        collapsable: false,
        items: [
          {
            text: 'Data Representation',
            link: '/concepts/data-representation.md',
          },
          { text: 'ABI', link: '/concepts/abi.md' },
          { text: 'Accounts', link: '/concepts/accounts.md' },
          { text: 'Messages', link: '/concepts/messages.md' },
          { text: 'Transactions', link: '/concepts/transactions.md' },
        ],
      },

      {
        text: 'Guide',
        collapsable: false,

        items: [
          {
            text: 'Keys & Signatures',
            link: '/guides/keys-and-signatures.md',
          },
          {
            text: 'Working with Cells',
            link: '/guides/working-with-cells.md',
          },
          {
            text: 'Working with ABI',
            link: '/guides/working-with-abi.md',
          },
          {
            text: 'Working with Accounts',
            link: '/guides/working-with-accounts.md',
          },
          {
            text: 'Working with Messages',
            link: '/guides/working-with-messages.md',
          },
          {
            text: 'Working with Transactions',
            link: '/guides/working-with-transactions.md',
          },
          {
            text: 'Working with Transport',
            link: '/guides/working-with-transport.md',
          },
        ],
      },
    ],
    editLink: {
      pattern: 'https://github.com/cyace84/nekoton-python/edit/docs/docs/:path',
    },
    socialLinks: [{ icon: 'github', link: GITHUB_URL }],
  },

  esbuild: {
    target: ['chrome89', 'edge89', 'firefox79', 'safari14.1'],
  },
};
