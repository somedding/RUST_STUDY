import random

nums = list(range(1, 75+1))

random.shuffle(nums)
nums[12] = "*" #와일드카드 지정

for y in range(0,5):
    for x in range(0,5):
        print("{:>3},".format(nums[y*5+x]), end="")
    print("")

    