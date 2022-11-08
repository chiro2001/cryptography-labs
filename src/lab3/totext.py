#!/bin/env python3
import sys
from bs4 import BeautifulSoup as Soup

lines = sys.stdin.readlines()
print(Soup(''.join(lines), features="lxml").get_text().replace("\n\n", "\n").replace("\n\n", "\n"))