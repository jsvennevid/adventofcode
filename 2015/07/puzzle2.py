import re
wires = {}
for i in open('day7.txt'):
 set = re.match(r'([a-z0-9]+) -> ([a-z]+)',i)
 if set:
   wires[set.group(2)] = set.group(1)
 op1 = re.match(r'(NOT) ([a-z0-9]+) -> ([a-z]+)',i)
 if op1:
  wires[op1.group(3)] = [op1.group(1), op1.group(2)]
 op2 = re.match(r'([a-z0-9]+) (AND|OR|LSHIFT|RSHIFT) ([a-z0-9]+) -> ([a-z]+)',i)
 if op2:
  wires[op2.group(4)] = [op2.group(2), op2.group(1), op2.group(3)]

def visit(wire,results):
 if re.match(r'[0-9]+',wire):
  return int(wire)
 if results.has_key(wire):
  return results[wire]
 data = wires[wire]
 if not isinstance(data, list):
  return visit(data, results)
 value = {
  'NOT': lambda d: (~visit(d[1],results)) & 65535,
  'AND': lambda d: visit(d[1],results) & visit(d[2],results),
  'OR': lambda d: visit(d[1],results) | visit(d[2],results),
  'RSHIFT': lambda d: (visit(d[1],results) >> visit(d[2],results)) & 65535,
  'LSHIFT': lambda d: (visit(d[1],results) << visit(d[2],results)) & 65535
 }[data[0]](data)
 results[wire] = value
 return value

wires['b'] = str(visit('a', {}))
print 'a:', visit('a', {})
