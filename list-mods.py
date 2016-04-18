import glob
import os
import re
import zipfile

nameRe = re.compile(r'"name"\s*:\s*"([^"]+)"')
mods = []

def guess(jar):
    return os.path.split(jar)[-1].split("-", 1)[0]

for jar in glob.glob("mods/*.jar"):
    zip = zipfile.ZipFile(jar)
    info = zip.read("mcmod.info").decode("utf-8")
    matches = re.findall(nameRe, info)
    
    if len(matches) == 0:
        mods.append(guess(jar))
        
        continue
    
    for name in matches:
        mods.append(name)

print("\n".join(mods))
