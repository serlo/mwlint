var mwlint_examples = [];

function add_script(url) {
    return new Promise((resolve, reject) => {
        var script = document.createElement('script');
        document.body.appendChild(script);
        script.onload = resolve;
        script.onerror = reject;
        script.async = true;
        script.src = url;
    })
};

if (mw.config.get("wgPageName").startsWith("Mathe_für_Nicht-Freaks:") && document.getElementById("wpTextbox1") != null) {
	var linter = add_script('https://tools-static.wmflabs.org/mwlint/wasm/mwlint_wasm.js').then(() => {
		return window.wasm_bindgen('https://tools.wmflabs.org/mwlint/static/wasm/mwlint_wasm_bg.wasm');
	});
  add_script('https://tools-static.wmflabs.org/cdnjs/ajax/libs/codemirror/5.34.0/codemirror.min.js').then(() => {
    mw.loader.load('https://tools-static.wmflabs.org/cdnjs/ajax/libs/codemirror/5.34.0/codemirror.min.css', 'text/css');
    
    var lint_addon = add_script('https://tools-static.wmflabs.org/mwlint/lint.js');
    var markdown = add_script('https://tools-static.wmflabs.org/cdnjs/ajax/libs/codemirror/5.34.0/mode/markdown/markdown.min.js');
    var brackets= add_script('https://tools-static.wmflabs.org/cdnjs/ajax/libs/codemirror/5.34.0/addon/edit/matchbrackets.min.js');
    var active_line = add_script('https://tools-static.wmflabs.org/cdnjs/ajax/libs/codemirror/5.34.0/addon/selection/active-line.min.js');
    
      Promise.all([lint_addon, markdown, brackets, active_line, linter]).then((values) => {
		mwlint_examples = JSON.parse(window.wasm_bindgen.examples());
		init_editor();
      });
  });
};

