import * as React from 'react'
import {
  default as glamorous,
  H1, Img, Section,
  CSSProperties,
} from 'glamorous'

import Base from '../templates/Base'
import TextLink from '../components/TextLink'
import {
  green, purple, red,
  articleTextWidth, contentStartSpacing, largeHeaderSize,
  deepTextShadow,
} from '../style'
const waveTexture = require( '../assets/wave-texture.jpg' )


export default function IndexPage(): JSX.Element {
  return (
    <Base>
      <Section css={ sectionStyle }>
        <H1 css={ h1Style }>
          <Word>
            <H>A</H><H>x</H><H>o</H><H>l</H><H>o</H><H>t</H><H>l</H>
          </Word>
          <Word>
            <H>I</H><H>n</H><H>d</H><H>u</H><H>s</H><H>t</H><H>r</H><H>i</H><H>e</H><H>s</H>
          </Word>
        </H1>
        <Img css={ imgStyle } src={ waveTexture } />
        <P>Oh hey there! I'm Kevin Sullivan.</P>
        <P>
          I do stuff at the intersection of technology and creativity.
          Sometimes <TextLink href="/babbles">I write about that</TextLink>.
        </P>
        <P>
          I create interfaces at <TextLink href="https://aclima.io">Aclima</TextLink> with
          dense environmental sensor data.
        </P>
        <P>
          You can find me on <TextLink href="https://github.com/awfulaxolotl">Github</TextLink>, <TextLink href="https://keybase.io/awfulaxolotl">Keybase</TextLink>, or <TextLink href="mailto:kevin.sullivan@axolotl.industries">email</TextLink>. Say hello!
        </P>
        <P>
          Oh yeah, this website has nothing to do with cave salamanders.
          Not even one bit. No refunds.
        </P>
      </Section>
    </Base>
  )
}


const sectionStyle: CSSProperties = {
  marginTop: `calc(${ contentStartSpacing } / 2)`,
}

const h1Style: CSSProperties = {
  fontSize: largeHeaderSize,
  marginBottom: '-.3em',
  position: 'relative',
  zIndex: 1,
}

const imgStyle: CSSProperties = {
  paddingLeft: '30px',
  marginBottom: contentStartSpacing,

  mixBlendMode: 'lighten',
}

const P = glamorous.p({
  fontSize: '30px',
  lineHeight: 1.45,

  margin: '0 auto 4em',
  maxWidth: articleTextWidth,
})

const H = glamorous.span({
  ':nth-of-type( 3n )': {
    textShadow: deepTextShadow( 4, 4, green ),
  },
  ':nth-of-type( 3n+1 )': {
    textShadow: deepTextShadow( 0, 4, red ),
  },
  ':nth-of-type( 3n+2 )': {
    textShadow: deepTextShadow( -4, 4, purple ),
  },
})

const Word = glamorous.span({
  display: 'inline-block',
  marginRight: '.5em',

  ':last-child': {
    margin: 0,
  },
})