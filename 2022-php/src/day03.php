<?php
require_once "day.php";
class Day03 extends Day {

    public ?int $testResultPart1 = 157;
    public ?int $testResultPart2 = 70;

    private function getPriority(string $c): int
    {
        $value = ord($c);
        if ($value >= 65 && $value <= 90) {
            return $value - 38;
        }
        return $value - 96;
    }

    private function parsePart1(bool $test): array {
        $data = $test ? $this->testData : $this->data;
        $rucksacks = explode(PHP_EOL, $data);
        return array_map(function ($rucksack) {
            return array_map('str_split', str_split($rucksack, strlen($rucksack) / 2));
        }, $rucksacks);
    }

    private function parsePart2(bool $test): array {
        $data = $test ? $this->testData : $this->data;
        return array_chunk(array_map('str_split', explode(PHP_EOL, $data)), 3);
    }

    public function part1(bool $test): int {
        $priorities = $this->parsePart1($test);
        return array_sum(array_map(function ($priority) {
            $intersection = array_intersect($priority[0], $priority[1]);
            return $this->getPriority($intersection[array_key_first($intersection)]);
        }, $priorities));
    }

    public function part2(bool $test): int {
        $groups = $this->parsePart2($test);
        return array_sum(array_map(function ($group) {
            $intersection = array_intersect($group[0], $group[1], $group[2]);
            return $this->getPriority($intersection[array_key_first($intersection)]);
        }, $groups));
    }
}
