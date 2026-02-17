#!/usr/bin/env python3
"""Inventory TypeScript/JS files under a directory and group by top-level module.

Usage: python inventory_ts.py <path-to-src>
Writes a simple JSON object to stdout with counts per top-level folder.
"""
import os
import sys
import json

def walk(src):
    counts = {}
    for root, dirs, files in os.walk(src):
        for f in files:
            if f.endswith(('.ts', '.tsx', '.js', '.jsx')):
                rel = os.path.relpath(root, src)
                top = rel.split(os.sep)[0] if rel != '.' else '.'
                counts.setdefault(top, 0)
                counts[top] += 1
    return counts

def main():
    if len(sys.argv) < 2:
        print('Usage: inventory_ts.py <path-to-src>')
        sys.exit(2)
    src = sys.argv[1]
    counts = walk(src)
    print(json.dumps(counts, indent=2, sort_keys=True))

if __name__ == '__main__':
    main()
