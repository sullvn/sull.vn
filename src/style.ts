// Colors
export const white  = '#fffbf4'
export const black  = '#1b1d29'
export const green  = '#adcc66'
export const red    = '#ef5b5b'
export const purple = '#927bd0'


// Fades
export const fade          =  0.7
export const halfFade      =  0.35
export const doubleFade    =  0.85
export const quadrupleFade =  0.93


// Fonts
export const fontFutura   = '"futura-pt-bold", Futura, "Trebuchet MS", Arial, sans-serif'
export const fontPalatino = '"Palatino Linotype", "Book Antiqua", Palatino, serif'


// Sizes
export const baseWidth           = '50rem'
export const articleTextWidth    = '30rem'
export const borderWidth         = '.2rem'
export const contentStartSpacing = '8rem'
export const largeHeaderSize = '70px'


// Selectors
export const heading = 'h1, h2, h3, h4, h5, h6'
export const textContent = 'blockquote, dd, dl, dt, hr, li, ol, p, ul'
export const visualContent = 'img, figure, pre, svg, pre[class*="language-"]'


// Themes
export interface Theme {
  fgColor: string,
  bgColor: string,
}

export const darkTheme: Theme = {
  fgColor: white,
  bgColor: black,
}

export const lightTheme: Theme = {
  fgColor: black,
  bgColor: white,
}


// Functions
export function deepTextShadow( x: number, y: number, color: string ): string {
  const m = Math.max( x, y )

  let tss = []
  for ( let i = 1; i <= m; i += 1 ) {
    const xi = i * x / m
    const yi = i * y / m

    tss.push( `${ xi }px ${ yi }px 0 ${ color }` )
  }

  return tss.join( ',' )
}
