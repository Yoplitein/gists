#!/usr/bin/env python3
import sys, os, argparse
import os.path as p
from functools import cmp_to_key
from locale import strcoll

exampleCommands = """
Typical operation:
$ %(prog)s $CLEAN/basedir >clean.txt
$ %(prog)s -c clean.txt $DIRTY/basedir >dirty.txt

Diffing subdirectories:
$ %(prog)s -c clean.txt -b $DIRTY/basedir $DIRTY/basedir/innerdir >dirty_subset.txt
or:
$ %(prog)s -b $CLEAN/basedir $CLEAN/basedir/innerdir >clean_subset.txt
$ %(prog)s -c clean_subset.txt -b $DIRTY/basedir $DIRTY/basedir/innerdir >dirty_subset.txt
""".strip()

def log(*args, **kwargs):
	print(*args, **kwargs, file=sys.stderr)

def directory(path):
	path = p.abspath(path)
	if not p.isdir(path):
		log(f"Error: {path} is not a directory")
		raise SystemExit(1)
	return path

def pathCompare(left, right):
	leftDirs = left.count("/")
	rightDirs = right.count("/")
	if leftDirs != rightDirs:
		return -1 if leftDirs < rightDirs else 1
	return strcoll(left, right)

def main():
	parser = argparse.ArgumentParser(
		prog="treediff.py",
		epilog=exampleCommands,
		formatter_class=argparse.RawDescriptionHelpFormatter,
		
	)
	parser.add_argument("root",
		nargs=1,
		type=directory,
		help="Root directory to search for files.",
	)
	parser.add_argument("-c", "--clean",
		metavar="files.txt",
		dest="cleanFiles",
		type=argparse.FileType("rt", encoding="UTF-8"),
		help="Path to text file listing files in the clean directory. Diffing is disabled when unspecified.",
		
	)
	parser.add_argument("-b", "--base",
		metavar="dir",
		dest="basePath",
		type=directory,
		help=
			"Base directory to determine paths from, for diffing subsets of the dirty directory. " +
			"This should be the corresponding directory specified as a root when generating the clean list"
		,
	)
	parser.add_argument("-i", "--ignore",
		metavar="dir",
		dest="ignored",
		action="append",
		type=directory,
		help="Add a directory to ignore when gathering the list of files.",
	)
	args = parser.parse_args()
	
	root = args.root[0]
	
	outputPrefix = None
	if not args.basePath:
		outputPrefix = p.dirname(root)
	else:
		if not args.basePath in root:
			log(f"Error: base path must be a parent of diff root")
			raise SystemExit(1)
		
		outputPrefix = p.dirname(args.basePath)
	
	for dir in args.ignored:
		if not root in dir:
			log(f"Error: ignored dir `{dir}` is not a child of the root directory")
			raise SystemExit(1)
	
	clean = set()
	if args.cleanFiles:
		with args.cleanFiles:
			filename = args.cleanFiles.name
			for line, file in enumerate(args.cleanFiles):
				file = file.strip()
				if file == "": continue
				
				if not file.startswith(outputPrefix):
					itsRoot = file.split("/")[0] or "/"
					log(
						f"Error: {filename}:{line + 1} has base path `{itsRoot}` " +
						f"but expected base path is `{outputPrefix}`"
					)
					raise SystemExit(1)
				
				clean.add(file)
		
		if len(clean) == 0:
			log("Warning: clean files list is empty!")
		
		log(f"Read {len(clean)} files from {args.cleanFiles.name}")
	
	current = set()
	for dir, _, files in os.walk(root):
		if any(x in dir for x in args.ignored): continue
		
		base = p.relpath(dir, outputPrefix)
		for file in files:
			current.add(p.join(base, file))
	
	out = list((current - clean) if args.cleanFiles else current)
	out.sort(key=cmp_to_key(pathCompare))
	log(
		f"Discovered {len(current)} files",
		f", {len(out)} are dirty" if args.cleanFiles else "",
		sep=""
	)
	
	if not os.isatty(1):
		# just in case it was set to something else by envvars or something
		sys.stdout.reconfigure(encoding="utf-8")
	print("\n".join(out))

if __name__ == "__main__":
	main()
