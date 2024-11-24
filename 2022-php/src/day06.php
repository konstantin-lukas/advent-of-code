<?php
require_once "day.php";

class Day06 extends Day {

    public mixed $testResultPart1 = 7;
    public mixed $testResultPart2 = 19;

    private function solve(bool $test, int $length): int
    {
        $data = $test ? $this->testData : $this->data;
        for ($i = $length; $i < strlen($data); $i++) {
            $sequence = substr($data, $i - $length, $length);
            if (count(array_unique(str_split($sequence))) === $length) break;
        }
        return $i;
    }

    public function part1(bool $test): int {
        return $this->solve($test, 4);
    }

    public function part2(bool $test): int {
        return $this->solve($test, 14);
    }
}
