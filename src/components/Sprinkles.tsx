import { keyframes } from 'glamor'
import glamorous from 'glamorous'

import { green, purple, red } from '../style'


export const sprinklesClassName = 'sprinkles'


const sprinklesTopLeft = keyframes({
  'from, to': {
    content: '"◆"',
    color: green,

    top: '-.3em',
    left: '-.3em',
  },
  '13%': {
    content: '"◡"',
    color: green,

    top: '-.8em',
    left: '30%',
    transform: 'rotateZ(120deg)',
  },
  '43%': {
    content: '"▱"',
    color: red,

    top: '-.6em',
    left: '-.3em',
    transform: 'rotateZ(160deg)',
  },
  '56%': {
    content: '"◇"',
    color: purple,

    top: '-.3em',
    left: '-.6em',
    transform: 'rotateZ(205deg)',
  },
  '83%': {
    content: '"◜"',
    color: red,

    top: '.5em',
    left: '10%',
    transform: 'rotateZ(35deg)',
  },
})

const sprinklesBottomRight = keyframes({
  'from, to': {
    content: '"●"',
    color: red,

    right: '30%',
    bottom: '-1em',
    transform: 'rotateZ(245deg)',
  },
  '12%': {
    content: '"○"',
    color: purple,

    right: 0,
    bottom: '.3em',
    transform: 'rotateZ(315deg)',
  },
  '30%': {
    content: '"□"',
    color: red,

    right: '-.3em',
    bottom: '-.7em',
    transform: 'rotateZ(5deg)',
  },
  '47%': {
    content: '"▲"',
    color: purple,

    right: '-.4em',
    bottom: '.3em',
    transform: 'rotateZ(200deg)',
  },
  '68%': {
    content: '"■"',
    color: green,

    right: '45%',
    bottom: '-.6em',
    transform: 'rotateZ(134deg)',
  },
  '92%': {
    content: '"△"',
    color: green,

    right: '15%',
    bottom: '-.9em',
    transform: 'rotateZ(320deg)',
  },
})


const Sprinkles = glamorous.span( sprinklesClassName, {
  position: 'relative',

  '::before, &::after': {
    content: '""',
    display: 'none',

    position: 'absolute',
    fontWeight: 'bold',
    fontSize: '1.5em',
  },

  ':hover': {
    '::before': {
      animation: `${ sprinklesTopLeft } .8s .5s step-start infinite`,

      display: 'block',
      left: 0,
      top: 0,
    },
    '::after': {
      animation: `${ sprinklesBottomRight } 1.3s .5s step-start infinite`,

      display: 'block',
      right: 0,
      bottom: 0,
    },
  },
})

export default Sprinkles
