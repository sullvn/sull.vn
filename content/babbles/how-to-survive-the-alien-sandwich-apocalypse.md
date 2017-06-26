+++
date = "2015-10-29T22:42:48-07:00"
draft = false
title = "How to Survive the Alien-Sandwich Apocalypse with Constraints"

+++

> Imagine you need to instruct an alien how to make a peanut butter and jelly sandwich. The alien understands English and basic concepts, but doesn't know any of the steps required to make a sandwich. You are in a kitchen with them, but cannot physically help.

> Go.

You may have some questions, such as (but not limited to):

* What are the alien's nutritional needs? Is it allergic to peanut butter?
* How is instructing it to make a peanut butter jelly sandwich more pertinent than calling the authorities?
* Does the alien deserve chunky peanut butter?
* What has the alien done to you, where you cannot physically help?!

Unfortunately, I am just as confused as you. This was the prompt given to me in elementary school to explain the concept of algorithms. Given this, most students provided an answer in some form of story telling, such as the following:

The Imperative Way
------------------

1. Gaze upon the four bread slices, open peanut butter, open jelly, and the knife
2. Yes, crunchy peanut butter is clearly the best
3. Hold the knife by the smooth end in your dominant appendage
4. Place the sharp end of the knife within the peanut butter
5. Scoop out some peanut butter, about a tablespoon
6. Scrap side of knife with peanut butter against a slice of bread
7. Spread until peanut butter is almost completely off the blade
8. Yadda, yadda whatever I'm not in elementary school anymore so I don't have to finish

Hopefully that makes sense, otherwise I'm definitely not going to survive the next alien invasion.

I'm going to throw a question out there. Feel free to think about it -- or not, I'm not your mother:

*Is there a different way to answer this?*

If you have ever had to indulge people and their rhetorical questions, then you know it's a "yes"!

The Lawful Method
-----------------

* Only smooth things are held
* Yes, crunchy peanut butter is clearly the best
* Knives are used for scooping, applying, and cutting
* Bread must have air on one side; spread on the other
* Jelly and peanut butter must be used in a one-to-one ratio

Notice the difference? Laws (or constraints) were declared, which hopefully restricts the amount of valid solutions to the correct ones.

I believe this is the better solution, but feel free to disagree. Either solution won't keep you alive if the aliens hate PB&Js!

However in case they do, the benefit comes down to how the fussy details are abstracted away and forces the instructor to dig into the essence of the problem. In the example, the definition of a PB&J sandwich and a knife is embedded in the constraints.

Fussy details, such as each individual spreading, are left out. This leaves less room for silly errors, plus it covers every alien-sandwich scenario. Awwww yeah, we are safe in multiple universes. Go us!

But, Let's Get Real
-------------------

Aliens and sandwiches are great, but *what does it all mean?*

A really shitty assignment, contributing to immense boredom in a roomful of eighth-graders.

Or it means defining and creating safer systems, with less error, faster. *Say what?*

### First, take your unit testing to infinity and beyond

If you are a programmer, then you know software needs to be tested to be considered high-quality. What you *don't* know is what unit tests to write.

Yep, I said it. You don't know what unit tests to write. Nothing against you and your mad skillz at all, it's just that you are human. Writing imperative test cases to prove your program is correct is just fool-hardy, *and* can be a lot of work!

So, what is there for a tired programmer to do? Declare some universal truths, generate test data, and try to find a counter-example! If this sounds like declaring constraints, it's because that's what it is.

Haha! I set you up, just to prove a point! Sorry -- but not really.

Check it out in Haskell ([thanks to RealWorldHaskell.org](http://book.realworldhaskell.org/read/testing-and-quality-assurance.html)):

```haskell
qsort :: Ord a => [a] -> [a]
qsort []     = []
qsort (x:xs) = qsort lhs ++ [x] ++ qsort rhs
    where lhs = filter  (< x) xs
          rhs = filter (>= x) xs

prop_idempotent xs = qsort (qsort xs) == qsort xs
prop_minimum    xs = head  (qsort xs) == minimum xs
```

[Quicksort](https://en.wikipedia.org/wiki/Quicksort) is defined above, with two universal truths as properties:

1. **Idempotent.** An already sorted collection doesn't change after another sort.
2. **First element is the minimum.**

The [QuickCheck](https://hackage.haskell.org/package/QuickCheck) library used above can generate an arbitrary amount of test cases to check the Quicksort implementation is valid. However, usually no more than a few hundred cases are required.

### A whole new world

Declaring universal laws can give you many more super powers beyond just generating test cases. I promise.

Just a few examples:

* [SQL](https://en.wikipedia.org/wiki/SQL) and other query languages include logical constraints a user may include. It seems rudimentary now, but this is huge. The database may fulfill the constraints in any way it sees fit, as long as the result is correct. This abstraction has contributed to SQL's sustained gargantuan popularity for decades.
* [Leslie Lamport: Thinking Above the Code](https://www.youtube.com/watch?v=-4Yp3j_jk8Q), where Leslie talks about verifying logic with [TLA+](https://en.wikipedia.org/wiki/TLA%2B) to truly know your program is logically sound. Includes a great deconstruction of Quicksort.
* Documenting and designing program behavior can benefit from assuming some properties which should always hold. As per the alien-sandwich example, it is much more concise, clear, and error-proof.

Hopefully this has convinced you there is more than one way to describe logic, and therefore whole systems. By elaborating on all possibilities, it gives the writer a better understanding of what they are designing.

And hey, it might even prevent a bug or two. Try it out!