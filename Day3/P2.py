total = 0
with open('input.txt', 'r') as file:
    lines = file.readlines()
arr = []


#Sorry for the crappy varible names did it as a joke as I kept messing up in python


it = 0
hit = False
temp427 = 0
temp999 = 0
temp2048 = 0
jt = 0
for line in lines:
    arr.append(line.strip("\n"))
temp = []
temp99 = 0
#remem = 0
for i in range(len(arr)):
    for j in range(len(arr[i])):
        if jt > 0:
            jt -= 1
        elif arr[i][j].isalnum():
            if i == 0:
                it = i+1
            else:
                it = i
            for t in arr[it-1:i+2]:
                temp.append(t[j-1:j+2])
            for x in temp:
                for y in x:
                    if not y.isalnum() and not y == ".":
                        #Number set works
                        hit = True
            if hit:
                #if it == i+1:
                temp99 = int(arr[i][j])
                if arr[i][j-1].isalnum():
                    temp99 += int(arr[i][j-1]) * 10
                    if arr[i][j-2].isalnum():
                        temp99 += int(arr[i][j-2]) * 100
                if arr[i][j+1].isalnum():
                    j += 1
                    jt += 1
                    temp99 = temp99 * 10 + int(arr[i][j]) 
                    if arr[i][j+1].isalnum():
                        j += 1
                        jt += 1
                        temp99 = temp99 * 10 + int(arr[i][j])
                
                
            
            hit = False
            temp.clear()
        if temp99 > 0:
            print(temp99) 
        total += temp99
        temp99 = 0
print(total)