arr = [6, -6]
# arr = [7,-7]
# arr = [10,2,-5]
for i in range((len(arr)-1), 0, -1):
    if arr[i] < 0 and arr[i-1] > 0:
        if arr[i] > (arr[i - 1]*-1):
            arr.pop(i)
        elif arr[i] < (arr[i - 1]*-1):
            arr.pop(i-1)
        else:
            arr.pop(i)
print(arr)
