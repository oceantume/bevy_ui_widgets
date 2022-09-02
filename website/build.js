const process = require('process');
const path = require('path');
const fs = require('fs');

const getDirArg = (i) => {
  const dir = process.argv[i];
  if (typeof dir !== 'string' || !fs.statSync(dir).isDirectory()) {
    throw new Error(`Invalid directory argument ${i}: ${dir}`);
  }
  return dir;
}

const inputDir = getDirArg(2);
const outputDir = getDirArg(3);
const examplesInputDir = getDirArg(4);

console.log('building website with args:', inputDir, outputDir, examplesInputDir);

const examplesOutputDir = path.resolve(outputDir, 'examples');
fs.mkdirSync(examplesOutputDir);

const exampleFiles = fs.readdirSync(examplesInputDir);

console.log(`copying ${exampleFiles.length} files to examples directory`);
console.log(exampleFiles.join(', '));

exampleFiles.forEach(file => {
  fs.copyFileSync(path.join(examplesInputDir, file), path.join(examplesOutputDir, file));
});

const examples = exampleFiles
  .map(file => file.match(/(.+)\.js$/))
  .filter(match => match)
  .map(match => match[1]);

console.log(`found ${examples.length} examples`);
console.log(examples.join(', '));

const template = fs.readFileSync(path.resolve(inputDir, 'index.html'), 'utf8');

// fill template
const result = template.replace('{{examples}}', examples.map(name => 
  `<li><a href="#example:${name}">${name}</a></li>`
).join(''));

fs.writeFileSync(path.resolve(outputDir, 'index.html'), result);

console.log('website build done.')
