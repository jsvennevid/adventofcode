import re
from collections import defaultdict
from copy import copy

locations = defaultdict(lambda: {})
for i in open(os.path.join(os.path.dirname(os.path.realpath(__file__)), '../day9.txt')):
 m = re.match('([A-Z]+) to ([A-Z]+) = ([0-9]+)', i, flags=re.I)
 if m:
  locations[m.group(1)][m.group(2)] = int(m.group(3))
  locations[m.group(2)][m.group(1)] = int(m.group(3))

def maxroute(curr, locations, visited, length):
 visited[curr] = True
 destinations = filter(lambda x: not visited.has_key(x), locations[curr].keys())
 if not destinations:
  return length
 maxlength = 0
 for target in destinations:
  if (visited.has_key(target)):
   continue
  maxlength = max(maxlength, maxroute(target, locations, copy(visited), length + locations[curr][target]))
 return maxlength

maxlength = 0
for location in locations:
 maxlength = max(maxlength, maxroute(location, locations, {}, 0))
print maxlength
