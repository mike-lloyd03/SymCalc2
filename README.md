# SymCalc2

A modern rewrite of [SymCalc](http://sym-calc.com/#)

## Purpose

This project serves two purposes:

1. Rewriting SymCalc. If it weren't for SymCalc, I probably wouldn't have graduated college. I used it extensively as I was pursuing my mechanical engineering degree and found it invaluable. I didn't need MatLab or Wolfram Alpha with this bad boy in my pocket. To this day, I still do it (albiet with much less complicated calculations.) However, I've noticed that it is no longer on the Google Play Store any longer. So I thought a simplified rewrite was due.

2. Everything should be rewritten in Rust. Linux. Fish shell. Reality. This project is a way for me to explore writing Android apps (which I have basically no idea how to do) in Rust. It uses [Mozilla's Uniffi](https://github.com/mozilla/uniffi-rs) library for generating FFI bindings from Rust to Kotlin and Swift. So all the business logic is written in the master programming language and all the UI tomfoolery can be in whatever lesser language is needed.

    I tried Flutter. Not a fan.
