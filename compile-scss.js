document.addEventListener(
    "DOMContentLoaded",
    async function()
    {
        const scssCompile = src => new Promise(resolve => Sass.compile(src, resolve));
        
        for(const script of document.querySelectorAll("script[type='text/scss']"))
        {
            const scssSource = script.src != "" ?
                await (await fetch(script.src)).text() :
                script.innerHTML
            ;
            const compiled = await scssCompile(scssSource);
            
            if(compiled.status != 0)
                throw new Error("Failed to compile");
            
            const stylesheet = document.createElement("style");
            stylesheet.type = "text/css";
            stylesheet.innerHTML = compiled.text;
            
            script.parentNode.insertBefore(stylesheet, script.nextSibiling);
        }
    }
);
