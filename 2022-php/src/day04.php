<?php
require_once "day.php";
class Day04 extends Day {

    public mixed $testResultPart1 = 2;
    public mixed $testResultPart2 = 4;

    private function parse(bool $test): array
    {
        $data = $test ? $this->testData : $this->data;
        $lines = explode(PHP_EOL, $data);
        return array_map(function ($line) {
            return array_map(function ($range) {
                return array_map('intval', explode('-', $range));
            }, explode(',', $line));
        }, $lines);
    }

    public function part1(bool $test): int
    {
        return count(array_filter($this->parse($test), function ($range) {
            [$left, $right] = $range;
            return $left[0] >= $right[0] && $left[1] <= $right[1] || $right[0] >= $left[0] && $right[1] <= $left[1];
        }));
    }

    public function part2(bool $test): int
    {
        return count(array_filter($this->parse($test), function ($range) {
            [$left, $right] = $range;
            return $left[0] <= $right[1] && $left[1] >= $right[0];
        }));
    }
}
