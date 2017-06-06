#!/usr/bin/env python

mimeList = ""
types = {}

with open("/etc/mime.types", "r") as f:
    mimeList = f.read().split("\n")

for entry in mimeList:
    entry = entry.split("\t")
    
    while '' in entry:
        entry.remove('')
    
    if len(entry) == 2: #entry is ["mime", "extension1 extension2"]
        mime, extensions = entry
        
        for extension in extensions.split(" "):
            types[mime] = types.get(mime, []) + [extension]

print("mimetype.assign = (")

for mime, extensions in types.iteritems():
    print('''    "{}" => "{}",'''.format(mime, " ".join(extensions)))

print(")")