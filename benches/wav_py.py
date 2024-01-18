#!/usr/bin/env python3
import sys
import os
from scipy.io import wavfile

folder_path = sys.argv[1]

for file_path in os.listdir(folder_path):
    abs_file_path = os.path.join(folder_path, file_path)
    samplerate, data = wavfile.read(abs_file_path)

    data = data if len(data.shape) < 2 else data[:, 0]

    print(f"Done Py. Length {data.shape[0]}")
