# Lightshow :traffic-light:

Make use of your Hue lights and set up a lightshow!

## About

This project stems from me learning Rust and trying to find a practical application
that would guide my learning.

A while back I created a Rust wrapper for the Hue lights called [lighthouse](https://github.com/finnkauski/lighthouse).
I then got sidetracked into writing a tool that connects my [midi drums with my lights!](https://www.youtube.com/watch?v=fEK2DofSwEE).
As part of that I decided that I wanted to somehow store mappings of light behaviour to the midi input sequences.
I wrote a really poor parser for a format and just left it. But having learned Haskell previously I always wanted to try
parser combinators.

This project is essentially me learning parser combinators by writing a scripting language that my
lights can interpret and that I can later map to my drums.