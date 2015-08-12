import gzip
import sys

import pynbt

def main():
    if len(sys.argv) < 2:
        print "list.py <agedata_x.dat>"
        
        return
    
    file = sys.argv[1]
    nbt = None
    
    with gzip.GzipFile(file) as f:
        nbt = pynbt.NBTFile(f)
    
    data = nbt["data"]
    pages = map(lambda x: x["symbol"].value.encode("utf-8") + "\n", data["Pages"][1:])
    header = " {} pages ".format(len(pages))
    
    print header
    print "=" * len(header)
    
    for index, page in enumerate(pages):
        print "{:3d} {}".format(index + 1, page.strip())

if __name__ == '__main__':
    main()
