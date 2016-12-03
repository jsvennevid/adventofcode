curr = 0
offset = 1
with open("day1.txt") as f:
 while True:
  c = f.read(1)
  if not c:
   break
  if c == '(':
   curr = curr + 1
  if c == ')':
   curr = curr - 1
  if curr == -1:
   break
  offset = offset + 1
print offset
