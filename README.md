<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD041 -->

SLURP: Silly Lua Interpreter
Fork of Lune designed to be used for linux utilities & programs.
rewriting readme is todo

A standalone [Luau](https://luau-lang.org) runtime.

Write and run programs, similar to runtimes for other languages such as [Node](https://nodejs.org), [Deno](https://deno.land), [Bun](https://bun.sh), or [Luvit](https://luvit.io) for vanilla Lua.

Lune provides fully asynchronous APIs wherever possible, and is built in Rust 🦀 for speed, safety and correctness.

## Features

- 🌙 Strictly minimal but powerful interface that is easy to read and remember, just like Luau itself
- 🧰 Fully featured APIs for the filesystem, networking, stdio, all included in the small (~5mb zipped) executable
- 📚 World-class documentation, on the web _or_ directly in your editor, no network connection necessary
- 🏡 Familiar runtime environment for Roblox developers, with an included 1-to-1 task scheduler port
- ✏️ Optional built-in library for manipulating Roblox place & model files, and their instances

## Non-goals

- Making programs short and terse - proper autocomplete / intellisense make using Lune just as quick, and readability is important
- Running full Roblox games outside of Roblox - there is some compatibility, but Lune is meant for different purposes

## Where do I start?

Head over to the [Installation](https://lune-org.github.io/docs/getting-started/1-installation) page to get started using Lune!
