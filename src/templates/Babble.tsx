import * as React from 'react'
import Helmet from 'react-helmet'
import styled from 'react-emotion'
import { transparentize } from 'polished'

import Base from './Base'
import { textLinkCSS } from '../components/TextLink'
import {
  black,
  fontFutura,
  halfFade,
  doubleFade,
  articleTextWidth,
  borderWidth,
  contentStartSpacing,
  heading,
  textContent,
  visualContent,
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

export default function Babble({ data }: BabbleProps) {
  const { markdownRemark: babble } = data

  return (
    <Base>
      <Helmet title={babble.frontmatter.title} />
      <Article>
        <Hgroup>
          <Title>{babble.frontmatter.title}</Title>
          <Time>{babble.frontmatter.date}</Time>
        </Hgroup>
        <div dangerouslySetInnerHTML={{ __html: babble.html }} />
      </Article>
    </Base>
  )
}

const Article = styled('article')({
  margin: '5em auto 0',

  '& a[href]': textLinkCSS,

  [`& ${textContent}, ${heading}`]: {
    maxWidth: articleTextWidth,
    marginLeft: 'auto',
    marginRight: 'auto',
  },

  [`& ${heading}`]: {
    marginTop: '3em',

    [`& + h1, + h2, + h3, + h4, + h5, + h6`]: {
      marginTop: 0,
    },
  },

  '& li': {
    marginLeft: '1.45em',
  },

  [`& ${visualContent}`]: {
    margin: '5em auto',
  },

  '& blockquote': {
    padding: '0 1em',

    fontStyle: 'italic',
    borderLeft: `${borderWidth} solid ${transparentize(doubleFade, black)}`,
  },
})

const Hgroup = styled('hgroup')({
  ...fontFutura,
  margin: `${contentStartSpacing} 0`,

  textAlign: 'center',

  [`@media(max-width: ${articleTextWidth})`]: {
    textAlign: 'left',
  },
})

const Title = styled('h1')({
  margin: 0,
  maxWidth: 'unset !important',
})

const Time = styled('time')({
  display: 'block',
  marginTop: '1em',

  textTransform: 'uppercase',
  opacity: halfFade,
})

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
