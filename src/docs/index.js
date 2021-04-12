(function () {
    var arrows = document.querySelectorAll(".colapsable");
    var _loop_1 = function (i) {
        var image = i.querySelector("img");
        image.addEventListener("click", function () {
            var classList = i.classList;
            if (classList.contains("colapsed")) {
                classList.remove("colapsed");
            }
            else {
                classList.add("colapsed");
            }
        });
    };
    for (var _i = 0, arrows_1 = arrows; _i < arrows_1.length; _i++) {
        var i = arrows_1[_i];
        _loop_1(i);
    }
    var parentDiv = function (e) { return e instanceof HTMLDivElement ? e : parentDiv(e.parentElement); };
    var docs = document.querySelectorAll(".close-docs");
    var _loop_2 = function (doc) {
        doc.addEventListener("click", function (_) {
            var par = parentDiv(doc.parentElement);
            if (par.classList.contains("doc-hidden")) {
                par.classList.remove("doc-hidden");
                for (var _i = 0, _a = par.querySelectorAll(".method"); _i < _a.length; _i++) {
                    var div = _a[_i];
                    if (div.classList.contains("colapsed")) {
                        div.classList.remove("colapsed");
                    }
                }
                for (var _b = 0, _c = par.querySelectorAll(".ref-element"); _b < _c.length; _b++) {
                    var div = _c[_b];
                    if (div.classList.contains("colapsed")) {
                        div.classList.remove("colapsed");
                    }
                }
            }
            else {
                par.classList.add("doc-hidden");
                for (var _d = 0, _e = par.querySelectorAll(".method"); _d < _e.length; _d++) {
                    var div = _e[_d];
                    if (!div.classList.contains("colapsed")) {
                        div.classList.add("colapsed");
                    }
                }
                for (var _f = 0, _g = par.querySelectorAll(".ref-element"); _f < _g.length; _f++) {
                    var div = _g[_f];
                    if (!div.classList.contains("colapsed")) {
                        div.classList.add("colapsed");
                    }
                }
            }
        });
    };
    for (var _a = 0, docs_1 = docs; _a < docs_1.length; _a++) {
        var doc = docs_1[_a];
        _loop_2(doc);
    }
    var stypes = document.getElementById("s-types");
    var sfuncs = document.getElementById("s-funcs");
    var applyFilter = function (e, filter) {
        if (e !== undefined) {
            var ule = e.querySelector("ul");
            for (var _i = 0, _a = ule.childNodes; _i < _a.length; _i++) {
                var child = _a[_i];
                if (child instanceof HTMLLIElement) {
                    if (child.textContent.toLowerCase().includes(filter.toLowerCase())) {
                        child.style.display = "list-item";
                    }
                    else {
                        child.style.display = "none";
                    }
                }
            }
        }
    };
    var sidebarSearchBar = document.getElementById("sidebar-search");
    sidebarSearchBar.addEventListener("keydown", function (e) {
        var value = sidebarSearchBar.value;
        if (value.startsWith("type:") || value.startsWith("t:")) {
            if (stypes !== undefined) {
                stypes.style.display = "block";
            }
            if (sfuncs !== undefined) {
                sfuncs.style.display = "none";
            }
        }
        else if (value.startsWith("f:") || value.startsWith("fn:") || value.startsWith("func:") || value.startsWith("function:")) {
            if (stypes !== undefined) {
                stypes.style.display = "none";
            }
            if (sfuncs !== undefined) {
                sfuncs.style.display = "block";
            }
        }
        else {
            if (stypes !== undefined) {
                stypes.style.display = "block";
            }
            if (sfuncs !== undefined) {
                sfuncs.style.display = "block";
            }
        }
        var i = value.split(":");
        var after = i.length === 1 ? i[0] : i[1];
        applyFilter(stypes, after);
        applyFilter(sfuncs, after);
    });
    var ref_elements = document.querySelectorAll(".ref-element");
    var methods = document.querySelectorAll(".method");
    var refcont = document.getElementById("ref-container");
    var methcont = document.getElementById("method-container");
    var staticmethcont = document.getElementById("static-method-container");
    var updateVis = function (e, b) {
        for (var _i = 0, _a = e.querySelectorAll(b); _i < _a.length; _i++) {
            var i = _a[_i];
            if (!i.classList.contains("search-hidden")) {
                if (e.classList.contains("empty-hidden")) {
                    e.classList.remove("empty-hidden");
                }
                return;
            }
        }
        if (!e.classList.contains("empty-hidden")) {
            e.classList.add("empty-hidden");
        }
    };
    var filterElements = function (elements, filter) {
        if (elements !== undefined) {
            for (var _i = 0, elements_1 = elements; _i < elements_1.length; _i++) {
                var el = elements_1[_i];
                var doc = el.querySelector(".doc");
                var definition = el.querySelector(".definition");
                if ((doc !== null && doc.innerText.includes(filter)) || (definition !== null && definition.innerText.includes(filter))) {
                    if (el.classList.contains("search-hidden")) {
                        el.classList.remove("search-hidden");
                    }
                }
                else {
                    if (!el.classList.contains("search-hidden")) {
                        el.classList.add("search-hidden");
                    }
                }
            }
        }
    };
    var mainSearchBar = document.getElementById("main-search");
    mainSearchBar.addEventListener("keyup", function (e) {
        var value = mainSearchBar.value;
        filterElements(ref_elements, value);
        filterElements(methods, value);
        if (refcont !== undefined && refcont !== null) {
            updateVis(refcont, ".ref-element");
        }
        if (methcont !== undefined && methcont !== null) {
            updateVis(methcont, ".method");
        }
        if (staticmethcont !== undefined && staticmethcont !== null) {
            updateVis(staticmethcont, ".method");
        }
    });
})();
