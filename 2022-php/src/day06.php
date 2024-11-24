<?php
require_once "day.php";

class Day06 extends Day {

    public mixed $testResultPart1 = 7;

    public function part1(bool $test): int {
        $data = $test ? $this->testData : $this->data;
        for ($i = 4; $i < strlen($data); $i++) {
            $sequence = substr($data, $i - 4, 4);
            if (count(array_unique(str_split($sequence))) === 4) break;
        }
        return $i;
    }

    public function part2(bool $test): int {
        return 0;
    }
}
