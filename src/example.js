console.log("Hello, runjs!\n")
console.error("fuckkkkk")

const path = "./log.txt"
try {
  const contents = await runjs.readFile(path);
  console.log(contents)
} catch (err) {
  console.error("Unable to read file", path, err);
}

await runjs.writeFile(path, "New content writed");
const contents  = await runjs.readFile(path);
console.log(contents);
runjs.removeFile(path);
console.log("File removed");
