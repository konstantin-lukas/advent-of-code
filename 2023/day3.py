import re

if __name__ == "__main__":
    with open('day3.data', 'r') as file:
        data = file.read().split('\n')[0:-1]

    solution_part1, solution_part2 = 0, 0

    for line_index, line in enumerate(data):
        current_number = ''
        keep_number = False
        # part1
        for char_index, char in enumerate(line):
            if '0' <= char <= '9':
                current_number += char
                min_char_index = 0 if char_index == 0 else char_index - 1
                max_char_index = len(line) - 1 if char_index == len(line) - 1 else char_index + 1
                adjacent_char_on_prev_line = (
                    line_index > 0
                    and (
                        re.match('[^0-9.]', data[line_index - 1][min_char_index])
                        or re.match('[^0-9.]', data[line_index - 1][char_index])
                        or re.match('[^0-9.]', data[line_index - 1][max_char_index])
                    )
                )
                adjacent_char_on_current_line = (
                    re.match('[^0-9.]', data[line_index][min_char_index])
                    or re.match('[^0-9.]', data[line_index][char_index])
                    or re.match('[^0-9.]', data[line_index][max_char_index])
                )
                adjacent_char_on_next_line = (
                    line_index < len(data) - 1
                    and (
                        re.match('[^0-9.]', data[line_index + 1][min_char_index])
                        or re.match('[^0-9.]', data[line_index + 1][char_index])
                        or re.match('[^0-9.]', data[line_index + 1][max_char_index])
                    )
                )
                if adjacent_char_on_prev_line or adjacent_char_on_current_line or adjacent_char_on_next_line:
                    keep_number = True
            if not '0' <= char <= '9' or char_index == len(line) - 1:
                if keep_number and not current_number == '':
                    solution_part1 += int(current_number)
                    keep_number = False
                current_number = ''
        # part2
        asterisks = re.finditer('\*', line)
        for asterisk in asterisks:
            adjacent_numbers = []
            position = asterisk.start()
            numbers_on_prev_line = [] if line_index == 0 else re.finditer('[0-9]+', data[line_index - 1])
            numbers_on_current_line = re.finditer('[0-9]+', line)
            numbers_on_next_line = [] if line_index == len(data) - 1 else re.finditer('[0-9]+', data[line_index + 1])
            for number in numbers_on_prev_line:
                if abs(number.start() - position) <= 1 or abs(number.end() - position - 1) <= 1:
                    adjacent_numbers.append(int(data[line_index - 1][number.start():number.end()]))
            for number in numbers_on_current_line:
                if number.start() == position + 1 or number.end() == position:
                    adjacent_numbers.append(int(line[number.start():number.end()]))
            for number in numbers_on_next_line:
                if abs(number.start() - position) <= 1 or abs(number.end() - position - 1) <= 1:
                    adjacent_numbers.append(int(data[line_index + 1][number.start():number.end()]))
            if len(adjacent_numbers) == 2:
                solution_part2 += adjacent_numbers[0] * adjacent_numbers[1]

    print('Solution Part 1: %d' % solution_part1)
    print('Solution Part 2: %d' % solution_part2)
