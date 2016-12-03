import md5

with open('day4.txt') as f:
 key = f.readline().strip()

index = 0
while True:
 inp = key + str(index)
 m = md5.new()
 m.update(inp)
 d = m.hexdigest()
 if d.startswith('000000'):
  break
 index = index + 1
print index
