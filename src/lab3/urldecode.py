#!/bin/env python3
import sys
raw = ''.join(sys.stdin.readlines())#.replace("%80", "%ef%bf%bd")
f = open(1, "wb")
if '%' in raw:
  raw_url = raw[raw.index('%'):raw.rindex('%') + 3]
  bits = [int(x.strip(), 16) for x in raw_url.split("%") if len(x.strip()) > 0]
  for b in raw[:raw.index('%')]:
    f.write(ord(b).to_bytes(1, 'little'))
  for b in bits:
    f.write(b.to_bytes(1, 'little'))
  for b in raw[raw.rindex('%') + 3:]:
    f.write(ord(b).to_bytes(1, 'little'))
else:
  for b in raw:
    f.write(ord(b).to_bytes(1, 'little'))