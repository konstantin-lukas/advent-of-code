<?php

// PARSE README FOR TIME TABLE
$readme = file_get_contents('../README.md');
$output = preg_split('/<!-- [SE]OT2022 -->/', $readme);
$table = $output[1];
echo $table;

// RUN EACH DAY
for ($i = 0; $i < 1; $i++) {
    $paddedNumber = sprintf('%02d', $i + 1);
    $filePath = "src/day$paddedNumber.php";
    require_once $filePath;

    $className = "Day$paddedNumber";
    $day = new $className($i + 1);

    for ($j = 1; $j < 3; $j++) {
        // TEST ON EXAMPLE DATA
        $part = 'part'.$j;
        $testResult = $day->{"testResultPart$j"};
        $solution = $day->$part(true);
        if ($solution !== $testResult) {
            echo "Assertion error on day $i; Expected $testResult but got $solution";
            exit(1);
        }

        // CALCULATE RESULT ON MAIN DATA AND MEASURE TIME
        $start = hrtime(true);
        $solution = $day->$part(false);
        $deltaTime = hrtime(true) - $start;

        $decimals = match (true) {
            $deltaTime >= 1_000_000_000 => substr($deltaTime, -9),
            $deltaTime >= 1_000_000 => substr($deltaTime, -6),
            $deltaTime >= 1_000 => substr($deltaTime, -3),
            default => $deltaTime,
        };

        if ($deltaTime >= 1_000) {
            while (strlen($decimals) > 3 || (str_ends_with($decimals, '0') && strlen($decimals) > 1)) {
                $decimals = substr($decimals, 0, -1);
            }
        }

        $newData = match (true) {
            $deltaTime >= 1_000_000_000 => sprintf("%d.%ss <!-- %d -->", intdiv($deltaTime, 1_000_000_000), $decimals, $deltaTime),
            $deltaTime >= 1_000_000 => sprintf("%d.%sms <!-- %d -->", intdiv($deltaTime, 1_000_000), $decimals, $deltaTime),
            $deltaTime >= 1_000 => sprintf("%d.%sμs <!-- %d -->", intdiv($deltaTime, 1_000), $decimals, $deltaTime),
            default => sprintf("%dns <!-- %d -->", $deltaTime, $deltaTime),
        };

        echo "Day $paddedNumber, part $j: $solution".PHP_EOL;
    }
}