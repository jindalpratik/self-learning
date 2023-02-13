def average(array):
    set1 = set(array)
    sum = 0
    for i in set1:
        sum += i
    average = (sum)/len(set1)
    return average

if __name__ == '__main__':
    n = int(input())
    arr = list(map(int, input().split()))
    result = average(arr)
    print(result)