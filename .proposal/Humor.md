# Use of Humor

## The Governing Principle

Dry humor in technical writing works exactly like salt. You only notice it when it's missing or when someone dumped the shaker. One observation per chapter, maximum. It should feel like something a colleague said while walking past your desk — not a bit, not a routine.

The reader should finish the book and remember two or three of these lines. If they remember more than that, there were too many.

---

## Placement

Humor lands after hard sections, not during them. The reader just wrestled with something, it finally worked, there's a natural exhale. A dry observation in that moment works because the reader is already relaxed. The same line dropped in the middle of a dense concept explanation just interrupts.

**Rule:** place humor at the exhale, not the inhale.

---

## What Works

The target register is observations that are technically accurate and slightly sinister. Not jokes. Not setups with punchlines. Just true statements delivered with timing.

The structure that works: state a few true facts in sequence, let the last one land with an unexpected implication.

**After the profiler reveals the hot path in chapter 8:**
> "That function you wrote in chapter 1 and never thought about again is now responsible for 40% of your wall time. It's been there this whole time. Waiting."

"Waiting" is doing all the work. It makes the function sound like it has intentions. The reader knows that's absurd — and that's exactly why it lands.

**After the borrow checker finally accepts the code in chapter 2:**
> "Congratulations. You have now spent more time thinking about memory than most engineers do in a year. This will either horrify or delight your colleagues, depending on which colleagues."

**After implementing the full ITCH message enum in chapter 3:**
> "Nasdaq's protocol documentation runs to 84 pages. You needed about six of them. The other 78 cover edge cases you'll hopefully never see — and one appendix that appears to exist purely to humble you."

**After the compiler refuses the deliberate data race in chapter 9:**
> "You wrote a data race. The compiler wrote you a rejection letter. In C++ this same code would have compiled, run, and occasionally produced the wrong answer on days ending in Tuesday."

**After getting async fan-out working in chapter 10:**
> "Everything is running concurrently and nothing is on fire. This feeling will pass the first time you add a fourth consumer, but enjoy it for now."

**After Miri catches undefined behavior in chapter 11:**
> "Miri found it in under two seconds. You would have found it in production at 3am. This is the better outcome."

**After publishing the crate in chapter 12:**
> "Your code is on crates.io. Someone you'll never meet will depend on it within six months. Try not to think about that."

---

## What Doesn't Work

**Jokes that announce themselves:**
> "Rust lifetimes: making developers question their career choices since 2010! 😅"

That's a conference talk slide. The emoji alone is grounds for dismissal.

**Forced attribution:**
> "As they say in the HFT world — latency is just another word for 'you're fired.'"

The "as they say" construction is a tell that the author wrote the thing they're attributing to someone else.

**Humor aimed at the reader:**
Laughing at the material, the ecosystem, or the situation — fine. Laughing at the reader, even gently, curdles fast. The reader is the protagonist. They don't get mocked.

**Humor mid-concept:**
If the reader is in the middle of learning something hard, a joke reads as the author not taking the difficulty seriously. Save it for after they've come out the other side.

---

## The Frequency Rule

One dry observation per chapter, maximum. Maybe one per two chapters in the dense middle sections.

The reader should finish the book and remember two or three of these lines. If they remember more than that, there were too many. The tone is a peer who finds the situation occasionally absurd and says so once, briefly, then gets back to work.