# Advent of Code
These are my solutions to the advent of code. I've been doing advent of code even before this public github repo. I mainly do advent of code to learn new languages.

## Benchmarks
Benchmarks are run automatically every time the main function is called. The best overall
time is recorded in the table below. Benchmarks do not include loading data from disk but do
include any kind of transformation on the input. Sometimes part 2 has to repeat steps already
taken in part 1. This leads to a slightly slower combined time of both parts. These benchmarks
were executed on an AMD Ryzen 7 7700X.
### 2023 (Rust)
<!-- SOT2023 -->
| Day | Best Time Part 1 | Best Time Part 2 | Code |
|---|---|---|---|
| 1 | 72.2μs <!-- 72200 --> | 71.3μs <!-- 71300 --> | [day01.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day01.rs) |
| 2 | 10.3μs <!-- 10300 --> | 9.4μs <!-- 9400 --> | [day02.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day02.rs) |
| 3 | 233.8μs <!-- 233800 --> | 158.1μs <!-- 158100 --> | [day03.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day03.rs) |
| 4 | 216.2μs <!-- 216200 --> | 159.8μs <!-- 159800 --> | [day04.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day04.rs) |
| 5 | 26.2μs <!-- 26200 --> | 40.0μs <!-- 40000 --> | [day05.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day05.rs) |
| 6 | 700ns <!-- 700 --> | 500ns <!-- 500 --> | [day06.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day06.rs) |
| 7 | 1.241ms <!-- 1241400 --> | 1.281ms <!-- 1281200 --> | [day07.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day07.rs) |
| 8 | 330.9μs <!-- 330900 --> | 1.539ms <!-- 1539200 --> | [day08.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day08.rs) |
| 9 | 121.2μs <!-- 121200 --> | 99.4μs <!-- 99400 --> | [day09.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day09.rs) |
| 10 | 140.9μs <!-- 140900 --> | 674.7μs <!-- 674700 --> | [day10.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day10.rs) |
| 11 | 719.7μs <!-- 719700 --> | 10.516ms <!-- 10516900 --> | [day11.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day11.rs) |
| 12 | 593.1μs <!-- 593100 --> | 70.851ms <!-- 70851400 --> | [day12.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day12.rs) |
| 13 | 239.7μs <!-- 239700 --> | 804.2μs <!-- 804200 --> | [day13.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day13.rs) |
<!-- EOT2023 -->

```
                                                       .-----.
                                                      ( #-...'`\
                                                       \ #     |
                                                      _ )"===="| _
                                                     (_`"======"`_)
                                                      /`""""""""`\
                                                     |        o _o\
                                   ,_                |         (_>|
              ___                 {` `}__             \      '.___/--#
 _  _      ."`   `"=,             `;"`   `"=,          '.    ;-._:'\
{_}:_`'=='` _,=="""=,\            /     _.-'`\           )===\  <)_/  __
     `'--.=" _.====, `|          ;  _.'`   _.;`    .---""`====`-'\__.'  `|
         `'_(| ^.^ |)/           ;'`    _.:_^}    /             ()\     /
           )';_'='_/.'  _  .'=_._\___.-|`|| || _  \___..--'        \_.-'
        .'`    '.=/(__/` | _`-.=||_||_||_||_|/` |    |           () |
       /    /   o|  \ \_/  .=`-`/           \\_/     ;              ;
      /`-,_/    o|  |-'        /`-..-'   ~~ |/        \          ()/
      `(`___   o/__/           \_/`        /           \      '.  /
       _`\ \`""`  \  __        __\ \   '-./ __,     _.'`\       `;
      || `'.'._   /-"//       {{o '.'.   //" //    (     `\       \_
      ||  ,/`-.`./  //        {{  \/`.`.//  //      \    .-`\       `\
  jgs `\\_/    `'-.//         `\\_/   `'; o//        \___)   `._____.'
                                `"`      `"`
```