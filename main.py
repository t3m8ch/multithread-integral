a = 9
b = 36
threads = 8
delta = 0.1

numbers = []
i = a
while i <= b:
  numbers.append(i) 
  i += delta

thread = 0

for t in range(threads):
  print(f"t = {t}:", end=' ')
  for i in range((len(numbers) // threads) * t, (len(numbers) // threads) * (t + 1)):
    print(i, end=' ')
  print()

print(f"t = 0", end=' ')
for i in range((len(numbers) // threads) * threads, len(numbers)):
  print(i, end=' ')
