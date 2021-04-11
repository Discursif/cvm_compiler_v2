(function () {
    var arrows = document.querySelectorAll(".colapsable");
    var _loop_1 = function (i) {
        var image = i.querySelector("img");
        image.addEventListener("click", function () {
            if (image.classList.contains("open")) {
                image.classList.remove("open");
                for (var _i = 0, _a = i.childNodes; _i < _a.length; _i++) {
                    var childs = _a[_i];
                    if (childs instanceof HTMLDivElement && !childs.classList.contains("definition")) {
                        childs.style.display = "none";
                    }
                }
            }
            else {
                image.classList.add("open");
                var count = true;
                for (var _b = 0, _c = i.childNodes; _b < _c.length; _b++) {
                    var childs = _c[_b];
                    if (count) {
                        count = false;
                        continue;
                    }
                    if (childs instanceof HTMLDivElement && !childs.classList.contains("definition")) {
                        childs.style.display = "block";
                    }
                }
            }
        });
    };
    for (var _i = 0, arrows_1 = arrows; _i < arrows_1.length; _i++) {
        var i = arrows_1[_i];
        _loop_1(i);
    }
    var docs = document.querySelectorAll(".close-docs");
    var _loop_2 = function (doc) {
        doc.addEventListener("click", function (_) {
            doc.parentElement.parentElement.childNodes.forEach(function (e) {
                if (e instanceof HTMLDivElement) {
                    e.querySelectorAll(".colapsable img").forEach(function (r) {
                        if (doc.textContent.includes("-")) {
                            r.classList.remove("open");
                        }
                        else {
                            r.classList.add("open");
                        }
                    });
                    e.querySelectorAll(".colapsable > div").forEach(function (r) {
                        if (!r.classList.contains("definition")) {
                            if (doc.textContent.includes("-")) {
                                r.style.display = "none";
                            }
                            else {
                                r.style.display = "block";
                            }
                        }
                    });
                }
            });
            if (doc.textContent.includes("-")) {
                doc.textContent = "[ + ]";
            }
            else {
                doc.textContent = "[ - ]";
            }
        });
    };
    for (var _a = 0, docs_1 = docs; _a < docs_1.length; _a++) {
        var doc = docs_1[_a];
        _loop_2(doc);
    }
})();
