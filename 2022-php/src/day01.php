<?php
require_once "day.php";
class Day01 extends Day {

    public ?int $testResultPart1 = 24000;

    public function part1(bool $test): int {
        $data = $test ? $this->testData : $this->data;
        $elves = explode(PHP_EOL.PHP_EOL, $data);
        $maxCalories = array_map(function($elf) {
            $individualCalories = array_map(function ($calorie) {
                return intval($calorie);
            },explode(PHP_EOL, $elf));
            return array_sum($individualCalories);
        },$elves);
        return max($maxCalories);
    }

    public function part2(bool $test): int {
        $data = $test ? $this->testData : $this->data;
        return 0;
    }
}
