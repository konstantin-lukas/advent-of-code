<?php

for ($i = 0; $i < 1; $i++) {
    $paddedNumber = sprintf('%02d', $i + 1);
    $filePath = "src/day$paddedNumber.php";
    require_once $filePath;

    $className = "Day$paddedNumber";
    $day = new $className($i + 1);

    for ($j = 1; $j < 3; $j++) {
        $part = 'part'.$j;
        $testResult = $day->{"testResultPart$j"};
        $solution = $day->$part();
        if ($solution !== $testResult) {
            echo "Assertion error on day $i; Expected $testResult but got $solution";
            exit(1);
        }
        echo "Day $paddedNumber, part $j: $solution".PHP_EOL;
    }
}