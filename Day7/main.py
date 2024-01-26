import sys
import math

# dict2 = {'2':2, '3':3, '4':4, '5':5, '6':6, '7':7, '8':8, '9':9, 'T':10, 'J':1, 'Q':12, 'K':13, 'A':14}
dict = {'2':2, '3':3, '4':4, '5':5, '6':6, '7':7, '8':8, '9':9, 'T':10, 'J':1, 'Q':12, 'K':13, 'A':14}
next_set = {5:7, 7:9, 9:11, 11:13, 13:17, 17:25, 25:25} #may skip 32
# Five of a kind    5*5 = 25
# Four of a kind    4*4 +1 = 17
# Full house        3*3 + 2*2 = 13
# Three of a kind   3*3 + 1*2 = 11
# Two pair          2*2 + 2*2 + 1 = 9
# One pair          2*2 + 1*3 = 7
# High card = 0     1*5 = 5
#
# Last If matching then high card in order

def bubbleSort(arr, arr2, arr3):
    n = len(arr)
    for i in range(n):
        for j in range(0, n - i - 1):
            if arr[j] > arr[j + 1]:
                arr[j], arr[j + 1] = arr[j + 1], arr[j]
                arr2[j], arr2[j + 1] = arr2[j + 1], arr2[j]
                arr3[j], arr3[j + 1] = arr3[j + 1], arr3[j]
            elif arr[j] == arr[j + 1]:
                for x in range(5):
                    if dict[arr2[j][x]] > dict[arr2[j+1][x]]:
                        arr[j], arr[j + 1] = arr[j + 1], arr[j]
                        arr2[j], arr2[j + 1] = arr2[j + 1], arr2[j]
                        arr3[j], arr3[j + 1] = arr3[j + 1], arr3[j]
                        break
                    elif dict[arr2[j][x]] < dict[arr2[j+1][x]]:
                        break

with open('input.text', 'r') as file:
        lines = file.readlines()
        
part1 = False
hands:list[str] = []
hands_val:list[int] = []
bets:list[int] = []
for line in lines:
    split = line.split()
    hands.append(split[0])
    hands_val.append(0)
    bets.append(int(split[1]))

for index in range(len(hands)):
    for ch in hands[index]:
        hands_val[index] += hands[index].count(ch)
for index in range(len(hands)):
    for c in range(hands[index].count('J')):
        hands_val[index] = next_set[hands_val[index]]
bubbleSort(hands_val, hands, bets)


count = 0
for i in range(len(bets)):
    count += bets[i] * (i+1)
print(bets)
print(hands_val)
print(count)

