# main.py
import sys
import os

if len(sys.argv) < 2:
    print("Usage: python main.py <file_path>")
    sys.exit(1)

file_path = sys.argv[1]

if not os.path.exists(file_path):
    print(f"File not found: {file_path}")
    sys.exit(1)

with open(file_path, 'r') as f:
    code = f.read()

exec(code)
