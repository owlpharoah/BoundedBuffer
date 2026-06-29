# Bounded Buffer Problem

## Whats The Bounded Buffer Problem

The bounded buffer problem refers to multithreaded situtation where threads called `producers` generate tasks onto a buffer whilst concurrently `consumer` threads consume said tasks from the buffer.

The fun lies in figuring out how to concurrently run the two without dataraces or deadlocks.

## What the code does

The code in the demo first defines a Buffer Queue and a maximum size for it. Since teh queue will be accessed (possibly) concurrently by the two types of threads, it must be made such that operation to it are `Atomic` in nature.
And thus, the Buffer remains wrapped around a `mutex` such that it provides exclusive access to one thread at a time.
We then define and spawn multiple (2 in our case) producers and consumers that will run concurrently.

### Referances
- [OSTEP](https://pages.cs.wisc.edu/~remzi/OSTEP/)
- [Rust Atomics & Locks](https://mara.nl/atomics/)
- Google Search (+ AI Suggestions)
