module.exports = {
  siteMetadata: {
    title: 'Gatsby Test',
  },
  plugins: [
    'gatsby-plugin-glamor',
    'gatsby-plugin-catch-links',
    'gatsby-plugin-react-helmet',
    'gatsby-plugin-typescript',
    {
      resolve: `gatsby-transformer-remark`,
      options: {
        plugins: [ { resolve: `gatsby-remark-prismjs` } ],
      },
    },
    {
      resolve: 'gatsby-source-filesystem',
      options: {
        path: `${ __dirname }/src/babbles`,
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
