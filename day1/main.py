
input = """1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"""

max_calories = 0
cur_calories = 0

with open('input.txt'):
    

for line in input.splitlines(keepends=True):
    if line == "\n":
        if cur_calories > max_calories:
            max_calories = cur_calories
            cur_calories = 0
            
        continue

    cur_calories += int(line)

print(max_calories)