#!/bin/bash

cacheFile=~/bin/updates.cache
cacheFileNew=~/bin/updates-new.cache

function getignores()
{
    #list ignored packages here, like so:
    #echo "packagename"
    #TODO: actually parse pacman.conf
}

function getupdates()
{
    res="$(checkupdates)"

    for ignore in $(getignores); do
        res="$(echo "$res" | grep -v "^$ignore$")"
    done

    for package in $res; do
        echo $package
    done
}

function getdiff()
{
    echo "$(diff $cacheFile $cacheFileNew | grep "^>" | cut -d " " -f 2)"
}

function main()
{
    echo "$(getupdates)" > $cacheFileNew

    diff="$(getdiff)"

    if [ "$diff" != "" ]; then
        echo "The following packages are ready for upgrade:"
        echo "$diff"

        echo "$(cat $cacheFileNew)" > $cacheFile
    fi
}

main
