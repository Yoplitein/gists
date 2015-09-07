#!/usr/bin/env python2

import random
import sys

from PIL import Image

limit = 50

def adjust(value):
    newValue = value + random.randrange(-limit, limit)
    
    return max(0, min(255, newValue))

def shift_color(color):
    return tuple(map(adjust, color[0:3]))

def shift(image):
    sx, sy = image.size
    hasAlpha = len(image.getpixel((0, 0))) == 4
    
    for x in xrange(0, sx):
        for y in xrange(0, sy):
            pos = (x, y)
            color = image.getpixel(pos)
            
            #ignore completely transparent pixels
            if hasAlpha and color[3] == 0:
                continue
            
            newColor = shift_color(color)
            
            #copy alpha
            if hasAlpha:
                newColor += (color[3],)
            
            image.putpixel(pos, newColor)

def main():
    if len(sys.argv) < 3:
        print "Usage: color_shift.py <input filename> <output filename> [limit]"
        
        return
    
    if len(sys.argv) >= 4:
        global limit
        
        try:
            limit = int(sys.argv[3])
        except ValueError:
            print sys.argv[3], "is not a number"
            
            return
    
    image = Image.open(sys.argv[1])
    
    shift(image)
    image.save(sys.argv[2])

if __name__ == '__main__':
    main()
