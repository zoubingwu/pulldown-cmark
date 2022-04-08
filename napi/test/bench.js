const { readFileSync } = require('fs')
const { join } = require('path')
const { execSync } = require('child_process');
const { Suite } = require('benchmark')
const MarkdownIt = require('markdown-it')
const { marked } = require('marked')
const pulldown = require('../')

const md = new MarkdownIt()
const suite = new Suite()

const content = readFileSync(join(__dirname, './fixture.md'), 'utf8')

!(async () => {
  // use import() to load esm module in cjs
  const { remark } = await import('remark')
  const { default: remarkHtml } = await import('remark-html')
  const { default: chalk } = await import('chalk')
  const { micromark } = await import('micromark')

  suite.add('pulldown with napi', () => {
    pulldown.transform(content)
  }).add('pulldown with napi no highlight', () => {
    pulldown.transformNoHighlight(content)
  }).add('marked', () => {
    marked.parse(content)
  }).add('markdown-it', () => {
    md.render(content)
  }).add('remark', () => {
    remark().use(remarkHtml).processSync(content).toString()
  }).add('micromark', () => {
    micromark(content)
  }).on('cycle', event => {
    console.log(String(event.target));
  })
  .on('complete', function() {
    console.info(`Fastest is ${chalk.green(this.filter('fastest').map('name'))}!`)
  })
  .run({ 'async': true });
})();
