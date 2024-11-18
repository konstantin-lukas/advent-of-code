<?php
require_once "day.php";
class Day03 extends Day {

    public ?int $testResultPart1 = 157;

    private function parse(bool $test): array {
        $data = $test ? $this->testData : $this->data;
        $rucksacks = explode(PHP_EOL, $data);
        return array_map(function ($rucksack) {
            return array_map(function ($compartment) {
                return array_map(function ($i) {
                    if ($i >= 65 && $i <= 90) {
                        return $i - 38;
                    }
                    return $i - 96;
                }, unpack("C*", $compartment));
            }, str_split($rucksack, strlen($rucksack) / 2));
        }, $rucksacks);
    }

    public function part1(bool $test): int {
        $priorities = $this->parse($test);
        return array_sum(array_map(function ($priority) {
            $intersection = array_intersect($priority[0], $priority[1]);
            return $intersection[array_key_first($intersection)];
        }, $priorities));
    }

    public function part2(bool $test): int {
        return 0;
    }
}
