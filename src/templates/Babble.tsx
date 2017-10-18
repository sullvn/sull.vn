import * as React from 'react'
import Helmet from 'react-helmet'
import {
  Article, Hgroup, H1, Time,
  CSSProperties,
} from 'glamorous'
import { transparentize } from 'polished'

import Base from './Base'
import { textLinkCSS } from '../components/TextLink'
import {
  black, fontFutura,
  halfFade, doubleFade,
  articleTextWidth, borderWidth, contentStartSpacing,
  heading, textContent, visualContent,
  lightTheme,
} from '../style'


interface BabbleProps {
  data: {
    markdownRemark: {
      html: string
      frontmatter: {
        title: string
        date: string
      }
    }
  }
}


export default function Babble({ data }: BabbleProps ) {
  const { markdownRemark: babble } = data

  return (
    <Base theme={ lightTheme }>
      <Helmet title={ babble.frontmatter.title } />
      <Article css={ articleStyle }>
        <Hgroup css={ hgroupStyle }>
          <H1 css={ h1Style }>{ babble.frontmatter.title }</H1>
          <Time css={ timeStyle }>{ babble.frontmatter.date }</Time>
        </Hgroup>
        <div dangerouslySetInnerHTML={{ __html: babble.html }} />
      </Article>
    </Base>
  );
}


const articleStyle: CSSProperties = {
  margin: '5em auto 0',

  fontFamily: '"Palatino Linotype", "Book Antiqua", Palatino, serif',

  '& a[href]': textLinkCSS,

  [ `& ${ textContent }, ${ heading }` ]: {
    maxWidth: articleTextWidth,
    marginLeft: 'auto',
    marginRight: 'auto',
  },

  [ `& ${ heading }` ]: {
    marginTop: '3em',

    [ `& + h1, + h2, + h3, + h4, + h5, + h6` ]: {
      marginTop: 0,
    }
  },

  '& li': {
    marginLeft: '1.45em',
  },

  [ `& ${ visualContent }` ]: {
    margin: '5em auto',
  },

  '& blockquote': {
    padding: '0 1em',

    fontStyle: 'italic',
    borderLeft: `${ borderWidth } solid ${ transparentize( doubleFade, black ) }`,
  },
}

const hgroupStyle: CSSProperties = {
  margin: `${ contentStartSpacing } 0`,

  fontWeight: 'bold',
  fontFamily: fontFutura,
  textAlign: 'center',

  [ `@media(max-width: ${ articleTextWidth })` ]: {
    textAlign: 'left',
  },
}

const h1Style: CSSProperties = {
  margin: 0,
  maxWidth: 'unset !important',
}

const timeStyle: CSSProperties = {
  display: 'block',
  marginTop: '1em',

  textTransform: 'uppercase',
  opacity: halfFade,
}


export const pageQuery = graphql`
  query BabbleByPath($path: String!) {
    markdownRemark(frontmatter: { path: { eq: $path } }) {
      html
      frontmatter {
        date(formatString: "MMMM DD, YYYY")
        path
        title
      }
    }
  }
`
