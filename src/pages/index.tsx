import * as React from 'react'
import { default as glamorous, H1, Section, CSSProperties } from 'glamorous'

import Base from '../templates/Base'
import Cube3D from '../components/Cube3D'
import Disc3D from '../components/Disc3D'
import TextLink from '../components/TextLink'
import {
  green,
  purple,
  red,
  articleTextWidth,
  contentStartSpacing,
  largeHeaderSize,
  deepTextShadow,
  Theme,
} from '../style'

const waveTexture = require('../assets/wave-texture.jpg')
const portraitImage = require('../assets/portrait.png')

export default function IndexPage(): JSX.Element {
  return (
    <Base>
      <Section css={sectionStyle}>
        <H1 css={h1Style}>
          <Word>
            <H>A</H>
            <H>x</H>
            <H>o</H>
            <H>l</H>
            <H>o</H>
            <H>t</H>
            <H>l</H>
          </Word>
          <Word>
            <H>I</H>
            <H>n</H>
            <H>d</H>
            <H>u</H>
            <H>s</H>
            <H>t</H>
            <H>r</H>
            <H>i</H>
            <H>e</H>
            <H>s</H>
          </Word>
        </H1>
        <Texture>
          <Disc3D
            css={{
              position: 'absolute',
              left: '12%',
              bottom: '25%',
            }}
          />
          <Cube3D
            css={{
              position: 'absolute',
              left: '30%',
              bottom: '40%',
            }}
          />
          <Portrait src={portraitImage} />
        </Texture>
        <P>Oh hey there! I'm Kevin Sullivan.</P>
        <P>
          I do stuff at the intersection of technology and creativity.{' '}
          <TextLink href="/babbles">Sometimes I write about that</TextLink>.
        </P>
        <P>
          I create interfaces at{' '}
          <TextLink href="https://aclima.io">Aclima</TextLink> with dense
          environmental sensor data.
        </P>
        <P>
          You can find me on{' '}
          <TextLink href="https://github.com/awfulaxolotl">Github</TextLink>,{' '}
          <TextLink href="https://keybase.io/awfulaxolotl">Keybase</TextLink>,
          or{' '}
          <TextLink href="mailto:kevin.sullivan@axolotl.industries">
            email
          </TextLink>. Say hello!
        </P>
        <P>
          What you can't find here are any cave salamanders. No refunds – sorry,
          not sorry!
        </P>
      </Section>
    </Base>
  )
}

const sectionStyle: CSSProperties = {
  marginTop: `calc(${contentStartSpacing} / 2)`,
}

const h1Style: CSSProperties = {
  fontSize: largeHeaderSize,
  marginBottom: '-.3em',
  position: 'relative',
  zIndex: 1,
}

const P = glamorous.p({
  fontSize: '30px',
  fontWeight: 'bold',

  lineHeight: 1.45,

  margin: '0 auto 2em',
  maxWidth: articleTextWidth,
})

// A direct parent is needed on the texture for it to blend correctly
const Texture = glamorous.div(
  {
    position: 'relative',
    height: '210px',
    padding: '0 30px 0 20px',
    marginBottom: contentStartSpacing,

    background: `no-repeat left/cover url(${waveTexture})`,
    backgroundBlendMode: 'lighten',
    backgroundClip: 'content-box',
  },
  ({ theme }: { theme: Theme }) => ({
    backgroundColor: theme.bgColor,
  }),
)

const Portrait = glamorous.img({
  position: 'absolute',
  right: 0,
  bottom: '-30px',

  height: '300px',
  margin: 0,
})

const H = glamorous.span({
  ':nth-of-type( 3n )': {
    textShadow: deepTextShadow(4, 4, green),
  },
  ':nth-of-type( 3n+1 )': {
    textShadow: deepTextShadow(0, 4, red),
  },
  ':nth-of-type( 3n+2 )': {
    textShadow: deepTextShadow(-4, 4, purple),
  },
})

const Word = glamorous.span({
  display: 'inline-block',
  marginRight: '.5em',

  ':last-child': {
    margin: 0,
  },
})
