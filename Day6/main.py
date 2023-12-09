import sys
import math

def main() -> None:
    print("The answer to Day 6 Part 1: ",part1())
    print("The answer to Day 6 Part 2: ", part2())


def part1() -> int:
    with open('input.text', 'r') as file:
        lines = file.readlines()
    times: list[int] = list(map(int, lines[0].split()[1:]))
    distances:list[int] = list(map(int, lines[1].split()[1:]))

    if len(times) != len(distances):
        print("Panicked because Race Times and Race Distances are not the same length")
        sys.exit()

    totals:list[int] = []
    for index in range(len(times)):
        totals.append(0)
        for hold in range(1, times[index]):
            distance = hold * (times[index]-hold)
            if distance > distances[index]:
                totals[index] += 1
        
    return (math.prod(totals))


def part2() -> int:
    with open('input.text', 'r') as file:
        lines = file.readlines()
    time: int = int(lines[0].replace(" ", "")[5:])
    distance: int = int(lines[1].replace(" ", "")[9:])
    total = 0
    for hold in range(1, time):
            predictedDistance = hold * (time-hold)
            if predictedDistance > distance:
                total += 1

    return (total)


if __name__ == "__main__":
    main()