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
| 3 | 233.8μs <!-- 233800 --> | 157.8μs <!-- 157800 --> | [day03.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day03.rs) |
| 4 | 216.2μs <!-- 216200 --> | 155.5μs <!-- 155500 --> | [day04.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day04.rs) |
| 5 | 26.2μs <!-- 26200 --> | 40.0μs <!-- 40000 --> | [day05.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day05.rs) |
| 6 | 700ns <!-- 700 --> | 500ns <!-- 500 --> | [day06.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day06.rs) |
| 7 | 1.239ms <!-- 1239900 --> | 1.273ms <!-- 1273700 --> | [day07.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day07.rs) |
| 8 | 330.9μs <!-- 330900 --> | 1.539ms <!-- 1539200 --> | [day08.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day08.rs) |
| 9 | 111.0μs <!-- 111000 --> | 97.1μs <!-- 97100 --> | [day09.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day09.rs) |
| 10 | 135.6μs <!-- 135600 --> | 674.7μs <!-- 674700 --> | [day10.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day10.rs) |
| 11 | 637.4μs <!-- 637400 --> | 10.48ms <!-- 10480800 --> | [day11.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day11.rs) |
| 12 | 580.5μs <!-- 580500 --> | 70.645ms <!-- 70645900 --> | [day12.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day12.rs) |
| 13 | 228.2μs <!-- 228200 --> | 766.8μs <!-- 766800 --> | [day13.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day13.rs) |
| 14 | 58.8μs <!-- 58800 --> | 27.914ms <!-- 27914400 --> | [day14.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day14.rs) |
| 15 | 212.4μs <!-- 212400 --> | 277.0μs <!-- 277000 --> | [day15.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day15.rs) |
| 16 | 98.2μs <!-- 98200 --> | 13.467ms <!-- 13046700 --> | [day16.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day16.rs) |
| 17 | 42.561ms <!-- 42056100 --> | 149.912ms <!-- 149912500 --> | [day17.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day17.rs) |
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