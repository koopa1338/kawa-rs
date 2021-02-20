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

I've decided to use [druid](https://github.com/linebender/druid) as the gui
framework for this application as it seems to be very easy to use and provides
a modern architecture. I haven't tested this on other platforms so this needs
some more investigation at a later point.

At the moment I build a basic data- and projectstructure but there are no
elements that will be rendered. My main focus in next iterations is to persist
data, add urls as packages/parts and download the files to disk.

## License

[MIT License](./LICENSE.md)
