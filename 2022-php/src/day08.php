<?php
require_once "day.php";

class Day08 extends Day {

    public mixed $testResultPart1 = 21;
    public mixed $testResultPart2 = 8;

    private function parse(bool $test): array {
        $data = $test ? $this->testData : $this->data;
        $rows = explode(PHP_EOL, $data);
        return array_map(function ($row) {
            return array_map('intval', str_split($row));
        }, $rows);
    }

    public function part1(bool $test): int {
        $data = $this->parse($test);
        $visibleTrees = 0;
        $rowCount = count($data);
        $colCount = count($data[0]);
        for ($row = 0; $row < $rowCount; $row++) {
            for ($col = 0; $col < $colCount; $col++) {
                $visible = [
                    "top" => true,
                    "left" => true,
                    "right" => true,
                    "bottom" => true,
                ];
                $height = $data[$row][$col];

                // RIGHT
                for ($i = $col + 1; $i < $colCount; $i++) {
                    if ($data[$row][$i] >= $height) {
                        $visible["right"] = false;
                        break;
                    }
                }

                if ($visible["right"]) {
                    $visibleTrees++;
                    continue;
                }

                // LEFT
                for ($i = $col - 1; $i >= 0; $i--) {
                    if ($data[$row][$i] >= $height) {
                        $visible["left"] = false;
                        break;
                    }
                }

                if ($visible["left"]) {
                    $visibleTrees++;
                    continue;
                }

                // BOTTOM
                for ($i = $row + 1; $i < $rowCount; $i++) {
                    if ($data[$i][$col] >= $height) {
                        $visible["bottom"] = false;
                        break;
                    }
                }

                if ($visible["bottom"]) {
                    $visibleTrees++;
                    continue;
                }

                // TOP
                for ($i = $row - 1; $i >= 0; $i--) {
                    if ($data[$i][$col] >= $height) {
                        $visible["top"] = false;
                        break;
                    }
                }

                if ($visible["top"]) {
                    $visibleTrees++;
                }
            }
        }
        return $visibleTrees;
    }

    public function part2(bool $test): int {
        $data = $this->parse($test);
        $maxScenicScore = 0;
        $rowCount = count($data);
        $colCount = count($data[0]);
        for ($row = 0; $row < $rowCount; $row++) {
            for ($col = 0; $col < $colCount; $col++) {
                $visible = [
                    "top" => 0,
                    "left" => 0,
                    "right" => 0,
                    "bottom" => 0,
                ];
                $height = $data[$row][$col];

                // RIGHT
                for ($i = $col + 1; $i < $colCount; $i++) {
                    $visible["right"]++;
                    if ($data[$row][$i] >= $height) {
                        break;
                    }
                }

                // LEFT
                for ($i = $col - 1; $i >= 0; $i--) {
                    $visible["left"]++;
                    if ($data[$row][$i] >= $height) {
                        break;
                    }
                }

                // BOTTOM
                for ($i = $row + 1; $i < $rowCount; $i++) {
                    $visible["bottom"]++;
                    if ($data[$i][$col] >= $height) {
                        break;
                    }
                }

                // TOP
                for ($i = $row - 1; $i >= 0; $i--) {
                    $visible["top"]++;
                    if ($data[$i][$col] >= $height) {
                        break;
                    }
                }

                $scenicScore = $visible["top"] * $visible["left"] * $visible["right"] * $visible["bottom"];
                if ($scenicScore > $maxScenicScore) {
                    $maxScenicScore = $scenicScore;
                }
            }
        }
        return $maxScenicScore;
    }
}
