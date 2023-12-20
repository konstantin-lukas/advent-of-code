
def parse_ticket(ticket: str):
    trimmed_ticket = ticket[10:]
    winning_numbers = trimmed_ticket.split('|')[0].strip().split()
    holding_numbers = trimmed_ticket.split('|')[1].strip().split()
    return {
        "winning_numbers": winning_numbers,
        "holding_numbers": holding_numbers
    }


def get_points(correct_numbers: int):
    if correct_numbers <= 1:
        return correct_numbers
    return pow(2, correct_numbers - 1)


def get_correct_numbers(t):
    c = 0
    for number in t["holding_numbers"]:
        if number in t["winning_numbers"]:
            c += 1
    return c


def get_scratchcards(t, i, d):
    c = get_correct_numbers(t)
    for idx in range(i + 1, i + c + 1):
        c += get_scratchcards(d[idx], idx, d)
    return c


if __name__ == "__main__":

    with open('day4.data', 'r') as file:
        data = file.read().split('\n')[0:-1]

    data = list(map(parse_ticket, data))
    solution_part1, solution_part2 = 0, 0

    for index, ticket in enumerate(data):
        correct_numbers = get_correct_numbers(ticket)
        solution_part1 += get_points(correct_numbers)
        solution_part2 += get_scratchcards(ticket, index, data) + 1

    print('Solution Part 1: %d' % solution_part1)
    print('Solution Part 2: %d' % solution_part2)
