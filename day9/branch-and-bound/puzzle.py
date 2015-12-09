import re
import sys
import os
from collections import defaultdict, OrderedDict
from copy import copy

class SleighRouteCalculator:
 def __init__(this, routes):
  this.routes = defaultdict(lambda: {})

  for i in open(routes):
   m = re.match('([A-Z]+) to ([A-Z]+) = ([0-9]+)', i, flags=re.I)
   if m:
    this.routes[m.group(1)][m.group(2)] = int(m.group(3))
    this.routes[m.group(2)][m.group(1)] = int(m.group(3))
	
 def __search(this, source, visited, cost, compare):
  visited[source] = True
  if (len(visited) == len(this.routes)):
   if compare(cost, this.optimalCost):
    this.optimalRoute = visited.keys()
    this.optimalCost = cost
  else:
   destinations = filter(lambda x: not visited.has_key(x), this.routes[source])
   for destination in destinations:
    curr = this.routes[source][destination]
    if not compare(cost + curr, this.optimalCost):
     continue
    this.__search(destination, copy(visited), cost + curr, compare)

 def search(this, compare = lambda x, y: x < y):
  this.optimalRoute = []
  if compare(0, sys.maxint):
   this.optimalCost = sys.maxint
  else:
   this.optimalCost = 0
  for source in this.routes:
   this.__search(source, OrderedDict(), 0, compare)
  return (this.optimalRoute, this.optimalCost)


calculator = SleighRouteCalculator(os.path.join(os.path.dirname(os.path.realpath(__file__)), '../day9.txt'))
print "Shortest:", calculator.search()[1]
print "Longest:", calculator.search(compare = lambda x, y: x > y)[1]
