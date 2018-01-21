import * as React from 'react'
import {
  default as glamorous,
  Nav,
  Header as StyledHeader,
  CSSProperties,
} from 'glamorous'
import Link from 'gatsby-link'

import { doubleFade, articleTextWidth, fontFutura } from '../style'
import Sprinkles from './Sprinkles'

export default function Header(): JSX.Element {
  return (
    <StyledHeader css={headerStyle}>
      <Nav css={navStyle}>
        <HeaderLink to="/">
          <Sprinkles>Axolotl Industries</Sprinkles>
        </HeaderLink>
        <HeaderLink to="/babbles">
          <Sprinkles>Babbles</Sprinkles>
        </HeaderLink>
        <HeaderLink to="/resume">
          <Sprinkles>Resume</Sprinkles>
        </HeaderLink>
      </Nav>
    </StyledHeader>
  )
}

const headerStyle: CSSProperties = fontFutura

const navStyle: CSSProperties = {
  display: 'flex',
  justifyContent: 'flex-start',

  textTransform: 'uppercase',

  [`@media(max-width: ${articleTextWidth})`]: {
    flexDirection: 'column',
    alignItems: 'flex-start',
    fontSize: '1rem',
  },
}

const HeaderLink = glamorous(Link)({
  color: 'inherit',
  opacity: 1 - doubleFade,
  textDecoration: 'none',

  margin: '0 1em',

  ':hover': {
    opacity: 1,
  },

  ':first-child': {
    marginLeft: 0,
  },

  ':last-child': {
    marginRight: 0,
  },

  [`@media(max-width: ${articleTextWidth})`]: {
    margin: '0',
  },
})
