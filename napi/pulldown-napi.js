const { transform, transformNoHighlight } = require('./index');

const defaultOptions = {
  enableTables: true,
  enableFootnotes: true,
  enableStrikethrough: true,
  enableTasklists: true,
  enableSmartPunctuation: true,
  enableHeadingAttributes: true,
  enableContainer: false,
  enableMdx: false,
  codeBlockTheme: 'InspiredGitHub',
}

module.exports.transform = (content, options = {}) => {
  const opts = Object.assign(defaultOptions, options);
  return transform(content, opts);
}
module.exports.transformNoHighlight = (content, options = {}) => {
  const opts = Object.assign(defaultOptions, options);
  return transformNoHighlight(content, opts);
};