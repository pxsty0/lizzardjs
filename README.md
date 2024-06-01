
# lizzardjs
JavaScript runtimer built with Rust



## Authors

- [@pxsty0](https://www.github.com/pxsty0) main and sole developer

  
## Usage and Example

```javascript
console.log("developed by mustafa 'pxsty' kok");

console.time();
console.timeEnd();

require("./printInfo.js")();
/* printInfo.js
() => {
  let device = process.os;

  console.log(`işletim sistemi: ${device.type()}`);
  console.log(`işletim sistemi versiyonu: ${device.version()}`);
  console.log(`işletim sistemi bit : ${device.bitness()}`);
};

 */
const filePath = `C:/`;

console.log(lizzard.fs.exists(filePath));
console.log(lizzard.fs.writeFile("./mustafa.txt", "pxsty"));
console.log(lizzard.fs.readFile("./mustafa.txt"));


```

  
