<?php
require_once "day.php";

class Monkey {
    public int $inspectCount = 0;
    public function __construct(
        public array $items,
        public bool $multiply,
        public ?int $factor,
        public int $conditional,
        public int $trueMonkey,
        public int $falseMonkey,
    ) {}
}

class Day11 extends Day {
    public mixed $testResultPart1 = 10605;

    private function parse(bool $test): array
    {
        $data = $test ? $this->testData : $this->data;
        $rawMonkeys = explode(PHP_EOL.PHP_EOL, $data);
        return array_map(function ($monkey) {
            $lines = explode(PHP_EOL, $monkey);
            $items = array_map('intval', explode(', ', substr($lines[1], 18)));
            $multiply = $lines[2][23] === "*";
            $rawFactor = substr($lines[2], 25);
            $factor = $rawFactor === "old" ? null : intval($rawFactor);
            $conditional = intval(substr($lines[3], 21));
            $trueMonkey = intval(substr($lines[4], -1));
            $falseMonkey = intval(substr($lines[5], -1));
            return new Monkey($items, $multiply, $factor, $conditional, $trueMonkey, $falseMonkey);
        }, $rawMonkeys);
    }

    public function part1(bool $test): int {
        $monkeys = $this->parse($test);
        for ($round = 0; $round < 20; $round++) {
            foreach ($monkeys as $monkey) {
                while ($item = array_shift($monkey->items)) {
                    $monkey->inspectCount++;
                    $factor = $monkey->factor === null ? $item : $monkey->factor;
                    if ($monkey->multiply) {
                        $item *= $factor;
                    } else {
                        $item += $factor;
                    }
                    $item = intdiv($item, 3);
                    if ($item % $monkey->conditional === 0) {
                        $monkeys[$monkey->trueMonkey]->items[] = $item;
                    } else {
                        $monkeys[$monkey->falseMonkey]->items[] = $item;
                    }
                }
            }
        }
        $inspectCounts = array_map(fn ($monkey) => $monkey->inspectCount, $monkeys);
        rsort($inspectCounts);
        return $inspectCounts[0] * $inspectCounts[1];
    }

    public function part2(bool $test): int {
        return 0;
    }
}
