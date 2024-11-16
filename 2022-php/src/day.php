<?php
class Day {
    protected string $data;
    protected string $testData;

    public int $testResultPart1 = 0;
    public int $testResultPart2 = 0;

    public function __construct(int $day) {
        $this->data = file_get_contents(__DIR__."/../data/day".sprintf("%02d", $day));
        $this->testData = file_get_contents(__DIR__."/../example/day".sprintf("%02d", $day));
    }

    public function part1(bool $test): int {
        return 0;
    }

    public function part2(bool $test): int {
        return 0;
    }
}