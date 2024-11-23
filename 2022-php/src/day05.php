<?php
require_once "day.php";

class Day05 extends Day {

    public mixed $testResultPart1 = "CMZ";
    public mixed $testResultPart2 = "MCD";

    private function parse(bool $test): array
    {
        $data = $test ? $this->testData : $this->data;
        [$containers, $moves] = explode(PHP_EOL.PHP_EOL, $data);
        $stacks = [];
        $containerRows = array_reverse(explode(PHP_EOL, $containers));

        foreach (array_slice($containerRows, 1) as $row) {
            for ($i = 1; $i < strlen($row); $i += 4) {
                $container = $row[$i];
                if ($container === " ") continue;
                $stacks[intdiv($i, 4) + 1][] = $container;
            }
        }

        $moveRows = explode(PHP_EOL, $moves);
        $moves = array_map(function ($move) {
            $separated = explode(" ", $move);
            return [
                "count" => intval($separated[1]),
                "from" => intval($separated[3]),
                "to" => intval($separated[5])
            ];
        }, $moveRows);

        return [$stacks, $moves];
    }

    public function part1(bool $test): string {
        [$stacks, $moves] = $this->parse($test);
        foreach ($moves as $move) {
            for ($i = 0; $i < $move["count"]; $i++) {
                $elem = array_pop($stacks[$move["from"]]);
                $stacks[$move["to"]][] = $elem;
            }
        }
        $topContainers = array_map(function ($stack) {
            return end($stack);
        }, $stacks);

        return implode($topContainers);
    }

    public function part2(bool $test): string {
        [$stacks, $moves] = $this->parse($test);
        foreach ($moves as $move) {
            $arr = &$stacks[$move["from"]];
            $cnt = $move["count"];
            $elems = array_splice($arr, count($arr) - $cnt, $cnt);
            array_push($stacks[$move["to"]], ...$elems);
        }
        $topContainers = array_map(function ($stack) {
            return end($stack);
        }, $stacks);

        return implode($topContainers);
    }
}
