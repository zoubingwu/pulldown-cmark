const { readFileSync } = require('fs');
const path = require('path');
const test = require('ava');
const { transform } = require('..');


test('pulldown-napi should work', t => {
  const content = readFileSync(path.join(__dirname, './fixture.md'), 'utf-8');
  const result = transform(content);
  console.log(result)
  t.snapshot(result);
})