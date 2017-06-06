#!/usr/bin/env python2

mimeList = ""
mapping = {}

with open("/etc/mime.types", "r") as f:
    mimeList = f.read().split("\n")

for entry in mimeList:
    entry = entry.split("\t")

    while '' in entry:
        entry.remove('')

    if len(entry) == 2: #entry is ["mime", "extension1 extension2"]
        mime, extensions = entry

        for extension in extensions.split(" "):
            mapping[extension] = mime

print("mimetype.assign = (")

for extension, mime in mapping.iteritems():
    print('''    "{}" => "{}",'''.format(extension, mime))

print(")")