import re


def parse_number(num: str):
    digit_names = {
        'one': '1', 'two': '2', 'three': '3', 'four': '4', 'five': '5',
        'six': '6', 'seven': '7', 'eight': '8', 'nine': '9'
    }
    if num in digit_names:
        return digit_names[num]
    return num


if __name__ == "__main__":
    with open('day1.data', 'r') as file:
        data = file.read().split('\n')

    solution_part1, solution_part2 = 0, 0
    for string in data:
        if string:
            digits = re.findall('\d', string)
            digits_and_words = re.findall('(?=(\d|one|two|three|four|five|six|seven|eight|nine))', string)
            solution_part1 += int(digits[0] + digits[-1])
            solution_part2 += int(parse_number(digits_and_words[0]) + parse_number(digits_and_words[-1]))

    print('Solution Part 1: %d' % solution_part1)
    print('Solution Part 2: %d' % solution_part2)
