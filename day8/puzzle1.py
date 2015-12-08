import re

chars = 0
mem = 0

def repl(p):
 return {
  r'\"': lambda x: '"',
  r'\\': lambda x: '\\',
  r'\x': lambda x: x.decode('hex')
 }[filter(lambda x: x is not None, [p.group(2), p.group(4), p.group(5)])[0]](p.group(3))

for i in open('day8.txt'):
 line = i.strip()
 chars += len(line)
 m = re.match('^"(.*)"$', line)
 if m != None:
  s = re.sub(r'((\\x)([0-9A-F]{2})|(\\")|(\\\\))', repl, m.group(1), flags=re.I)
  mem += len(s)
print chars - mem
