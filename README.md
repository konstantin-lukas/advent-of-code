# Advent of Code
These are my solutions to the advent of code. I've been doing advent of code even before this public github repo. I mainly do advent of code to learn new languages.

## Benchmarks
Benchmarks are run automatically every time the main function is called. The best overall
time is recorded in the table below. Benchmarks do not include loading data from disk but do
include any kind of transformation on the input. Sometimes part 2 has to repeat steps already
taken in part 1. This leads to a slightly slower combined time of both parts. These benchmarks
were executed on an AMD Ryzen 7 7700X.
### 2022 (PHP)
<!-- SOT2022 -->
| Day | Best Time Part 1 | Best Time Part 2 | Code |
|---|---|---|---|
| 1 | 124.29μs <!-- 124290 --> | 137.309μs <!-- 137309 --> | [day01.php](https://github.com/konstantin-lukas/advent-of-code/blob/master/2022-php/src/day01.php) |
| 2 | 340.058μs <!-- 340058 --> | 309.809μs <!-- 309809 --> | [day02.php](https://github.com/konstantin-lukas/advent-of-code/blob/master/2022-php/src/day02.php) |
<!-- EOT2022 -->

### 2023 (Rust)
<!-- SOT2023 -->
| Day | Best Time Part 1 | Best Time Part 2 | Code |
|---|---|---|---|
| 1 | 72.2μs <!-- 72200 --> | 71.3μs <!-- 71300 --> | [day01.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day01.rs) |
| 2 | 10.3μs <!-- 10300 --> | 9.4μs <!-- 9400 --> | [day02.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day02.rs) |
| 3 | 233.8μs <!-- 233800 --> | 157.8μs <!-- 157800 --> | [day03.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day03.rs) |
| 4 | 216.2μs <!-- 216200 --> | 147.9μs <!-- 147900 --> | [day04.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day04.rs) |
| 5 | 25.8μs <!-- 25800 --> | 39.2μs <!-- 39200 --> | [day05.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day05.rs) |
| 6 | 700ns <!-- 700 --> | 500ns <!-- 500 --> | [day06.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day06.rs) |
| 7 | 1.234ms <!-- 1234600 --> | 1.273ms <!-- 1273700 --> | [day07.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day07.rs) |
| 8 | 325.6μs <!-- 325600 --> | 1.539ms <!-- 1539200 --> | [day08.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day08.rs) |
| 9 | 111.0μs <!-- 111000 --> | 97.1μs <!-- 97100 --> | [day09.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day09.rs) |
| 10 | 133.7μs <!-- 133700 --> | 674.7μs <!-- 674700 --> | [day10.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day10.rs) |
| 11 | 637.4μs <!-- 637400 --> | 10.48ms <!-- 10480800 --> | [day11.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day11.rs) |
| 12 | 570.5μs <!-- 570500 --> | 70.645ms <!-- 70645900 --> | [day12.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day12.rs) |
| 13 | 228.2μs <!-- 228200 --> | 766.8μs <!-- 766800 --> | [day13.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day13.rs) |
| 14 | 58.8μs <!-- 58800 --> | 27.914ms <!-- 27914400 --> | [day14.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day14.rs) |
| 15 | 200.6μs <!-- 200600 --> | 275.9μs <!-- 275900 --> | [day15.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day15.rs) |
| 16 | 95.3μs <!-- 95300 --> | 13.467ms <!-- 13046700 --> | [day16.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day16.rs) |
| 17 | 25.834ms <!-- 25834800 --> | 146.874ms <!-- 146874600 --> | [day17.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day17.rs) |
| 18 | 41.7μs <!-- 41700 --> | 46.2μs <!-- 46200 --> | [day18.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day18.rs) |
| 19 | 272.0μs <!-- 272000 --> | 316.6μs <!-- 316600 --> | [day19.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day19.rs) |
| 20 | 1.149ms <!-- 1149800 --> | 5.364ms <!-- 5364400 --> | [day20.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day20.rs) |
| 21 | 8.56ms <!-- 8560300 --> | 479.8μs <!-- 479800 --> | [day21.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day21.rs) |
| 22 | 16.852ms <!-- 16852500 --> | 21.396ms <!-- 21396400 --> | [day22.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day22.rs) |
| 23 | 8.253ms <!-- 8253900 --> | 2.477s <!-- 2477428200 --> | [day23.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day23.rs) |
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