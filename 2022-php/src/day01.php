<?php
require_once "day.php";
class Day01 extends Day {

    public ?int $testResultPart1 = 24000;
    public ?int $testResultPart2 = 45000;

    private function getCalories(bool $test): array {
        $data = $test ? $this->testData : $this->data;
        $elves = explode(PHP_EOL.PHP_EOL, $data);
        return array_map(function($elf) {
            $individualCalories = array_map(function ($calorie) {
                return intval($calorie);
            },explode(PHP_EOL, $elf));
            return array_sum($individualCalories);
        }, $elves);
    }

    public function part1(bool $test): int {
        $maxCalories = $this->getCalories($test);
        return max($maxCalories);
    }

    public function part2(bool $test): int {
        $maxCalories = $this->getCalories($test);
        rsort($maxCalories);
        return $maxCalories[0] + $maxCalories[1] + $maxCalories[2];
    }
}
