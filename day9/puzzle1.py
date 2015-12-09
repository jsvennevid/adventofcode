import re
import sys
from collections import defaultdict
from copy import copy

locations = defaultdict(lambda: {})
for i in open('day9.txt'):
 m = re.match('([A-Z]+) to ([A-Z]+) = ([0-9]+)', i, flags=re.I)
 if m:
  locations[m.group(1)][m.group(2)] = int(m.group(3))
  locations[m.group(2)][m.group(1)] = int(m.group(3))

def minroute(curr, locations, visited, length):
 visited[curr] = True
 destinations = filter(lambda x: not visited.has_key(x), locations[curr].keys())
 if not destinations:
  return length
 minlength = sys.maxint
 for target in destinations:
  if (visited.has_key(target)):
   continue
  minlength = min(minlength, minroute(target, locations, copy(visited), length + locations[curr][target]))
 return minlength

minlength = sys.maxint
for location in locations:
 minlength = min(minlength, minroute(location, locations, {}, 0))
print minlength
