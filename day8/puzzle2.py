import re

chars = 0
mem = 0

def repl(p):
 return {
  '"': lambda: '\\"',
  '\\': lambda: '\\\\',
 }[filter(lambda x: x is not None, [p.group(0), p.group(1)])[0]]()

for i in open('day8.txt'):
 line = i.strip()
 chars += len(line)
 s = re.sub(r'(?:(\")|(\\))', repl, line, flags=re.I)
 mem += len(s) + 2
print mem - chars
