#!/usr/bin/env dub
/+ dub.sdl:
    name "collage"
    dependency "imageformats" version="~>6.1.0"
    dependency "mustache-d" version="~>0.1.1"
+/

import std.algorithm;
import std.array;
import std.file;
import std.path;
import std.range;
import std.stdio;
import std.string;

import imageformats;
import mustache;

immutable validExtensions = [
    ".png",
    //".gif", //not supported by imageformats. :(
    ".jpg",
    ".jpeg",
];
immutable documentTemplate = `
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Collage</title>
    <style type="text/css">
        body
        {
            padding: 0px;
            margin: 0px;
            background-color: #d3d3d3;
        }
        
        img
        {
            float: left;
        }
    </style>
</head>
<body>
    {{{images}}}
</body>
</html>
`;
immutable wideImageTemplate = `
<img src="{{path}}" style="width: {{width}}px;"/>
`;
immutable tallImageTemplate = `
<img src="{{path}}" style="height: {{height}}px;"/>
`;

struct Image
{
    string path;
    int width;
    int height;
    
    this(string path)
    {
        this.path = path;
        int channels;
        
        read_image_info(path, width, height, channels);
    }
}

alias Mustache = MustacheEngine!string;

void main()
{
    auto images = dirEntries(".", SpanMode.shallow)
        .filter!(path => validExtensions.canFind(path.extension.toLower))
        .map!(path => Image(path))
        .array
    ;
    
    if(images.length == 0)
    {
        writeln("No images found!");
        
        return;
    }
    
    ulong targetWidth = images
        .map!(img => img.width)
        .reduce!min
    ;
    ulong targetHeight = images
        .map!(img => img.height)
        .reduce!min
    ;
    string[] resizedImages = new string[images.length];
    Mustache mustache;
    auto ctx = new Mustache.Context;
    
    foreach(index, image; images)
    {
        string tpl = wideImageTemplate;
        
        if(image.width < image.height)
            tpl = tallImageTemplate;
        
        ctx["path"] = image.path;
        ctx["width"] = targetWidth;
        ctx["height"] = targetHeight;
        resizedImages[index] = mustache.renderString(tpl, ctx).strip;
    }
    
    ctx["images"] = resizedImages.join("\n    ");
    
    std.file.write("out.html", mustache.renderString(documentTemplate, ctx));
}
