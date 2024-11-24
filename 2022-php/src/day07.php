<?php
require_once "day.php";

class Dir {
    public array $children = [];
    public function __construct(public string $name, public ?Dir &$parent = null) {}
}

class File {
    public function __construct(public int $size, public string $name, public Dir &$parent) {}
}

class Day07 extends Day {

    public mixed $testResultPart1 = 95437;
    public mixed $testResultPart2 = 24933642;

    private function parse(bool $test): Dir
    {
        $data = $test ? $this->testData : $this->data;
        $root = new Dir("/");
        $wd = &$root;
        $lines = explode(PHP_EOL, $data);
        foreach ($lines as $line) {
            if (str_starts_with($line, "$")) {
                if ($line[2] === "c") {
                    $name = substr($line, 5);
                    if ($name === "/") {
                        $wd = &$root;
                        continue;
                    }
                    if ($name === "..") {
                        $wd = &$wd->parent;
                    } else {
                        for ($i = 0; $i < count($wd->children); $i++) {
                            if ($wd->children[$i] instanceof Dir && $wd->children[$i]->name === $name) {
                                $wd = &$wd->children[$i];
                                break;
                            }
                        }
                    }
                }
            } else {
                if (str_starts_with($line, "dir")) {
                    $wd->children[] = new Dir(substr($line, 4), $wd);
                } else {
                    [$size, $name] = explode(" ", $line);
                    $wd->children[] = new File($size, $name, $wd);
                }
            }
        }
        return $root;
    }

    private function traverse(Dir | File $node, array& $dirSizes): int {
        if ($node instanceof File) return $node->size;
        $result = 0;
        foreach ($node->children as $child) {
            $result += $this->traverse($child, $dirSizes);
        }
        $dirSizes[] = $result;
        return $result;
    }
    public function part1(bool $test): int {
        $root = $this->parse($test);
        $dirSizes = [];
        $this->traverse($root, $dirSizes);
        return array_sum(array_filter($dirSizes, fn ($x) => $x <= 100000));
    }

    public function part2(bool $test): int {
        $diskSpace = 70_000_000;
        $neededSpace = 30_000_000;
        $root = $this->parse($test);
        $dirSizes = [];
        $usedSpace = $this->traverse($root, $dirSizes);
        $unusedSpace = $diskSpace - $usedSpace;
        $requiredSpace = $neededSpace - $unusedSpace;
        $potentialDirs = array_filter($dirSizes, fn ($x) => $x >= $requiredSpace);
        return min($potentialDirs);
    }
}