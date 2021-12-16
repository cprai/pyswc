from sys import argv
from pyswc import minify


# Get filename from args
filename = argv[1]

# Read in file
src = open(filename).read()

# Minify cody using pyswc
minified = minify(filename, src)

# Print result
print(minified)
