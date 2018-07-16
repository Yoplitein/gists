const cache = {};
const queue = [];

export function enqueueImage(path)
{
    if(cache.hasOwnProperty(path))
        return;
    
    queue.push(path);
}

export async function cacheImages()
{
    const promises = [];
    
    for(let path of queue)
    {
        const img = new Image();
        img.src = path;
        
        cache[path] = img;
        promises.push(new Promise(resolve => img.addEventListener("load", resolve)));
    }
    
    for(let promise of promises)
        await promise;
    
    queue.length = 0;
}

export function getImage(path)
{
    if(cache.hasOwnProperty(path))
        return cache[path];
    
    throw new Error(`Image at \`${path}\` is not cached!`);
}
