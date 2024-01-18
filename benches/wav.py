#!/usr/bin/env python3
import sys
from scipy.io import wavfile

file_path = sys.argv[1]
samplerate, data = wavfile.read(file_path)

data = data if len(data.shape) < 2 else data[:, 0]

print(f"Done Py. Length {data.shape[0]}")
