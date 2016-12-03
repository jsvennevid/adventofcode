import re
with open('day11.txt') as f:
 inp = f.readline().strip()

base = ord('a')
maxc = ord('z') - base

def mutate(inp):
 mutate.overflow = 1
 def increment(v):
  nc = (ord(v)-base)+mutate.overflow
  mutate.overflow = max(0, nc - maxc)
  return chr((nc % (maxc + 1)) + base)
 return "".join(reversed(map(increment, reversed(inp))))

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
