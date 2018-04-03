module.exports = {
  siteMetadata: {
    title: 'Axolotl Industries',
  },
  plugins: [
    'gatsby-plugin-emotion',
    'gatsby-plugin-catch-links',
    'gatsby-plugin-react-helmet',
    'gatsby-plugin-typescript',
    {
      resolve: `gatsby-transformer-remark`,
      options: {
        plugins: [
          'gatsby-remark-prismjs',
          'gatsby-remark-smartypants',
          'gatsby-remark-katex',
          {
            resolve: 'gatsby-remark-images',
            options: {
              maxWidth: 864,
              linkImagesToOriginal: false,
              backgroundColor: '#fffbf4',
            },
          },
        ],
      },
    },
    {
      resolve: 'gatsby-source-filesystem',
      options: {
        path: `${__dirname}/src/babbles`,
        name: 'babbles',
      },
    },
    {
      resolve: 'gatsby-plugin-typography',
      options: {
        pathToConfigModule: 'src/config/typography.js',
      },
    },
  ],
}
