import * as React from 'react'
import styled, { keyframes } from 'react-emotion'
import { lighten } from 'polished'
import { range } from 'ramda'

import { red } from '../style'

interface Disc3DProps {
  style?: React.CSSProperties
  className?: string
}

const sides = 40
const radiusEm = 2.5
const depthEm = 0.3
const edgeFaceWidthEm = 2 * radiusEm / Math.tan(0.5 * Math.PI - Math.PI / sides)

export default function Disc3D(props: Disc3DProps): JSX.Element {
  return (
    <Container {...props}>
      <Disc>
        {range(0, sides).map(n => <EdgeFace key={n} side={n} />)}
        <SideFace transform={`translateZ( ${0.05}em )`} />
        <SideFace transform={`translateZ( ${depthEm - 0.05}em )`} />
      </Disc>
    </Container>
  )
}

const Container = styled('div')({
  position: 'relative',

  width: `${2 * radiusEm}em`,
  height: `${2 * radiusEm}em`,

  perspective: '12em',
})

const Disc = styled('div')({
  position: 'absolute',
  top: '50%',
  left: '50%',

  transformStyle: 'preserve-3d',

  animation: `43s linear 0s infinite normal ${keyframes({
    from: {
      transform: 'rotateX( 1.3turn ) rotateZ( 0.9turn ) rotateZ( 1.77turn )',
    },
    to: {
      transform: 'rotateX( 0.3turn ) rotateZ( 1.9turn ) rotateZ( .77turn )',
    },
  })}`,
})

interface SideProps {
  transform: string
}

const SideFace = styled<SideProps, 'div'>('div')(
  ({ transform }: SideProps) => ({
    transform: `rotateX( .25turn ) ${transform || ''}`,
  }),
  {
    margin: `
    -${radiusEm}em
    -${radiusEm}em
    -${radiusEm * 2}em
  `,

    height: `${2 * radiusEm}em`,
    width: `${2 * radiusEm}em`,
    borderRadius: `${radiusEm}em`,

    backgroundColor: red,
  },
)

interface EdgeProps {
  side: number
}

const EdgeFace = styled<EdgeProps, 'div'>('div')(
  ({ side }: EdgeProps) => ({
    transform: `rotateY( ${side / sides}turn ) translateZ( ${radiusEm}em )`,
  }),
  {
    margin: `
    -${depthEm / 2}em
    -${edgeFaceWidthEm / 2}em
    -${depthEm}em
  `,
    width: `${1.1 * edgeFaceWidthEm}em`,
    height: `${depthEm}em`,

    backgroundColor: lighten(0.08, red),
    backfaceVisibility: 'hidden',
  },
)
