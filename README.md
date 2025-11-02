# ramidier
Akai Pro APC key 25 abstraction layer => R[ust] A[kai] midi [lay]er
Built as an abstraction layer using [midir](https://github.com/Boddlnagg/midir) and [midi-msg](https://github.com/AlexCharlton/midi-msg)

It should also work on other midi hardware but a lot of the biased data structures and methods won't then be available

It supports all the platforms/backends that *midir* supports using feature flags, check [here](https://github.com/Boddlnagg/midir?tab=readme-ov-file#features) for more info 

## Minimal example
Examples can be found in the examples folder.
You can execute them by cloning the repo and then
```bash
cargo run --example button_press
```
or
```bash
cargo run --example turn_all_pads_on
```
### Why aren't you using channels?!
I wanted to keep the most lightweight abstraction layer possible. Channels could be easily implemented on top.
