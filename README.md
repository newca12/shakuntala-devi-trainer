[![Actions Status](https://github.com/newca12/shakuntala-devi-trainer/workflows/Continuous%20integration/badge.svg)](https://github.com/newca12/shakuntala-devi-trainer/actions)
[![Coverage Status](https://coveralls.io/repos/github/newca12/shakuntala-devi-trainer/badge.svg?branch=main)](https://coveralls.io/github/newca12/shakuntala-devi-trainer?branch=main)
[![Crates.io](https://img.shields.io/crates/v/shakuntala-devi-trainer.svg)](https://crates.io/crates/shakuntala-devi-trainer)
[![Crates.io](https://img.shields.io/crates/d/shakuntala-devi-trainer.svg)](https://crates.io/crates/shakuntala-devi-trainer)
[![Documentation](https://docs.rs/shakuntala-devi-trainer/badge.svg)](https://docs.rs/shakuntala-devi-trainer)
[![](https://tokei.rs/b1/github/newca12/shakuntala-devi-trainer)](https://github.com/newca12/shakuntala-devi-trainer)
[![Crates.io](https://img.shields.io/crates/l/shakuntala-devi-trainer.svg)](https://github.com/newca12/shakuntala-devi-trainer/blob/main/LICENSE)

![Image](./screenshot.png?raw=true)

### About ###
[Shakuntala Devi][1]'s trainer is a brain training tool available as a GUI, a text console version and also an experimental [online][2] version thank to the cross platform GUI library [Iced](https://github.com/hecrj/iced).

shakuntala-devi-trainer is an EDLA project.

The purpose of [edla.org](http://www.edla.org) is to promote the state of the art in various domains.

### Installation ###

```
cargo install shakuntala-devi-trainer
```

### Usage ###
The goal is to determine [the day of the week](https://en.wikipedia.org/wiki/Determination_of_the_day_of_the_week) for a given date.  
The technic used by Shakuntala Devi is describe in her book [Figuring the Joy of Numbers](https://www.amazon.com/gp/product/8122200389).  
You can see an overview in Tibee's video [India's Human Computer](https://www.youtube.com/watch?v=4LHzUkfQ8oE&t=534s) or in this livejournal [entry](https://fiat-knox.livejournal.com/1067226.html) 

To launch the GUI 
```
shakuntala-devi-trainer
``` 

To launch the text console version
```
shakuntala-devi-trainer --cli
```

If you use the console version your answer should be encoded as an integer like described in the book like so :
* Sunday 0
* Monday 1
* Tuesday 2
* Wednesday 3
* Thursday 4
* Friday 5
* Saturday 6

If your answer is wrong only the text console version will give you a hint for now.  
Each hint is the result of a step of Shakuntala Devi's algorithm.

You can adjust the range of the random date with two handy sliders.

### Tips ###
In the Gregorian calendar, three criteria must be taken into account to identify leap years:
* The year must be evenly divisible by 4;
* If the year can also be evenly divided by 100, it is not a leap year;
* unless... The year is also evenly divisible by 400. Then it is a leap year.

According to these rules, the years 2000 and 2400 are leap years,
while 1800, 1900, 2100, 2200, 2300, and 2500 are not leap years.

### Web version ###
You can try the online version [shakuntala-devi-trainer][2]
or built it yourself :  
```
rustup target add wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli
cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/shakuntala-devi-trainer.wasm --out-dir shakuntala-devi-trainer  --web
```
Known issues :
* [Web canvas is stuck at fixed size](https://github.com/iced-rs/iced/issues/1265)
* [Some glitch with firefox](https://github.com/iced-rs/iced/pull/1096#pullrequestreview-866907637)

### Alternate systems for mentally calculating the day of the week for any given date. ###

https://mattbaker.blog/2020/04/26/mental-math-and-calendar-calculations/

### Developer Notes ###
* [Modulo of negative numbers shows languages in two different camps.](https://torstencurdt.com/tech/posts/modulo-of-negative-numbers)  
* Rust conveniently allow infinite lazy stream with iterator
* num-traits is required for Weekday::from_u32
* lazy_static is required to provide a singleton HashMap
* [By default Rust test programs hide the stdout of successful tests](https://stackoverflow.com/a/25107081)
* [Rust built-in test framework does not support parameterized tests](https://stackoverflow.com/questions/34662713/how-can-i-create-parameterized-tests-in-rust)

### License ###
© 2020-2022 Olivier ROLAND. Distributed under the GPLv3 License.

[1]: https://en.wikipedia.org/wiki/Shakuntala_Devi
[2]: https://edla.org/shakuntala-devi-trainer
