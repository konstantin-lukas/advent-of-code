<?php
require_once "day.php";

class Day10 extends Day {

    public mixed $testResultPart1 = 13140;
    public mixed $testResultPart2 =
        PHP_EOL."##..##..##..##..##..##..##..##..##..##..".
        PHP_EOL."###...###...###...###...###...###...###.".
        PHP_EOL."####....####....####....####....####....".
        PHP_EOL."#####.....#####.....#####.....#####.....".
        PHP_EOL."######......######......######......####".
        PHP_EOL."#######.......#######.......#######.....";

    private function parse(bool $test): array {
        $data = $test ? $this->testData : $this->data;
        $lines = explode(PHP_EOL, $data);
        return array_map(function ($line) {
            return intval(substr($line, 5));
        }, $lines);
    }

    public function part1(bool $test): int {
        $instructions = $this->parse($test);
        $result = 0;
        $register = 1;
        $cycle = 1;
        for ($i = 0; $i < count($instructions); $i++) {
            if (($cycle + 20) % 40 === 0) {
                $result += $register * $cycle;
            } else if ($instructions[$i] !== 0 && ($cycle + 21) % 40 === 0) {
                $result += $register * ($cycle + 1);
            }
            if ($instructions[$i] === 0) {
                $cycle++;
            } else {
                $register += $instructions[$i];
                $cycle += 2;
            }
        }
        return $result;
    }

    public function part2(bool $test): string {
        $instructions = $this->parse($test);
        $result = "";
        $register = 1;
        $cycle = 0;
        for ($i = 0; $i < count($instructions); $i++) {
            $result .= abs($register - ($cycle % 40)) <= 1 ? "#" : '.';
            if ($instructions[$i] === 0) {
                $cycle++;
            } else {
                $result .= abs($register - (($cycle + 1) % 40)) <= 1 ? "#" : '.';
                $cycle += 2;
                $register += $instructions[$i];
            }
        }
        return PHP_EOL . join(PHP_EOL, str_split($result, 40));
    }
}
