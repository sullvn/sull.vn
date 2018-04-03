import * as React from 'react'
import styled from 'react-emotion'
import Link from 'gatsby-link'

import { doubleFade, articleTextWidth, fontFutura } from '../style'
import Sprinkles from './Sprinkles'

export default function Header(): JSX.Element {
  return (
    <StyledHeader>
      <Nav>
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

const StyledHeader = styled('header')(fontFutura)

const Nav = styled('nav')({
  display: 'flex',
  justifyContent: 'flex-start',

  textTransform: 'uppercase',

  [`@media(max-width: ${articleTextWidth})`]: {
    flexDirection: 'column',
    alignItems: 'flex-start',
    fontSize: '1rem',
  },
})

const HeaderLink = styled(Link)({
  fontSize: '1.2rem',
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
