---
path: "/babbles/dot-waves-a-purescript-short-story"
date: "2016-02-02T00:37:48-08:00"
title: "Dot Waves: A PureScript Short Story"
---

If you haven't checked out [PureScript](http://www.purescript.org/), then you totally should. It's a spiritual child to Haskell, but currently focused for compiling to Javascript. Readable Javascript at that!

I'm not going to go in-depth about the language as a whole, as there are already [amazing learning resources](https://leanpub.com/purescript/read) out there. If you like what React, Redux, ClojureScript and Elm are doing, then take a look!

This post is a walk-through of an example PureScript project as its developed. It's intended as a realistic look at what the development train of thought is while creating a small project with PureScript. I just started with the language, and was surprised to see little resources like it. Hopefully it's of some use to you or maybe you just have a special relationship with wavy dots. Hey, no judgements here.

Let's go ahead and make some dots. Enjoy -- but not too much, please.

## Uno Dot

<p data-height="567" data-theme-id="21896" data-slug-hash="PZRLQV" data-default-tab="result" data-user="awfulaxolotl" data-embed-version="2" data-pen-title="Single Pulsating Dot" class="codepen">See the Pen <a href="https://codepen.io/awfulaxolotl/pen/PZRLQV/">Single Pulsating Dot</a> by Kevin Sullivan (<a href="https://codepen.io/awfulaxolotl">@awfulaxolotl</a>) on <a href="https://codepen.io">CodePen</a>.</p>

The best way to start is to start small. So let's start with a single pulsating dot on a canvas.

First the HTML.

```html
<!DOCTYPE html>
<html>
  <head>
    <title>Dot Waves</title>
  </head>
  <body>
    <canvas width="800" height="500" id="dot-waves"></canvas>
    <script src="/app.js"></script>
  </body>
</html>
```

Phew, so far nothing to mess up. Looking good, looking good. But a blank canvas is a boring canvas, so let's make some magic.

```haskell
main :: forall eff. Eff ( timer :: Timer, dom :: DOM, canvas :: Canvas | eff) Unit
main = do
  Just canvas <- getCanvasElementById "dot-waves"
  ctx <- getContext2D canvas
```

The first step is to get the HTML canvas context object. PureScript is a functional language which values purity, so functions which touch the outside world are wrapped in the `Eff` context. That's why the type (return type in imperative languages) of the main function is `Unit` (nothing) but wrapped in `Eff`. To unwrap a value in `Eff` is to commit to touching the outside world and giving up total purity.

Bummer. Purity sounds kinda nice, but as my dad once said, "You gotta get a little dirty to get anything done, kid." Actually he never said that, but don't let the truth get in the way of a relevant anecdote.

Anyways, we have a canvas context now. Let's take a pit stop in Dot Land and write a function to render a dot, given its coordinate and radius.

```haskell
renderDot :: Number -> Number -> Number -> Drawing
renderDot x y r = 
  filled (fillColor color) dot
  where
    dot :: Shape
    dot = circle x y r

    color :: Color
    color = rgb 200.0 220.0 210.0
```

