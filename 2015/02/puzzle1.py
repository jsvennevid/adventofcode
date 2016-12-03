import re

total = 0
for l in open('day2.txt'):
 m = re.match('([0-9]+)x([0-9]+)x([0-9]+)', l)
 l = int(m.group(1))
 w = int(m.group(2))
 h = int(m.group(3))
 area = (2*l*w)+(2*w*h)+(2*h*l)
 extra = min(l*w,w*h,h*l)
 total = total + area + extra
 print l, w, h, "=", area, extra
print "total", total
