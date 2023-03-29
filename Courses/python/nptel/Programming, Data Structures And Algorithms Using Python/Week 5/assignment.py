cond = True
state_num = 1
skip_line = True
list_book_name = []
list_borrower_name = []
list_details = []
num = 0
throwaway = input()

# Takking inputs and storing them in lists.
while state_num != 0:
    inp_str = input()
    
    if inp_str == "Borrowers" or inp_str == "Checkouts" or inp_str == "EndOfInput":
        if inp_str == "Borrowers":
            state_num = 2
        elif inp_str == "Checkouts":
            state_num = 3
        else:
            state_num = 0
        continue
    
    if state_num == 1:
        temp = inp_str.split("~")
        list_book_name.append(temp)
    elif state_num == 2:
        temp = inp_str.split("~")
        list_borrower_name.append(temp)
    elif state_num == 3:
        temp = inp_str.split("~")
        list_details.append(temp)
    num += 1
    
# Conversting some of lists into dictionaries for easier acces to some values.
dict_book_name = dict(list_book_name)
dict_borrower_name = dict(list_borrower_name)

# Storing the dates in a list to sorted.
new_list = []
for i in list_details:
    new_list.append(i[2])

# Sorting the dates.
new_list.sort()

# Loop to iterate through sorted dates.
for k in new_list:
    
    name_list_1 = []
    name_list_2 = []
    
    # Storing the names of people who borrowed on the same in a list.
    for f in range(len(list_details)):
        if list_details[f] == "null":
            continue
        elif list_details[f][2] != k:
            continue
        
        name_list_1.append(dict_borrower_name[list_details[f][0]])
        name_list_2.append([dict_borrower_name[list_details[f][0]],list_details[f][0],list_details[f][1]])
    
    book_list = []
    
    # Sorting the names in alphabetical order.
    name_list_1.sort()
    
    # Getting the person who comes first and storing the books borrowed by that person on that day in a list.
    for j in name_list_2:
        if j[0] == name_list_1[0]:
            key = j[1]
            book_list.append(j[2])
    
    # Sorting the the books borrowed by that person via id.
    book_list.sort()

    # Loop to output in the desired order.
    for i in range(len(list_details)):
        if list_details[i] == "null":
            continue
        elif list_details[i][2] != k:
            continue
        elif list_details[i][0] != key:
            continue
        elif list_details[i][1] != book_list[0]:
            continue
        
        print(list_details[i][2],end="~")  
        print(dict_borrower_name[list_details[i][0]],end="~") 
        print(list_details[i][1],end="~") 
        print(dict_book_name[list_details[i][1]])          
        
        list_details[i] = "null"
