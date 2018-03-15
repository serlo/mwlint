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

function cmp_severity(a, b) {
    var values = {
        info: 10,
        warning: 20,
        error: 30
    };
    return values[b.severity] - values[a.severity];
}

CodeMirror.registerHelper("lint", "markdown", function(text, options) {
  var found = [];

  if (last_lints === null || last_lints === undefined) {
    console.run("last lints is undefined, can't show lints in editor!");
    return found;
  }

  last_lints.sort(cmp_severity);

  for ( var i = 0; i < last_lints.length; i++) {
    var lint = last_lints[i];
    var severity = lint.severity;
    console.log(lint);
    found.push({
      from: CodeMirror.Pos(lint.position.start.line - 1, lint.position.start.col - 1),
      to: CodeMirror.Pos(lint.position.end.line - 1, lint.position.end.col - 1),
      messageHTML: "<span class=\"explanation explanation-" + lint.severity + 
        "\">" + lint.explanation + "</span>" + 
        "<br><span class=\"solution\">" + lint.solution + "</span>" + 
        "<br><span class=\"explanation_long\">" + lint.explanation_long + "</span>",
      severity : severity
    });
  }
  return found;
});

});
