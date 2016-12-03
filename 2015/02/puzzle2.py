import re

total = 0
for l in open('day2.txt'):
 m = re.match('([0-9]+)x([0-9]+)x([0-9]+)', l)
 l = int(m.group(1))
 w = int(m.group(2))
 h = int(m.group(3))
 lw = 2*(l+w)
 wh = 2*(w+h)
 hl = 2*(h+l)
 wrap = min(lw,wh,hl)
 ribbon = l * w * h
 total = total + wrap + ribbon
 print l, w, h, "=", wrap, ribbon 
print "total", total
