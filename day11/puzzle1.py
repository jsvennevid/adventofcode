import re
with open('day11.txt') as f:
 inp = f.readline().strip()

base = ord('a')
maxc = ord('z') - base

def mutate(inp):
 data = list(reversed(inp))
 overflow = 1
 for i, c in enumerate(data):
  if overflow == 0:
   break
  nc = (ord(c)-base)+overflow
  overflow = max(0, nc - maxc)
  data[i] = chr((nc % (maxc + 1)) + base)
 if overflow > 0:
  data.append(chr((overflow - 1) + base))
 return "".join(reversed(data))

iol = re.compile(r"[iol]")
aabb = re.compile(r"(.)\1.*(.)\2")
def collide(inp):
 valid = any(map(lambda ofs: (ord(inp[ofs])+2 == ord(inp[ofs+1])+1 == ord(inp[ofs+2])), xrange(0, max(0,len(inp)-2))))
 valid = valid and (iol.search(inp) == None)
 valid = valid and (aabb.search(inp) != None)
 return not valid

while True:
 inp = mutate(inp)
 if not collide(inp):
  break

print inp
