import re
c = 0
for i in open('day5.txt'):
 m1 = len(re.findall(r'(.)(.).*\1\2', i))
 m2 = len(re.findall(r'(.)(.)\1', i))
 m = (m1 > 0) and (m2 > 0)
 if m:
  c = c + 1
print c
