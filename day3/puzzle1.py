x = 0
y = 0
houses = {}
with open('day3.txt') as f:
 while True:
  address = ':'.join([str(x),str(y)])
  presents = 0
  if houses.has_key(address):
   presents = houses[address]
  houses[address] = presents + 1
  c = f.read(1)
  if not c:
   break
  if c == '^':
   y = y - 1
  if c == 'v':
   y = y + 1
  if c == '>':
   x = x + 1
  if c == '<':
   x = x - 1
print len(houses)
