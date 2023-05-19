import * as React from 'react'
import Helmet from 'react-helmet'
import Typekit from 'react-typekit'
import styled from '@emotion/styled'
import { transparentize } from 'polished'

import Header from '../components/Header'
import Tombstone from '../components/Tombstone'
import {
  baseFontSize,
  baseWidth,
  black,
  doubleFade,
  borderWidth,
  fontFutura,
  fontSystem,
  foreground,
  background,
} from '../style'

import '../../node_modules/prismjs/themes/prism-solarizedlight.css'
import favicon16 from '../assets/favicon.16.png'
import favicon32 from '../assets/favicon.32.png'
import favicon64 from '../assets/favicon.64.png'
import favicon152 from '../assets/favicon.152.png'

interface BaseProps {
  children: React.ReactNode
}

export default function Base(props: BaseProps): JSX.Element {
  const { children } = props

  return (
    <StyledBase>
      <Helmet titleTemplate="%s ◇ Kevin Sullivan" defaultTitle="Kevin Sullivan">
        <link rel="icon" sizes="16x16" href={favicon16} />
        <link rel="icon" sizes="32x32" href={favicon32} />
        <link rel="icon" sizes="64x64" href={favicon64} />
        <link rel="icon" sizes="152x152" href={favicon152} />

        <meta property="og:site_name" content="Kevin Sullivan" />
        <meta property="og:locale" content="en_US" />

        <meta name="theme-color" content={background} />
      </Helmet>

      {/*
          Causes jittering on page transitions if placed within
          React Helmet.

          This is due to React Helmet naively reinserting the style
          on every page load.
      */}
      <style>{`
        :root {
          font-size: ${baseFontSize};
        }

        html {
          background: ${background};
        }
      `}</style>

      <Typekit kitId="obw1zlc" />

      <Body>
        <Header />

        <Main>{children as React.ReactElement<any>}</Main>

        <Footer>
          <Tombstone />
        </Footer>
      </Body>
    </StyledBase>
  )
}

const Body = styled('div')({
  display: 'flex',
  flexDirection: 'column',
  minHeight: '100vh',
  maxWidth: baseWidth,

  margin: '0 auto',
  padding: '1em',
})

const Main = styled('main')({
  flex: 1,
})

const Footer = styled('footer')({
  display: 'flex',
  justifyContent: 'center',

  margin: '4rem 0 2rem',

  [`@media print`]: {
    display: 'none',
  },
})

const StyledBase = styled('div')({
  color: foreground,
  fontFamily: fontSystem.fontFamily,

  transition: '1s linear background',

  '& h1, h2, h3, h4, h5, h6': fontFutura,

  '& a:any-link': {
    textDecoration: 'none',
  },

  '& pre[class*="language-"]': {
    padding: '1.5em',

    color: 'inherit',
    background: 'none',
    borderLeft: `${borderWidth} solid ${transparentize(doubleFade, black)}`,
    borderRadius: 0,
  },
})
