#!/usr/bin/env python3
import sys
import os

folder_path = sys.argv[1]

for file_path in os.listdir(folder_path):
    abs_file_path = os.path.join(folder_path, file_path)

    os.system(f"../wavem/target/release/wavem {abs_file_path}")
