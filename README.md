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
| 1 | 77.1μs <!-- 77100 --> | 76.0μs <!-- 76000 --> | [day01.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day01.rs) |
| 2 | 1.458s <!-- 1000045800 --> | 1.245s <!-- 1000024500 --> | [day02.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day02.rs) |
| 6 | 2.9μs <!-- 2900 --> | 700ns <!-- 700 --> | [day06.rs](https://github.com/konstantin-lukas/advent-of-code/blob/master/2023-rust/src/solutions/day06.rs) |
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