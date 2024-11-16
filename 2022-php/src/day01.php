<?php
require_once "day.php";
class Day01 extends Day {
    public int $testResultPart1 = 0;
    public int $testResultPart2 = 0;

    public function part1(bool $test): int {
        $data = $test ? $this->testData : $this->data;
        return 0;
    }

    public function part2(bool $test): int {
        $data = $test ? $this->testData : $this->data;
        return 0;
    }
}
