def read_input(day: int, sample: bool = False) -> str:
    """Read the input file for the given day"""
    if sample:
        filename = f"input/d{day:02d}.sample.dat"
    else:
        filename = f"input/d{day:02d}.dat"
        
    with open(filename) as file:
        return file.read()
