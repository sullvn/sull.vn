---
path: "/babbles/introducing-tastes"
date: "2018-08-08T18:08:01.385Z"
title: "Introducing Tastes and Rapid Prototyping with Flavor"
---

<img src="face-boxes.png" />

&nbsp;

I've made a Typescript library for intelligent sample generation during creative prototyping. It's called Tastes.

[Check it out on GitHub!](https://github.com/awfulaxolotl/tastes)

# A Quick Bite

Let's say we're creating a minimal poster. No — a _giant_ minimal poster because we have something to say.

Let's use Tastes to quickly check out different combinations of font sizes and colors.

The code below is also available for interactivity [on Runkit](https://runkit.com/awfulaxolotl/a-quick-bite-of-tastes).

```js
// 1. Define the variables we want to play with
import { integer, record } from 'tastes'

const hue = integer({ min: 0, max: 360 })
const poster = record({
  fgHue: hue,
  bgHue: hue,
  headerPt: integer({ min: 20, max: 32 }),
  bodyPt: integer({ min: 12, max: 20 }),
})

// 2. We can checkout the specific poster design at
//    `(0.4, 0.2, 0.75, 0)` in the sample space.
console.log(poster([0.4, 0.2, 0.75, 0]))

// 3. But that's too manual. Let's just ask for 30 random
//    sample poster designs.
import { sampleRandom, take } from 'tastes'

for (const s of take(30, sampleRandom(poster))) {
  console.log(s)
}

// 4. But random designs may not be the best examples.
//    Let's check out carefully selected "representative"
//    samples instead.
import { sampleBatch } from 'tastes'

// Use detail of order 3
for (const s of sampleBatch(poster, 3)) {
  // `console.log` is used in abscence of
  // a proper rendering function
  console.log(s)
}
```

# The Humble Hypothesis

A happy bunch of Javascript random data generators already exist. Such as [faker](https://github.com/marak/Faker.js/), [casual](https://github.com/boo1ean/casual), and [chance](http://chancejs.com/) [](https://github.com/jsverify/jsverify)— plus [jsverify](https://github.com/jsverify/jsverify) for property-based testing.

So, why yet another library for data generation?

Well, I wanted something to facilitate creative prototyping. I’m tired of tweaking different variables and settings to get what I want. So let’s just ask for our computers' help!

Random data generators just don’t cut it for this.

You can experiment with how your code reacts to random data, but it will be an aimless pursuit. It’s the equivalent of button mashing on a video game controller. Will you win? Maybe. But will you know why? No!

So that’s where Tastes comes in. It’s a more intentional foundation for new methods in creative prototyping. The hypothesis is large productivity benefits if example-driven prototyping is used to its fullest extent.

&nbsp;

<img src="faces-conversation.png" />

# The Plan

Tastes is currently not much more than an assortment of data generation functions. Not exactly useless, but also not exactly exciting.

The idea is to expand Tastes out to a complete foundation for some pretty rad developer tooling. Think of a composable toolkit, a la [D3](https://d3js.org), but for generative prototyping.

Here's a rough outline of what's coming down the pipe:

- Components for interactive exploration of sample spaces.
- Plug and play for React components with prop types.
- Auto-generate unit test cases from Typescript functions.
- Auto-generate infinite examples for React Storybook.
- A richer library of pre-made sample spaces. This can include geospatial types, colors, lorem ipsum, shapes, data structures, time series, and more.
- Pick and choose sample space exploration.

# A Generative New World

Make no mistake; Tastes is not being created in a vacuum.

It's a part of the growing field of higher-level software interfaces. "Higher-level" being abstract thinking with "lower-level" thought focusing on minutiae and essentials.

Human-computer interfaces have progressively ascended to higher levels over time. We started with mainframes where users needed to enter low-level code into punchcards. Now we can ask for GIFs with our voice.

Progress!

<div>
  <video src="https://zippy.gfycat.com/ActualBelatedAmbushbug.mp4" autoplay loop muted></video>
</div>

Wait - but hold on. As it turns out, progress isn’t universal.

Creative tooling hasn't fundamentally changed since [The Mother of All Demos](https://www.youtube.com/watch?v=M5PgQS3ZBWA) in 1968. Sure, we use mice and graphics now instead of the command line, but our level of expression hasn’t changed. We still operate with pixels in Photoshop, letters in Word, and notes in Ableton.

Okay, pixels and the like aren't going away. Of course not — they're the building blocks of their respective media. A novel without letters isn't a novel and a picture without pixels isn't a picture.

But we humans don't think in letters and pixels. We express ourselves with narratives, metaphors, emotions, moments, and messages.

Software should help us work at that level. Who cares about pixel-tweaking if we can simply ask for a dragon in our profile picture? Who would worry about spelling in a world where you can ask for infinite valid alternative versions of your essay?

&nbsp;

<img src="dragon-love.jpg" />

&nbsp;

And we’re getting there. Just a few examples are the following:

- [**Project Dreamcatcher.**](https://autodeskresearch.com/projects/dreamcatcher) Generate parts for industrial designs from constraints.
- [**Magenta.**](https://magenta.tensorflow.org/) Creating music and art from samples and styles.
- [**The Grid.**](https://thegrid.io/) Derive websites from content.
- [**René.**](https://jon.gold/2016/06/declarative-design-tools/) Interactively explore possible design combinations.

In fact, Tastes could be considered an evolution of René. It generalizes the idea for more data types and integrates it into a software development workflow.

# Alright, Then

Check out Tastes when you get a chance!

It's just an NPM install away:

```sh
npm install tastes --save-dev
# or `yarn add tastes --dev`
```
