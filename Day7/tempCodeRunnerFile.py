def tieSort(arr, arr2, arr3):
    n = len(arr)
    for i in range(n-1):
        if arr[i] == arr[i+1]:
            for j in range(5):
                if dict[arr2[i][j]] > dict[arr2[i+1][j]]:
                    arr[i], arr[i + 1] = arr[i + 1], arr[i]
                    arr2[i], arr2[i + 1] = arr2[i + 1], arr2[i]
                    arr3[i], arr3[i + 1] = arr3[i + 1], arr3[i]
                    break
                elif dict[arr2[i][j]] < dict[arr2[i+1][j]]:
                    break