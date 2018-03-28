var last_lints = [];

$(document).ready(function(){
	var wikiEditorToolbarEnabled, useCodeMirror, codeMirror, observer;
	
	var base = "https://vroland.de/git";
	
	function escapeHtml(unsafe) {
	    return unsafe
	        .replace(/&/g, "&amp;")
	        .replace(/</g, "&lt;")
	        .replace(/>/g, "&gt;")
	        .replace(/"/g, "&quot;")
	        .replace(/'/g, "&#039;");
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
	    var xhttp = new XMLHttpRequest();
	    xhttp.onreadystatechange = function() {
	        if (this.readyState == 4 && this.status == 200) {
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
	        }
	    };
	    xhttp.open("POST", base, true);
	    xhttp.setRequestHeader("Content-type","application/x-www-form-urlencoded");
	    xhttp.send("source=" + encodeURIComponent(source)); 
	}
	
	function submit_source() {
	    var source = codeMirror.getValue("\n") + "\n";
	    fetch_lints(source);
	};

	function mutate_editor() {
		observer.disconnect();
		
		// The WikiEditor extension exists the WikiEditor beta toolbar is used by the user
		wikiEditorToolbarEnabled = !!mw.loader.getState( 'ext.wikiEditor' ) &&
			// This can be the string "0" if the user disabled the preference - Bug T54542#555387
			mw.user.options.get( 'usebetatoolbar' ) > 0;
			
		useCodeMirror = mw.user.options.get( 'usecodemirror' ) > 0;
		
		console.log("use code mirror: " + useCodeMirror);
		if (!useCodeMirror) {
			return
		}
		
		register_mfnf_extensions();
		
		codeMirror = $('#wpTextbox1').next('.CodeMirror')[0].CodeMirror;
		
		codeMirror.setOption("lineNumbers", true);
		codeMirror.setOption("lineWrapping", true);
		//codeMirror.setOption("matchBrackets", true);
		//codeMirror.setOption("styleActiveLine", true);
		codeMirror.setOption("gutters", ["CodeMirror-lint-markers"]);
		codeMirror.setOption("lint", true);
		codeMirror.on("changes", submit_source);
		console.log(codeMirror);
	
		submit_source();
	}
	
	// Create an observer instance linked to the callback function
	observer = new MutationObserver(mutate_editor);
	
	// Start observing the target node for configured mutations
	observer.observe(document.getElementById("wpTextbox1"), { attributes: true, childList: false });
            
	
});

function register_mfnf_extensions() {
	// mw-lint.js
	
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
	
	CodeMirror.registerHelper("lint", "mediawiki", function(text, options) {
	  var found = [];
	  
	  if (last_lints === null || last_lints === undefined) {
	    console.run("last lints is undefined, can't show lints in editor!");
	    return found;
	  }
	
	  last_lints.sort(cmp_severity);
	
	  for ( var i = 0; i < last_lints.length; i++) {
	    var lint = last_lints[i];
	    var severity = lint.severity;
	
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
	  console.log(found);
	  return found;
	});
	
	});
	
	// lint.js
	
	// CodeMirror, copyright (c) by Marijn Haverbeke and others
	// Distributed under an MIT license: http://codemirror.net/LICENSE
	
	(function(mod) {
	  if (typeof exports == "object" && typeof module == "object") // CommonJS
	    mod(require("../../lib/codemirror"));
	  else if (typeof define == "function" && define.amd) // AMD
	    define(["../../lib/codemirror"], mod);
	  else // Plain browser env
	    mod(CodeMirror);
	})(function(CodeMirror) {
	  "use strict";
	  var GUTTER_ID = "CodeMirror-lint-markers";
	
	  function showTooltip(e, content) {
	    var tt = document.createElement("div");
	    tt.className = "CodeMirror-lint-tooltip";
	    tt.appendChild(content.cloneNode(true));
	    document.body.appendChild(tt);
	
	    function position(e) {
	      if (!tt.parentNode) return CodeMirror.off(document, "mousemove", position);
	      tt.style.top = Math.max(0, e.clientY - tt.offsetHeight - 5) + "px";
	      tt.style.left = (e.clientX + 5) + "px";
	    }
	    CodeMirror.on(document, "mousemove", position);
	    position(e);
	    if (tt.style.opacity != null) tt.style.opacity = 1;
	    return tt;
	  }
	  function rm(elt) {
	    if (elt.parentNode) elt.parentNode.removeChild(elt);
	  }
	  function hideTooltip(tt) {
	    if (!tt.parentNode) return;
	    if (tt.style.opacity == null) rm(tt);
	    tt.style.opacity = 0;
	    setTimeout(function() { rm(tt); }, 600);
	  }
	
	  function showTooltipFor(e, content, node) {
	    var tooltip = showTooltip(e, content);
	    function hide() {
	      CodeMirror.off(node, "mouseout", hide);
	      if (tooltip) { hideTooltip(tooltip); tooltip = null; }
	    }
	    var poll = setInterval(function() {
	      if (tooltip) for (var n = node;; n = n.parentNode) {
	        if (n && n.nodeType == 11) n = n.host;
	        if (n == document.body) return;
	        if (!n) { hide(); break; }
	      }
	      if (!tooltip) return clearInterval(poll);
	    }, 400);
	    CodeMirror.on(node, "mouseout", hide);
	  }
	
	  function LintState(cm, options, hasGutter) {
	    this.marked = [];
	    this.options = options;
	    this.timeout = null;
	    this.hasGutter = hasGutter;
	    this.onMouseOver = function(e) { onMouseOver(cm, e); };
	    this.waitingFor = 0
	  }
	
	  function parseOptions(_cm, options) {
	    if (options instanceof Function) return {getAnnotations: options};
	    if (!options || options === true) options = {};
	    return options;
	  }
	
	  function clearMarks(cm) {
	    var state = cm.state.lint;
	    if (state.hasGutter) cm.clearGutter(GUTTER_ID);
	    for (var i = 0; i < state.marked.length; ++i)
	      state.marked[i].clear();
	    state.marked.length = 0;
	  }
	
	  function makeMarker(labels, severity, multiple, tooltips) {
	    var marker = document.createElement("div"), inner = marker;
	    marker.className = "CodeMirror-lint-marker-" + severity;
	    if (multiple) {
	      inner = marker.appendChild(document.createElement("div"));
	      inner.className = "CodeMirror-lint-marker-multiple";
	    }
	
	    if (tooltips != false) CodeMirror.on(inner, "mouseover", function(e) {
	      showTooltipFor(e, labels, inner);
	    });
	
	    return marker;
	  }
	
	  function getMaxSeverity(a, b) {
	    if (a == "error") return a;
	    else return b;
	  }
	
	  function groupByLine(annotations) {
	    var lines = [];
	    for (var i = 0; i < annotations.length; ++i) {
	      var ann = annotations[i], line = ann.from.line;
	      (lines[line] || (lines[line] = [])).push(ann);
	    }
	    return lines;
	  }
	
	  function annotationTooltip(ann) {
	    var severity = ann.severity;
	    if (!severity) severity = "error";
	    var tip = document.createElement("div");
	    tip.className = "CodeMirror-lint-message-" + severity;
	    if (typeof ann.messageHTML != 'undefined') {
	        tip.innerHTML = ann.messageHTML;
	    } else {
	        tip.appendChild(document.createTextNode(ann.message));
	    }
	    return tip;
	  }
	
	  function lintAsync(cm, getAnnotations, passOptions) {
	    var state = cm.state.lint
	    var id = ++state.waitingFor
	    function abort() {
	      id = -1
	      cm.off("change", abort)
	    }
	    cm.on("change", abort)
	    getAnnotations(cm.getValue(), function(annotations, arg2) {
	      cm.off("change", abort)
	      if (state.waitingFor != id) return
	      if (arg2 && annotations instanceof CodeMirror) annotations = arg2
	      cm.operation(function() {updateLinting(cm, annotations)})
	    }, passOptions, cm);
	  }
	
	  function startLinting(cm) {
	    var state = cm.state.lint, options = state.options;
	    /*
	     * Passing rules in `options` property prevents JSHint (and other linters) from complaining
	     * about unrecognized rules like `onUpdateLinting`, `delay`, `lintOnChange`, etc.
	     */
	    var passOptions = options.options || options;
	    var getAnnotations = options.getAnnotations || cm.getHelper(CodeMirror.Pos(0, 0), "lint");
	    if (!getAnnotations) return;
	    if (options.async || getAnnotations.async) {
	      lintAsync(cm, getAnnotations, passOptions)
	    } else {
	      var annotations = getAnnotations(cm.getValue(), passOptions, cm);
	      if (!annotations) return;
	      if (annotations.then) annotations.then(function(issues) {
	        cm.operation(function() {updateLinting(cm, issues)})
	      });
	      else cm.operation(function() {updateLinting(cm, annotations)})
	    }
	  }
	
	  function updateLinting(cm, annotationsNotSorted) {
	    clearMarks(cm);
	    var state = cm.state.lint, options = state.options;
	
	    var annotations = groupByLine(annotationsNotSorted);
	
	    for (var line = 0; line < annotations.length; ++line) {
	      var anns = annotations[line];
	      if (!anns) continue;
	
	      var maxSeverity = null;
	      var tipLabel = state.hasGutter && document.createDocumentFragment();
	
	      for (var i = 0; i < anns.length; ++i) {
	        var ann = anns[i];
	        var severity = ann.severity;
	        if (!severity) severity = "error";
	        maxSeverity = getMaxSeverity(maxSeverity, severity);
	
	        if (options.formatAnnotation) ann = options.formatAnnotation(ann);
	        if (state.hasGutter) tipLabel.appendChild(annotationTooltip(ann));
	
	        if (ann.to) state.marked.push(cm.markText(ann.from, ann.to, {
	          className: "CodeMirror-lint-mark-" + severity,
	          __annotation: ann
	        }));
	      }
	
	      if (state.hasGutter)
	        cm.setGutterMarker(line, GUTTER_ID, makeMarker(tipLabel, maxSeverity, anns.length > 1,
	                                                       state.options.tooltips));
	    }
	    if (options.onUpdateLinting) options.onUpdateLinting(annotationsNotSorted, annotations, cm);
	  }
	
	  function onChange(cm) {
	    var state = cm.state.lint;
	    if (!state) return;
	    clearTimeout(state.timeout);
	    state.timeout = setTimeout(function(){startLinting(cm);}, state.options.delay || 500);
	  }
	
	  function popupTooltips(annotations, e) {
	    var target = e.target || e.srcElement;
	    var tooltip = document.createDocumentFragment();
	    for (var i = 0; i < annotations.length; i++) {
	      var ann = annotations[i];
	      tooltip.appendChild(annotationTooltip(ann));
	    }
	    showTooltipFor(e, tooltip, target);
	  }
	
	  function onMouseOver(cm, e) {
	    var target = e.target || e.srcElement;
	    if (!/\bCodeMirror-lint-mark-/.test(target.className)) return;
	    var box = target.getBoundingClientRect(), x = (box.left + box.right) / 2, y = (box.top + box.bottom) / 2;
	    var spans = cm.findMarksAt(cm.coordsChar({left: x, top: y}, "client"));
	
	    var annotations = [];
	    for (var i = 0; i < spans.length; ++i) {
	      var ann = spans[i].__annotation;
	      if (ann) annotations.push(ann);
	    }
	    if (annotations.length) popupTooltips(annotations, e);
	  }
	
	  CodeMirror.defineOption("lint", false, function(cm, val, old) {
	    if (old && old != CodeMirror.Init) {
	      clearMarks(cm);
	      if (cm.state.lint.options.lintOnChange !== false)
	        cm.off("change", onChange);
	      CodeMirror.off(cm.getWrapperElement(), "mouseover", cm.state.lint.onMouseOver);
	      clearTimeout(cm.state.lint.timeout);
	      delete cm.state.lint;
	    }
	
	    if (val) {
	      var gutters = cm.getOption("gutters"), hasLintGutter = false;
	      for (var i = 0; i < gutters.length; ++i) if (gutters[i] == GUTTER_ID) hasLintGutter = true;
	      var state = cm.state.lint = new LintState(cm, parseOptions(cm, val), hasLintGutter);
	      if (state.options.lintOnChange !== false)
	        cm.on("change", onChange);
	      if (state.options.tooltips != false && state.options.tooltips != "gutter")
	        CodeMirror.on(cm.getWrapperElement(), "mouseover", state.onMouseOver);
	
	      startLinting(cm);
	    }
	  });
	
	  CodeMirror.defineExtension("performLint", function() {
	    if (this.state.lint) startLinting(this);
	  });
	});
};

