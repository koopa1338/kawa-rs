## Kawa-rs

If it comes to download manager for on click hoster, there are not many options
that are easy to install, convenient to use and fast at the same time. Most
used options are pyload that is in heavy refactoring at the moment and
JDownloader that is feature rich (maybe even bloated) and written in java. I am
using JDownloader till this application is in a usable state but
the startup times of JDownloader were the reason for this project.

I've decided to use rust as it is fast on the one hand and has great dependency
management on the other. Also this is a great oppertunity to learn rust in
a real project that I use in the future. Keep in mind that I consider myself
very new to rust and am constantly learning more about the language. Mistakes
will be made so be aware. Main focus is easy configuration of premium accounts
and support for the most common one click hoster, maybe a generic way to
support them can be realised.

### GUI
There will be no usable gui till the core functionality is implemented.

I've evaluated different gui libraries like druid, ice and gtk but they have
all different problems I don't want to deal with in my application. The easiest
way to get a good looking and cross plattform gui seems to be with egui. There
are also some features that are currently in development or I have to implement
myself (like a decent [table widget](https://github.com/emilk/egui/issues/296)).

### TODO
egui comes with a persistence feature so I don't have to deal with implementing
this myself. At the moment I build a basic UI as a prototype to test usability.

## License

[MIT License](./LICENSE.md)
