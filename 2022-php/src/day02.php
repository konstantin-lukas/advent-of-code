<?php
require_once "day.php";

class Day02 extends Day {

    public mixed $testResultPart1 = 15;
    public mixed $testResultPart2 = 12;

    private function parse(bool $test): array {
        $data = $test ? $this->testData : $this->data;
        $lines = explode(PHP_EOL, $data);
        return array_map(function ($line) {
            return explode(' ', $line);
        }, $lines);
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
        $data = $this->parse($test);
        $results = array_map(function ($round) {
            [$first, $second] = $round;
            $resultScore = match ($second) {
                'X' => 0,
                'Y' => 3,
                'Z' => 6,
            };
            $handScore = match ($second) {
                'X' => match ($first) {
                    'A' => 3,
                    'B' => 1,
                    'C' => 2,
                },
                'Y' => match ($first) {
                    'A' => 1,
                    'B' => 2,
                    'C' => 3,
                },
                'Z' => match ($first) {
                    'A' => 2,
                    'B' => 3,
                    'C' => 1,
                },
            };
            return $handScore + $resultScore;
        }, $data);
        return array_sum($results);
    }
}
