import re
c = 0
for i in open('day5.txt'):
 m1 = len(re.findall("[aeiou]", i))
 m2 = len(re.findall(r"(.)\1", i))
 m3 = len(re.findall("(ab|cd|pq|xy)", i))
 m = (m1 >= 3) and (m2 > 0) and (m3 == 0)
 if m:
  c = c + 1
print c
