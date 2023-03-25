list = [2,1,2]
# list = [3,2,1,2,2]
# list = [0,1,2,2]
basket_1 = []
basket_2 = []
for i in list:
    if i not in basket_1:
        basket_1.append(i)
    elif i not in basket_2:
        basket_2.append(i)
    else:
        break
print("Test 1")
# print("Test 2")
# print("Test 3")
print(len(basket_1+basket_2))