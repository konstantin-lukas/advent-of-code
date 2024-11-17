<?php
require_once "day.php";

class Day02 extends Day {

    public ?int $testResultPart1 = 15;

    private function parse(bool $test): array {
        $data = $test ? $this->testData : $this->data;
        $lines = explode(PHP_EOL, $data);
        $rounds = array_map(function ($line) {
            return explode(' ', $line);
        }, $lines);
        return $rounds;
    }

    public function part1(bool $test): int {
        $data = $this->parse($test);
        $results = array_map(function ($round) {
            [$first, $second] = $round;
            $handScore = match ($second) {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
            };
            $resultScore = match ($first) {
                'A' => match ($second) {
                    'X' => 3,
                    'Y' => 6,
                    'Z' => 0,
                },
                'B' => match ($second) {
                    'X' => 0,
                    'Y' => 3,
                    'Z' => 6,
                },
                'C' => match ($second) {
                    'X' => 6,
                    'Y' => 0,
                    'Z' => 3,
                },
            };
            return $handScore + $resultScore;
        }, $data);
        return array_sum($results);
    }

    public function part2(bool $test): int {
        return 0;
    }
}
