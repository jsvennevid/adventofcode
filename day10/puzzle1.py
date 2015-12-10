import re
with open('day10.txt') as f:
 inp = f.readline().strip()

for i in xrange(0, 40):
 inp = "".join(map(lambda x: str(len(x[0])) + x[1], re.findall(r'((.)\2*)', inp)))
  
print len(inp)
