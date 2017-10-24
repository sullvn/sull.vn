import * as React from 'react'
import Helmet from 'react-helmet'
import Typekit from 'react-typekit'
import {
  default as glamorous,
  Div, Main, Footer,
  CSSProperties, ThemeProvider,
} from 'glamorous'
import { transparentize } from 'polished'

import Header from '../components/Header'
import Tombstone from '../components/Tombstone'
import {
  baseWidth, black,
  doubleFade, borderWidth, fontFutura,
  Theme, darkTheme,
} from '../style'

import '../../node_modules/prismjs/themes/prism-solarizedlight.css'
const favicon16 = require( '../assets/favicon.16.png' )
const favicon32 = require( '../assets/favicon.32.png' )
const favicon64 = require( '../assets/favicon.64.png' )
const favicon152 = require( '../assets/favicon.152.png' )


interface BaseProps {
  children: React.ReactNode
  theme?: Theme
}


export default function Base( props: BaseProps ): JSX.Element {
  const {
    children,
    theme = darkTheme,
  } = props

  return (
    <ThemeProvider theme={ theme }>
      <StyledBase>
        <Helmet
          titleTemplate="%s ◇ Axolotl Industries"
          defaultTitle="Axolotl Industries"
        >
          <link rel="icon" sizes="16x16" href={ favicon16 } />
          <link rel="icon" sizes="32x32" href={ favicon32 } />
          <link rel="icon" sizes="64x64" href={ favicon64 } />
          <link rel="icon" sizes="152x152" href={ favicon152 } />

          <meta name="theme-color" content={ theme.bgColor } />
          <style>{`
            html {
              background: ${ theme.bgColor };
            }
          `}</style>
        </Helmet>

        <Typekit kitId="obw1zlc" />

        <Div css={ bodyStyle }>
          <Header />

          <Main css={ mainStyle }>
            { children as React.ReactElement<any> }
          </Main>

          <Footer css={ footerStyle }>
            <Tombstone />
          </Footer>
        </Div>
      </StyledBase>
    </ThemeProvider>
  )
}


const bodyStyle: CSSProperties = {
  display: 'flex',
  flexDirection: 'column',
  minHeight: '100vh',
  maxWidth: baseWidth,

  margin: '0 auto',
  padding: '1em',
}

const mainStyle: CSSProperties = {
  flex: 1,
}

const footerStyle: CSSProperties = {
  display: 'flex',
  justifyContent: 'center',

  margin: '4rem 0 2rem',
}

const StyledBase = glamorous.div({
  fontFamily: fontFutura,

  transition: '1s linear background',

  '& h1, h2, h3, h4, h5, h6': {
    fontFamily: fontFutura,
    textTransform: 'uppercase',
  },

  '& a:any-link': {
    textDecoration: 'none',
  },

  '& pre[class*="language-"]': {
    fontSize: '.8em',
    lineHeight: '1.5',

    padding: '1.5em',

    color: 'inherit',
    background: 'none',
    borderLeft: `${ borderWidth } solid ${ transparentize( doubleFade, black ) }`,
    borderRadius: 0,
  },
}, ({ theme }: { theme: Theme }) => ({
  color: theme.fgColor,
}))
