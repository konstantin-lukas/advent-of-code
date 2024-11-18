<?php

$dir = "./src";
$fi = new FilesystemIterator($dir, FilesystemIterator::SKIP_DOTS);
$solvedDays = iterator_count($fi) - 1;

// PARSE README FOR TIME TABLE
$readme = file_get_contents('../README.md');
$output = preg_split('/<!-- [SE]OT2022 -->/', $readme);
$table = trim($output[1]);
$rows = explode(PHP_EOL, $table);
if ($rows[0] !== '| Day | Best Time Part 1 | Best Time Part 2 | Code |' || $rows[1] !== '|---|---|---|---|') {
    exit(1);
}

$times = array_map(function($line) {
    preg_match_all('/(?<=<!--\s)(\d+)(?=\s-->)/', $line, $matches);
    if (count($matches[0]) !== 2) {
        echo 'Incorrect number of results';
        exit(1);
    }
    return array_map(fn($result) => (int)$result, $matches[0]);
}, array_slice($rows, 2));

if (count($times) < $solvedDays) {
    $times = array_pad($times, $solvedDays, [PHP_INT_MAX, PHP_INT_MAX]);
}

$newTable = '<!-- SOT2022 -->' . PHP_EOL . $rows[0] . PHP_EOL . $rows[1] . PHP_EOL;
echo "<pre>";
// RUN EACH DAY
for ($i = 0; $i < $solvedDays; $i++) {
    $paddedNumber = sprintf('%02d', $i + 1);
    $filePath = "src/day$paddedNumber.php";
    require_once $filePath;

    $className = "Day$paddedNumber";
    $day = new $className($i + 1);

    $row = '| ' . ($i + 1) . ' | ';
    for ($j = 1; $j <= 2; $j++) {
        // TEST ON EXAMPLE DATA
        $part = 'part'.$j;
        $testResult = $day->{"testResultPart$j"};

        if ($testResult === null) {
            $row .= 'N/A <!-- ' . PHP_INT_MAX . ' --> | ';
            continue;
        }

        $solution = $day->$part(true);
        if ($testResult !== null && $solution !== $testResult) {
            echo sprintf("Assertion error on day%d $part; Expected $testResult but got $solution", $i + 1);
            exit(1);
        }

        // CALCULATE RESULT ON MAIN DATA AND MEASURE TIME
        $start = hrtime(true);
        $solution = $day->$part(false);
        $deltaTime = hrtime(true) - $start;

        if ($deltaTime >= $times[$i][$j - 1]) {
            $deltaTime = $times[$i][$j - 1];
        }

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

        $row .= "$newData | ";

        echo "Day $paddedNumber, part $j: $solution".PHP_EOL;
    }

    $row .= "[day$paddedNumber.php](https://github.com/konstantin-lukas/advent-of-code/blob/master/2022-php/src/day$paddedNumber.php) |";
    $newTable .= $row . PHP_EOL;
}
echo "</pre>";
$newTable .= '<!-- EOT2022 -->';

file_put_contents('../README.md', $output[0] . $newTable . $output[2]);