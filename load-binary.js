const fs = require("fs");

module.exports = function(source)
{
    const buffer = fs.readFileSync(this.resourcePath)
    
    return `module.exports = Uint8Array.from([${buffer.toJSON().data}]);`;
}
