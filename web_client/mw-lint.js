// CodeMirror, copyright (c) by Marijn Haverbeke and others
// Distributed under an MIT license: http://codemirror.net/LICENSE

// Depends on csslint.js from https://github.com/stubbornella/csslint

// declare global: CSSLint

(function(mod) {
  if (typeof exports == "object" && typeof module == "object") // CommonJS
    mod(require("../../lib/codemirror"));
  else if (typeof define == "function" && define.amd) // AMD
    define(["../../lib/codemirror"], mod);
  else // Plain browser env
    mod(CodeMirror);
})(function(CodeMirror) {
"use strict";

CodeMirror.registerHelper("lint", "markdown", function(text, options) {
  var found = [];
  if (last_lints === null || last_lints === undefined) {
    console.run("last lints is undefined, can't show lints in editor!");
    return found;
  }

  for ( var i = 0; i < last_lints.length; i++) {
    var lint = last_lints[i];
    console.log(lint);
    var severity = lint.severity;

    found.push({
      from: CodeMirror.Pos(lint.position.start.line - 1, lint.position.start.col - 1),
      to: CodeMirror.Pos(lint.position.end.line - 1, lint.position.end.col - 1),
      message: lint.explanation + "\n=> try: " + lint.solution,
      severity : severity
    });
  }
  return found;
});

});
