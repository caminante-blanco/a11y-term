# a11y-term

## Background
What is the point of this piece of software? Well, I really wanted to make something interesting for my personal website, and that pursuit took me here. I think that websites are something like gardens. You're going to be the only one who sees it 98% of the time, but here is a certain joy that comes from making it yours and taking the time to mold it to your preferences.

I wanted something interesting, and I have been experimenting with learning Rust, and WASM has fascinated me since I first heard of it. My original attempt at building a personal website was to compile egui/eframe to WASM, but that was so clunky, and came with a lot of antipatterns I did not like. First and foremost, the lack of anything but a canvas element was a major obstacle to accessibilty, and accessibilty was at the forefront of my mind. Additionaly, while I liked the fact that my website was for all intents and purposes a desktop app, the cool factor wasn't significant enough to justify the complexity over just making a basic website. 

So what did i do to alleviate these issues? Doubled down of course. That's how I decided to build a full terminal emulator with ARIA support soley as a backend to my personal website.
