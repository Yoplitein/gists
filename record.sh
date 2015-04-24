#!/bin/bash

if [ -z "$@" ]; then
    echo "./record.sh <filename.webm>"
    exit 1
fi

SELECTION=`xrectsel`
INRES=`echo $SELECTION | cut -d"+" -f1`
WIDTH=`echo $INRES | cut -d"x" -f1`
HEIGHT=`echo $INRES | cut -d"x" -f2`

if expr $WIDTH % 2 != 0 >/dev/null; then
    WIDTH=`expr $WIDTH - 1`
fi

if expr $HEIGHT % 2 != 0 >/dev/null; then
    HEIGHT=`expr $HEIGHT - 1`
fi

INRES="${WIDTH}x${HEIGHT}"
INPUT=`echo $SELECTION | cut -d"+" -f2`,`echo $SELECTION | cut -d"+" -f3`
FPS="30"
THREADS="`expr $(grep ^processor /proc/cpuinfo | wc -l) - 1`"
RATE="1000k"
QUALITY="ultrafast"
TEMP=`mktemp -u`.mp4

ffmpeg -f x11grab -s $INRES -r $FPS -i :0.0+$INPUT -c:v libx264 -qp 0 -preset $QUALITY -threads $THREADS $TEMP
ffmpeg -i $TEMP -c:v libvpx -minrate $RATE -maxrate $RATE -b:v $RATE -threads $THREADS $@
rm -f $TEMP
