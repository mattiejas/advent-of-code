import os
import inspect
import numpy as np

def read(y, d, sample=False, dtype=None):
    return np.array([raw.strip() for raw in open(f"../y{y}/input/d{d}.{'sample.' if sample else ''}dat").readlines()], dtype=dtype)
