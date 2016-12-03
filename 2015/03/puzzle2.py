x = [0,0]
y = [0,0]
houses = {}
i = 0
with open('day3.txt') as f:
 while True:
  address = ':'.join([str(x[i]),str(y[i])])
  presents = 0
  if houses.has_key(address):
   presents = houses[address]
  houses[address] = presents + 1
  c = f.read(1)
  if not c:
   break
  if c == '^':
   y[i] = y[i] - 1
  if c == 'v':
   y[i] = y[i] + 1
  if c == '>':
   x[i] = x[i] + 1
  if c == '<':
   x[i] = x[i] - 1
  i = (i + 1) & 1
print len(houses)