So we have a function which takes three numbers (x, y, radius) and returns a drawing of a dot. `filled` is a function from the [purescript-drawing](https://github.com/paf31/purescript-drawing/blob/master/docs/Graphics/Drawing.md#filled) library which takes a fill style and shape to create a drawing. Nifty!

What's next? We have the canvas, the dot... oh! The *pulsating*, of course. Let's pulse. The easiest way is to make pulsating a function of the current time. If you imagine the two-dimensional graph of such a function, it would look like a sin wave. So let's use the `sin` function!

```haskell
pulse :: forall eff. Number -> Number -> Eff (dom :: DOM, timer :: Timer | eff) (Signal Number)
pulse min max = do
  nowMs <- animationFrame
  return $ nowMs ~> amp
    where secs ms = ms / 1000.0
          unitAmp x = ((sin x) + 1.0) / 2.0
          amp ms = (unitAmp $ secs ms) * (max - min) + min
```

I know what you are thinking, "That's no simple sin function, what's with all that extra crap?" Sorry. Don't send me an angry email just yet; please let me explain!

First, checkout the `amp` function which is defined at the bottom. It takes a number of milliseconds since the [Unix epoch](https://en.wikipedia.org/wiki/Unix_time) and returns the pulsating value at that instant.

Great, so what's with the extra crap. Well, actually that "extra crap" has a name thank-you-very-much. Her name is "signals" and she is what enables us to consider all pulsating values now and in the future.

Say wut.

Bear with me here. Imagine an infinite queue of data, where you can't be sure when the next item will become available. That's a signal in a nutshell, but I guess real nutshells can't contain infinite things. Whatever, you got me.

So now that we know what a signal is, let's take a step back and review what we wanted in the first place. We wanted a dot which pulses over time. If we had a signal which an infinite list of Unix epoch times, we could apply the `amp` function to every element to get the infinite list of pulse values at every moment in time. We could simply consider the latest item fed to us in this "pulse signal" to get the current pulse value.

And that's exactly what's happening here:

```haskell
nowMs <- animationFrame
return $ nowMs ~> amp
```

We use `animationFrame`, a signal of Unix epoch times with values fed to us at the screen refresh rate. `nowMs ~> amp` is the `amp` function being mapped to every element in this infinite signal. We then return the "pulse signal" in the context of `Eff`, as we had to touch the browser environment to ask for `animationFrame`.

Everything's going so well! We've got the canvas context, a dot renderer, the pulse value over time, and had a minor existential crisis over nutshells to boot. Now it's time to link everything together.

```haskell
rendering :: forall eff. Context2D -> Number -> Eff (canvas :: Canvas | eff) Unit
rendering ctx n = do
  clearRect ctx boundaries
  render ctx $ renderDot 400.0 250.0 n
```

`rendering` takes the canvas, a moment in time, and renders a dot on the canvas with a guest appearance from `renderDot`.

```haskell
main :: forall eff. Eff ( timer :: Timer, dom :: DOM, canvas :: Canvas | eff) Unit
main = do
  Just canvas <- getCanvasElementById "dot-waves"
  ctx <- getContext2D canvas
  p <- pulse 30.0 100.0
  runSignal $ p ~> (rendering ctx)
```

This updated main function takes a pulse signal, between 30.0 and 100.0, and runs the renderer on every element when it comes available.

And that's how you render a pulsating dot, folks. Checkout the [full source](https://github.com/bitantics/dot-waves/blob/c4eeb67378f97dc4c3fdd43cae39a6a6d7ded31c/src/Main.purs).

## More Dots, Please

<p data-height="575" data-theme-id="21896" data-slug-hash="wMmOxL" data-default-tab="result" data-user="awfulaxolotl" data-embed-version="2" data-pen-title="Rectangle of Dots" class="codepen">See the Pen <a href="https://codepen.io/awfulaxolotl/pen/wMmOxL/">Rectangle of Dots</a> by Kevin Sullivan (<a href="https://codepen.io/awfulaxolotl">@awfulaxolotl</a>) on <a href="https://codepen.io">CodePen</a>.</p>

You aren't happy with one dot? *You want more?!*

Okay, let's do it. Specifically, let's make a rectangle of pulsating dots in a grid.

This is a simple change, but it highlights a huge shift in thinking from an imperative to functional programming mindset. I liken it to changing your thought process from thinking in instructions to thinking about a data flow, made of transformations in a pipeline.

In the imperative world, you might write a for loop, iterating to each dot and rendering them as you go.

Not so in the functional world. We want to start with a seed of data, then continually add sugar and spice until we have baked up a proper set of delicious dots.

```haskell
places :: Array { x :: Int, y :: Int }
places = (\x y -> { x, y }) <$> 0 .. (xn - 1) <*> 0 .. (yn - 1)
```

The places within the rectangular grid is a decent place to start. It's an array of (x, y) coordinates using PureScript's records, a sort of static key-value map.

Now this is where things get really interesting. It may not look like it, but there are a few intriguing mechanisms happening here. The best way to explain what's going on is by walking through how this expression is evaluated.

It starts with `xn` and `yn`, the grid's width and height respectively. We want to create the full array of width and height coordinates, so we use the infix `..` operator to create those. The snippet above evaluates into:

```haskell
places = (\x y -> { x, y }) <$> [ 0, 1, ..., xn - 1 ] <*> [ 0, 1, ..., yn - 1 ]
```

Next up, let's checkout that sweet, sweet lambda function and that... `<$>`? As it turns out, that's just the infix operator version of your typical map function -- but with a twist.

Functional programmers like to generalize things, so it shouldn't come to much surprise they generalized the good old map function too. Instead of operating on just lists, it will operate on anything which defines how to traverse itself. Nifty, but not relevant here so let's move on.

In an alternative Javascript universe, it would look like this:

```javascript
// In Javascript
[ 0, 1, ..., xn - 1 ].map( x => y => ({ x, y }))
```

```haskell
-- In PureScript
(\x y -> { x, y }) <$> [ 0, 1, ..., xn - 1 ]
```

If you look closely enough, you may spot something kinda odd. The dreaded *double lambda!* Nah, I'm joking. It's not dreaded; it's actually loved! It's function which returns a function, and that means you can partially apply its arguments. It's called [currying](https://en.wikipedia.org/wiki/Currying) and it's awesome. All functions in PureScript are curried by default.

Let's checkout what happens when we evaluate the above Javascript statement alongside its PureScript expression counterpart.

```javascript
// In Javascript
[
  y => ({ x: 0, y }),
  y => ({ x: 1, y }),
  ..,
  y => ({ x: xn - 1, y })
]
```

```haskell
-- In PureScript
[
  \y -> { x: 0, y },
  \y -> { x: 1, y },
  ...,
  \y -> { x: xn - 1, y }
]
```

Woah, they look almost identical! It turns out there's a simple reason: Javascript recently took inspiration from functional languages for it's lambda function. Good ideas tend to permeate between all languages.

Okay, back to our original function:

```haskell
places :: Array { x :: Int, y :: Int }
places = (\x y -> { x, y }) <$> 0 .. (xn - 1) <*> 0 .. (yn - 1)
```

We now know it's equivalent to this:

```haskell
places = [ \y -> { x: 0, y }, ... ] <*> [ 0, 1, ..., yn - 1 ]
```

So, what the heck is that `<*>` operator? Remember how functional programmers like to generalize things? Turns out those guys and gals generalized function application itself as well. *Oh them*.

Specifically, function applications are generalized between contexts. In this case, a list of functions knows how to apply itself to a list of arguments. Let's complete the evaluation and checkout the final value!

```haskell
places = [ { x: 0, y: 0 }, { x: 0, y: 1 }, ..., { x: 1, y: 0 }, ..., { x: xn, y: yn } ]
```

I'm not sure about you, but I feel like I just witnessed something beautiful. By abstracting function mapping and application, we wrote -- in my opinion -- really clear and concise code. Sure, the mechanics behind the abstractions are a little involved. But once you learn them, they become second nature. Look 'ma, no more for-loops!

Okay, okay, so we've come pretty far. What now? Well, remember how I mentioned seeds of data, pipelines, and some other crap? The list of places is our seed of data. Now we just have to add a little extra spice to get our dots. Let's go!

```haskell
renderDots :: Int -> Int -> Number -> Number -> Drawing
renderDots xn yn spacing diameter = fold dots
  where 
    places :: Array { x :: Int, y :: Int }
    places = (\x y -> { x, y }) <$> 0 .. (xn - 1) <*> 0 .. (yn - 1)

    coord { x, y } = { x: x' * spacing, y: y' * spacing }
      where x' = toNumber x
            y' = toNumber y

    coords :: Array { x :: Number, y :: Number }
    coords = coord <$> places

    dot { x, y } = renderDot x y (diameter / 2.0)
    dots = dot <$> coords
```

`fold dots` is the actual body of the function, where everything else after `where` are intermediate values.

First we make our initial places, which we have already gone over. Great, moving on.

Using the `coord` function, we map over the places by multiplying them by a constant. This spaces them apart, creating our final pixel coordinates.

Afterwards, we need to turn these pixel coordinates into dots. So, let's map over our coordinates, using our existing `renderDot` function. *Boom bada bing,* we got an array of rendered dots.

However, there's one more nugget of intrigue in that code snippet. Can you spot it? Checkout `fold dots`. It's your typical fold/reduce function, but with only one argument? How do you fold an array with no fold function?

I'll give you a moment to guess... okay, moment over.

Generalization! In this case, combination is generalized. The `Drawing` type (what a dot is) has defined how to combine members of itself. This is called a [monoid](https://en.wikipedia.org/wiki/Monoid). Therefore the fold function simply asks the accumulated dot drawing to combine itself with the next dot in the array.

One of the great things with PureScript, and most functional languages for that matter, is how it attempts to climb the tower of programming abstraction. By combining several generalities, you can produce extremely concise, correct, and flexible code.

Abstract on, my fellow programmer. Abstract on.

That's it for this chapter, but feel free to checkout the [final code](https://github.com/bitantics/dot-waves/blob/bc80d29380c3a0b09d5917a440e6b552f5678d7e/src/Main.purs) for this feature.

## Make Some Waves

<p data-height="300" data-theme-id="21896" data-slug-hash="bEvZzb" data-default-tab="html,result" data-user="awfulaxolotl" data-embed-version="2" data-pen-title="Colored Waves" class="codepen">See the Pen <a href="https://codepen.io/awfulaxolotl/pen/bEvZzb/">Colored Waves</a> by Kevin Sullivan (<a href="https://codepen.io/awfulaxolotl">@awfulaxolotl</a>) on <a href="https://codepen.io">CodePen</a>.</p>

So far this short story has been a lie. The title promised dot waves; not pulsating dots! Without a doubt, it's the greatest tragedy of our time. Let's fix it and make some waves!

Conveniently, most of the logic to create waves is already made. We have a flow of time, pulsating, and a composable render function. If each dot offsets its timer according to its location in the grid, then a wave-like pattern should occur. Let's check it out.

```haskell
renderDot :: Int -> Int -> Number -> Drawing
renderDot x y t = 
  filled (fillColor dotConfig.color) dot
  where
    xp = (toNumber x) * dotConfig.separation
    yp = (toNumber y) * dotConfig.separation

    xf = (toNumber x) / (toNumber dotConfig.horizontal)
    yf = (toNumber y) / (toNumber dotConfig.vertical)

    t' = t + xf * yf
    unitAmp = ((sin t') + 1.0) / 2.0
    diameter = (pow unitAmp 3.0) * (dotConfig.maxSize - dotConfig.minSize) + dotConfig.minSize

    dot :: Shape
    dot = circle xp yp (diameter / 2.0)
```

The new `renderDot` function has taken some logic out of the `renderDots` function defined in the previous chapter. This gives us more control on a per-dot basis, as we simply take the grid coordinate and current time (in milliseconds since the Unix epoch) to return a drawing. The `renderDots` wrapper simply composes these together to form the complete picture at any given moment.

```haskell
t' = t + xf * yf
```

The core of our changes happens to be the simplest. We literally define a new time, `t'`, by using the actual time plus an offset. The offset is dependent on the x and y location by design. This will create a sort-of wave effect from the bottom-right corner to the top-left.

And, well, that's just about it! The offsetted time and the refactoring make up the entirety of the changes. Check out [the source](https://github.com/bitantics/dot-waves/blob/66bd7f887b59fb7cdf6617a88cd911c2f7f32fe6/src/Main.purs) at this point.

## Wax On; Wax Off

<p data-height="522" data-theme-id="21896" data-slug-hash="jWzJJW" data-default-tab="result" data-user="awfulaxolotl" data-embed-version="2" data-pen-title="Polished Look & Feel" class="codepen">See the Pen <a href="https://codepen.io/awfulaxolotl/pen/jWzJJW/">Polished Look & Feel</a> by Kevin Sullivan (<a href="https://codepen.io/awfulaxolotl">@awfulaxolotl</a>) on <a href="https://codepen.io">CodePen</a>.</p>
<script async src="https://production-assets.codepen.io/assets/embed/ei.js"></script>

So we have our dots, our waves, and now we just need to make it look good. I'm thinking of a sort of 80's Miami nightlife feel, you follow me? No? Well you don't really have a choice, Ms. Passive Reader. Sorry.

So starting with your favorite color choice, let's come up with a way to have pulsating colors. With our refactored `renderDot` function, it should be easy to customize the dot colors to our liking. The crucial part of the implementation is how we calculate color components ([RGB](https://en.wikipedia.org/wiki/RGB_color_model)).

```haskell
clrCmp n off = off + ((unitAmp (t' + n)) * (200.0 - off))
color = rgb (clrCmp 0.0 40.0) (clrCmp 0.66 25.0) (clrCmp 0.33 25.0)
```

We want to synchronize the color with the wave at any given dot, so we need to use `t'` again. `clrCmp` is just the logic for a single color component. We have a time offset and an end-result offset to tweak per component behavior and maximums. Tweaking the `off` variable changes how saturated the colors become, while `n` changes how offset the component is from the other components.

We may want all components to change the same way at the same time, but this would result in a mostly grey/brown color if all components are even. The constants used in the `color` binding on the second line were hand tweaked to our favorite color scheme.

Now that we have the color, it's time to tweak the rest of the experience. Light colors on white don't evoke that Miami feel, so let's make the background black. Then for good measure, let's make the canvas bigger. As big as it can be!

```html
<!DOCTYPE html>
<html>
  <head>
    <title>Dot Waves</title>
    <style>
      html {
        background-color: black;
        height: 100%;
      }
      body {
        display: flex;
        align-items: center;
        justify-content: center;
        height: 100%;
      }
      canvas {
        max-height: 100%;
        max-width: 100%;
      }
    </style>
  </head>
  <body>
    <canvas width="1400" height="800" id="dot-waves"></canvas>
    <script src="/app.js"></script>
  </body>
</html>
```

Using the standard `html` selector for the background totally works because the canvas has a transparent background by default. In theory, we could use any standard CSS abilities in the background to spruce up the experience. But, there's just something about pure black, you know?

This is a pretty good place to end this journey. We've got dots, waves, colors, and a little polish as well. Feel free to visit the [final version](https://github.com/bitantics/dot-waves/blob/add2744f685262cfd4d193434c9b5f647dbcb9f1/src/Main.purs) of the source code.

Thank you for reading!
