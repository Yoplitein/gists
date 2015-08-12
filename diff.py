import difflib
import gzip
import sys

import pynbt

def main():
    if len(sys.argv) < 2:
        print "diff.py <agedata_x.dat>"
        
        return
    
    file = sys.argv[1]
    nbt = None
    
    with gzip.GzipFile(file) as f:
        nbt = pynbt.NBTFile(f)
    
    data = nbt["data"]
    pages = map(lambda x: x["symbol"].value.encode("utf-8") + "\n", data["Pages"][1:])
    symbols = map(lambda x: x.value.encode("utf-8") + "\n", data["Symbols"][:])
    
    for line in difflib.context_diff(pages, symbols, fromfile="Pages", tofile="Resulting Symbols"):
        print line,

if __name__ == '__main__':
    main()
