import re


if __name__ == "__main__":
    with open('day2.data', 'r') as file:
        data = file.read().split('\n')

    max_reds = 12
    max_greens = 13
    max_blues = 14

    solution_part1, solution_part2 = 0, 0

    for game in data:
        if game:
            split_line = game.split(': ')
            rounds = split_line[1].split(';')
            game_possible = True
            least_amount_of_reds = 0
            least_amount_of_greens = 0
            least_amount_of_blues = 0
            for round in rounds:
                reds = re.findall('\d+(?=\s*red)', round)
                greens = re.findall('\d+(?=\s*green)', round)
                blues = re.findall('\d+(?=\s*blue)', round)
                if len(reds) > 0:
                    red_count = int(reds[0])
                    if red_count > least_amount_of_reds:
                        least_amount_of_reds = red_count
                    if red_count > max_reds:
                        game_possible = False
                if len(greens) > 0:
                    green_count = int(greens[0])
                    if green_count > least_amount_of_greens:
                        least_amount_of_greens = green_count
                    if green_count > max_greens:
                        game_possible = False
                if len(blues) > 0:
                    blue_count = int(blues[0])
                    if blue_count > least_amount_of_blues:
                        least_amount_of_blues = blue_count
                    if blue_count > max_blues:
                        game_possible = False
            solution_part2 += least_amount_of_reds * least_amount_of_greens * least_amount_of_blues
            if game_possible:
                solution_part1 += int(re.findall('\d+', split_line[0])[0])

    print('Solution Part 1: %d' % solution_part1)
    print('Solution Part 2: %d' % solution_part2)

