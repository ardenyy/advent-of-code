all_lines = open("src/input2.txt").read().splitlines();

sum = 0

str_numbers = ["one", "two","three","four","five","six","seven","eight","nine"]
mathcing_numbers = range(1,10)

for line in all_lines:
    #print("")
    #print(line)
    digits, temp = [], ""
    for c in line:
        if c.isdigit():
            digits.append(c)
        else:
            temp += c
        for idx, num in enumerate(str_numbers):
            #print(temp)
            if num in temp:
                digits.append(str(mathcing_numbers[idx]))
                temp = temp[-1]
    number = digits[0] + digits[-1]
    sum += int(number)
    print(digits[0] + " " + digits[-1] + " " + str(sum))

print(sum)