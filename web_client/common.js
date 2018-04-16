var mwlint_examples = [];

if (document.getElementById("wpTextbox1") != null) {
	$.getScript('https://tools-static.wmflabs.org/cdnjs/ajax/libs/codemirror/5.34.0/codemirror.min.js', function() {
		console.log("codemirror loading...");
		var s_lint, s_md, s_mb, s_al;
		mw.loader.load( 'https://tools-static.wmflabs.org/cdnjs/ajax/libs/codemirror/5.34.0/codemirror.min.css', 'text/css' );
		s_lint = $.getScript( 'https://tools-static.wmflabs.org/cdnjs/ajax/libs/codemirror/5.34.0/addon/lint/lint.min.js' );
		s_md = $.getScript( 'https://tools-static.wmflabs.org/cdnjs/ajax/libs/codemirror/5.34.0/mode/markdown/markdown.min.js' );
		s_mb = $.getScript( 'https://tools-static.wmflabs.org/cdnjs/ajax/libs/codemirror/5.34.0/addon/edit/matchbrackets.min.js' );
		s_al = $.getScript( 'https://tools-static.wmflabs.org/cdnjs/ajax/libs/codemirror/5.34.0/addon/selection/active-line.min.js' );
		s_ex = 

		$.when(s_lint, s_md, s_mb, s_al).done(() => {
			$.getJSON('https://tools.wmflabs.org/mwlint/examples/', (result) => {
				mwlint_examples = result;
				init_editor();				
			});
		});
	});
} else {
	console.log("MWLint: no editor. -> no codemirror modifications");
}

function init_editor() {
	register_mfnf_extensions();
	
	var wikiEditorToolbarEnabled, useCodeMirror, codeMirror; 
	
	var base = "https://tools.wmflabs.org/mwlint/";
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

	function escapeHtml (string) {
		return String(string).replace(/[&<>"'`=\/]/g, function (s) {
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
        return new Promise(function(resolve, reject){
            var xhttp = new XMLHttpRequest();
            xhttp.onreadystatechange = function() {
                if (this.readyState == 4 && this.status == 200) {
                    var last_lints;
                    var result = JSON.parse(this.responseText);
                    // errors
                    if (result.hasOwnProperty("Error")) {
        
                        // parse error
                        if (result.Error.hasOwnProperty("parseerror")) {
                            var lint = build_parse_lint(result.Error.parseerror);
                            last_lints = [lint]; 
                        }
        
                        // transformation errors
                        if (result.Error.hasOwnProperty("transformationerror")) {
                            var lint = build_transformation_lint(
                                result.Error.transformationerror
                            );
                            last_lints = [lint]
                        }
                    }
                    
                    // lints
                    if (result.hasOwnProperty("Lints")) {
                        last_lints = result.Lints;
                    }

                    resolve(last_lints);
                }
                if (this.readyState == 4 && this.status != 200) {
                    reject([]);
                }
            };
            xhttp.open("POST", base, true);
            xhttp.setRequestHeader("Content-type","application/x-www-form-urlencoded");
            xhttp.send("source=" + encodeURIComponent(source)); 
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
		mode: {name: "markdown"},
		viewportMargin: Infinity,
	});	

    function register_mfnf_extensions() { 
       
        // CodeMirror, copyright (c) by Marijn Haverbeke and others
        // Distributed under an MIT license: http://codemirror.net/LICENSE
        // Modified by Valentin Roland
        
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
                return new Promise(function(resolve, reject){
                    var source = text.replace(/\n\r/g, "\n");
                    fetch_lints(source).then(function(lints){
                        var found = [];
                        lints.sort(cmp_severity); 
                        for ( var i = 0; i < lints.length; i++) {
                            var lint = lints[i];
                            var severity = lint.severity;
							var examples = get_examples(lint.kind);
							var example_html = "";
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
									"<div class=\"example-container\">" + 
											"<div class=\"example-header\">Examples:</div>" +
											example_html + 
									"</div>",	
                                severity : severity
                            });
                        }
                        resolve(found);
                    }, function() {
                        console.log("last lints is undefined, can't show lints in editor! -> maybe requests where blocked?");
                        reject([]);
                    });
                });
            });
        });	
    };
}

