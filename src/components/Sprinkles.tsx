import styled, { keyframes } from 'react-emotion'

import { green, purple, red } from '../style'

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
  },
  '43%': {
    content: '"▱"',
    color: red,

    top: '-.6em',
    left: '-.3em',
  },
  '56%': {
    content: '"◇"',
    color: purple,

    top: '-.3em',
    left: '-.6em',
  },
  '83%': {
    content: '"◜"',
    color: red,

    top: '.5em',
    left: '10%',
  },
})

const sprinklesBottomRight = keyframes({
  'from, to': {
    content: '"●"',
    color: red,

    right: '30%',
    bottom: '-1em',
  },
  '12%': {
    content: '"○"',
    color: purple,

    right: 0,
    bottom: '.3em',
  },
  '30%': {
    content: '"□"',
    color: red,

    right: '-.3em',
    bottom: '-.7em',
  },
  '47%': {
    content: '"▲"',
    color: purple,

    right: '-.4em',
    bottom: '.3em',
  },
  '68%': {
    content: '"■"',
    color: green,

    right: '45%',
    bottom: '-.6em',
  },
  '92%': {
    content: '"△"',
    color: green,

    right: '15%',
    bottom: '-.9em',
  },
})

const Sprinkles = styled('span')({
  position: 'relative',

  '::before, &::after': {
    content: '""',
    display: 'none',

    position: 'absolute',
    fontWeight: 'bold',
    fontSize: '1.5em',
  },

  '@media (hover: none)': {
    '::before, &::after': {
      display: 'none !important',
    },
  },

  ':hover': {
    '::before': {
      animation: `${sprinklesTopLeft} .8s .5s step-start infinite`,
      transform: 'rotateZ(-140deg)',

      display: 'block',
      left: 0,
      top: 0,
    },
    '::after': {
      animation: `${sprinklesBottomRight} 1.3s .5s step-start infinite`,
      transform: 'rotateZ(25deg)',

      display: 'block',
      right: 0,
      bottom: 0,
    },
  },
})

export default Sprinkles
