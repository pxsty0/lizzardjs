# lizzardjs

A JavaScript Runtimer developed with Rust using the Google V8 Engine

## Authors

- [@pxsty0](https://www.github.com/pxsty0) main developer

## Usage and Example

--> index.js

```javascript
console.log("developed by mustafa 'pxsty' kok"); // prints the entered arguments on a new line in order
console.time(); // starts the timer optional parameter can be entered uses the “default” parameter as standard
console.timeEnd(); // terminates the timer and prints the elapsed time

process; // It contains 2 objects, os and env
process.os; // contains functions that return information about the system

console.log(process.os.type()); // prints the type of operating system
console.log(process.os.version()); // prints the version of the operating system
console.log(process.os.bitness()); // prints how many bits the operating system is

process.env; // used to read data from the env file

console.log(process.env.developer); // prints data with the key “developer” in env

require; // modules are used when calling
require("./module.js")(); // executes the called module

lizzard; // contains the internal libraries of lizzardjs, currently contains fs

lizzard.fs; // internal library used to manage the file system

console.log(lizzard.fs.writeFile("./lizzard.txt", "mustafa 'pxsty' kok")); //writes the data "mustafa ’pxsty' kok” to the file “lizzard.txt'
console.log(lizzard.fs.appendFile("./lizzard.txt", "\npxsty")); // Adds “pxsty” data to “lizzard.txt” file
console.log(lizzard.fs.exists("./lizzard.txt")); // Checks for the existence of the file “lizzard.txt”
console.log(lizzard.fs.readFile("./lizzard.txt")); // Reads the file “lizzard.txt”
console.log(lizzard.fs.mkdir("./", "lizzard")); // Creates the “lizzard” folder in the path “./”

lizzard
  .fetch("https://httpbin.org/post", {
    method: "POST", // or GET,HEAD,POST,PUT,DELETE AND PATCH
    body: JSON.stringify({ run_with: "LizzardJS" }),
  })
  .then((data) => {
    console.log("reject output returned");
    console.log(`statusCode : ${data.statusCode}`);
    console.log(`response : ${data.response}`);
  })
  .catch((err) => {
    console.log(`reject output returned : ${err}`);
  });
```

--> printInfo.js

```javascript
() => {
  return "developed by mustafa 'pxsty' kok";
};
```

--> .env

```cs
developer=mustafa pxsty kok
```
