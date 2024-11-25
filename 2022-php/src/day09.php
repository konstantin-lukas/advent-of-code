<?php
require_once "day.php";

enum Direction {
    case North;
    case South;
    case East;
    case West;
}

class Day09 extends Day {

    public mixed $testResultPart1 = 13;
    public mixed $testResultPart2 = 1;

    private static function isAdjacent(int $x1, int $y1, int $x2, int $y2): bool
    {
        return abs($x1 - $x2) <= 1 && abs($y1 - $y2) <= 1;
    }

    private function parse(bool $test): array
    {
        $data = $test ? $this->testData : $this->data;
        return array_map(function ($line) {
            $dir = match ($line[0]) {
                "U" => Direction::North,
                "D" => Direction::South,
                "R" => Direction::East,
                "L" => Direction::West,
            };
            return ["dir" => $dir, "count" => intval(substr($line, 2))];
        }, explode(PHP_EOL, $data));
    }

    public function part1(bool $test): int {
        $moves = $this->parse($test);
        $head = [
            "x" => 0,
            "y" => 0,
        ];
        $tail = [
            "x" => 0,
            "y" => 0,
        ];
        $visited = [];

        foreach ($moves as $move) {
            for ($i = 0; $i < $move["count"]; $i++) {
                $oldPosition = $head;
                switch ($move["dir"]) {
                    case Direction::North:
                        $head["y"]++;
                        break;
                    case Direction::South:
                        $head["y"]--;
                        break;
                    case Direction::East:
                        $head["x"]++;
                        break;
                    case Direction::West:
                        $head["x"]--;
                        break;
                }
                if (!self::isAdjacent($head["x"], $head["y"], $tail["x"], $tail["y"])) {
                    $tail = $oldPosition;
                }
                $visited[] = $tail["x"] . "," . $tail["y"];
            }
        }
        return count(array_unique($visited));
    }

    public function part2(bool $test): int {
        $moves = $this->parse($test);
        $segments = array_fill(0, 10, ["x" => 0, "y" => 0]);
        $visited = [];

        foreach ($moves as $move) {
            for ($i = 0; $i < $move["count"]; $i++) {
                switch ($move["dir"]) {
                    case Direction::North:
                        $segments[0]["y"]++;
                        break;
                    case Direction::South:
                        $segments[0]["y"]--;
                        break;
                    case Direction::East:
                        $segments[0]["x"]++;
                        break;
                    case Direction::West:
                        $segments[0]["x"]--;
                        break;
                }

                for ($j = 1; $j < count($segments); $j++) {
                    $prev = $segments[$j - 1];
                    $curr = $segments[$j];

                    if (self::isAdjacent($prev["x"], $prev["y"], $curr["x"], $curr["y"])) {
                        break;
                    }
                    if ($prev["x"] != $curr["x"]) {
                        $segments[$j]["x"] += ($prev["x"] > $curr["x"]) ? 1 : -1;
                    }
                    if ($prev["y"] != $curr["y"]) {
                        $segments[$j]["y"] += ($prev["y"] > $curr["y"]) ? 1 : -1;
                    }
                }

                $tail = $segments[9];
                $visited[] = $tail["x"] . "," . $tail["y"];
            }
        }

        return count(array_unique($visited));
    }

}