function init_editor() {
  register_mfnf_extensions();

  var wikiEditorToolbarEnabled, useCodeMirror, codeMirror;

  var entityMap = {
    '&': '&amp;',
    '<': '&lt;',
    '>': '&gt;',
    '"': '&quot;',
    "'": '&#39;',
    '/': '&#x2F;',
    '`': '&#x60;',
    '=': '&#x3D;'
  };

  function escapeHtml(string) {
    return String(string).replace(/[&<>"'`=\/]/g, function(s) {
      return entityMap[s];
    });
  }

  function build_parse_lint(err) {
    var expected = escapeHtml(err.expected.join(", "));
    return {
      position: {
        start: {
          line: err.position.line,
          col: err.position.col,
        },
        end: {
          line: err.position.line,
          col: err.position.col,
        }
      },
      severity: "error",
      explanation_long: "A syntax error occurs when there are mistakes in your code which make it impossible for analyse you document. These are often missing closing brackets and the likes. Also check the surrounding code, as the mistake might have happend before or after the given position.",
      explanation: "Syntax error!",
      solution: "Expected one of: " + expected,
    };
  }

  function build_transformation_lint(err) {
    return {
      severity: "error",
      explanation_long: "A transformation error occurs when a document could not be properly processed after parsing. This can occur if you have a peculiar heading or list structure.",
      explanation: err.cause,
      position: err.position,
      solution: "somehow this document does not conform with the usual document structure...",
    };
  }

  function fetch_lints(source) {
    return new Promise(function(resolve, reject) {
      var result = JSON.parse(window.wasm_bindgen.lint(source));
      // errors
      if (result.hasOwnProperty("Err")) {

        // parse error
        if (result.Err.Error.hasOwnProperty("parseerror")) {
          resolve([build_parse_lint(result.Err.Error.parseerror)]);
        }

        // transformation errors
        if (result.Err.Error.hasOwnProperty("transformationerror")) {
          resolve([build_transformation_lint(
            result.Err.Error.transformationerror
          )]);
        }
      }

      // lints
      if (result.hasOwnProperty("Ok")) {
        resolve(result.Ok.Lints);
      }
      reject([]);
    });
  }

  function get_examples(kind) {
    var result = [];
    for (var index = 0; index < mwlint_examples.length; index++) {
      var example = mwlint_examples[index];
      if (example.kind == kind) {
        result.push(example);
      }
    }
    return result;
  }

  codeMirror = CodeMirror.fromTextArea(document.getElementById("wpTextbox1"), {
    lineNumbers: true,
    lineWrapping: true,
    autofocus: true,
    styleActiveLine: true,
    matchBrackets: true,
    gutters: ["CodeMirror-lint-markers"],
    lint: true,
    mode: {
      name: "markdown"
    },
    viewportMargin: Infinity,
  });

  function show_summary(stats) {
    var severities = ["error", "warning", "info"];
    var box = document.getElementById("mwlint-stats-box");
    if (!box) {
      var toolbar = document.getElementById("wikiEditor-ui-toolbar");
      if (!toolbar) { return };
      var box = document.createElement('div');
      box.setAttribute("id", "mwlint-stats-box")
      toolbar.insertBefore(box, toolbar.getElementsByClassName("sections")[0]);
      for (var i=0; i<severities.length; i++) {
        var elem = document.createElement('span');
        elem.setAttribute("id", "mwlint-stats-" + severities[i]);
        elem.setAttribute("class", "mwlint-stat CodeMirror-lint-marker-" + severities[i]);
        box.appendChild(elem);
      }
    }
    for (var i=0; i<severities.length; i++) {
      var severity = severities[i];
      var elem = document.getElementById("mwlint-stats-" + severity);
      if (elem) {
        if (stats.hasOwnProperty(severity)) {
          elem.textContent = stats[severity];
        } else {
          elem.textContent = "✓"
        }
      }
    }
  } 

  function register_mfnf_extensions() {

    // CodeMirror, copyright (c) by Marijn Haverbeke and others
    // Distributed under an MIT license: http://codemirror.net/LICENSE
    // Modified by Valentin Roland

    (function(mod) {
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
        return new Promise(function(resolve, reject) {
          var source = text.replace(/\n\r/g, "\n");
          fetch_lints(source).then(function(lints) {
            var found = [];
			var lint_counts = {};
            lints.sort(cmp_severity);
            for (var i = 0; i < lints.length; i++) {
              var lint = lints[i];
              var severity = lint.severity;
              var examples = get_examples(lint.kind);
              var example_html = "";

              if (!lint_counts[severity]) {
                lint_counts[severity] = 0;
              }
      			  lint_counts[severity] = lint_counts[severity] + 1

              for (var j = 0; j < examples.length; j++) {
                var example = examples[j];
                example_html = example_html + "<div class=\"example\">" +
                  "<div class=\"example-bad-tag\">bad:</div>" +
                  "<div class=\"example-bad\">" + escapeHtml(example.bad) + "</div>" +
                  "<div class=\"example-bad-expl\">" + escapeHtml(example.bad_explanation) + "</div>" +
                  "<div class=\"example-good-tag\">good:</div>" +

                  "<div class=\"example-good\">" + escapeHtml(example.good) + "</div>" +
                  "<div class=\"example-good-expl\">" + escapeHtml(example.good_explanation) + "</div>" +
                  "</div>"
              }

              found.push({
                from: CodeMirror.Pos(lint.position.start.line - 1, lint.position.start.col - 1),
                to: CodeMirror.Pos(lint.position.end.line - 1, lint.position.end.col - 1),
                messageHTML: "<div class=\"explanation explanation-" + lint.severity +
                  "\">" + escapeHtml(lint.explanation) + "</div>" +
                  "<div class=\"solution\">" + "&#8618; " + escapeHtml(lint.solution) + "</div>" +
                  "<div class=\"explanation_long\">" + escapeHtml(lint.explanation_long) + "</div>" +
				  "<hr class=\"example-sep\">" + 
                  "<div class=\"example-container\">" +
                  "<div class=\"example-header\">Examples:</div>" +
                  example_html +
                  "</div>",
                severity: severity
              });
            }
            show_summary(lint_counts);
            resolve(found);
          }, function() {
            console.log("getting lints failed! Maybe wasm loading error?");
            reject([]);
          });
        });
      });
    });
  };
}
