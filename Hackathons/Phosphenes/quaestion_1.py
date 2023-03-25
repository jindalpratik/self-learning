str_li = input().split("HIP")
str_new = ""
for i in str_li:
    # print(str_li)
    if i == "":
        continue
    str_new += i + " "
    # print(str_new)
k = str_new.strip()
print(k)