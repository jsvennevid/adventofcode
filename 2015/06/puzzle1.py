import re
grid = ([0] * 1000) * 1000
for i in open('day6.txt'):
 on = re.match(r"turn on ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)",i)
 if on:
  for y in xrange(int(on.group(2)), int(on.group(4)) + 1):
   for x in xrange(int(on.group(1)), int(on.group(3)) + 1):
    grid[x + y * 1000] = 1 
  continue
 off = re.match(r"turn off ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)",i)
 if off:
  for y in xrange(int(off.group(2)), int(off.group(4)) + 1):
   for x in xrange(int(off.group(1)), int(off.group(3)) + 1):
    grid[x + y * 1000] = 0 
  continue
 toggle = re.match(r"toggle ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)",i)
 if toggle:
  for y in xrange(int(toggle.group(2)), int(toggle.group(4)) + 1):
   for x in xrange(int(toggle.group(1)), int(toggle.group(3)) + 1):
    grid[x + y * 1000] = (grid[x + y * 1000] + 1) & 1 
  continue
print grid.count(1)
