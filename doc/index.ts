
(function () {
    const arrows = document.querySelectorAll(".colapsable");

    for (const i of arrows) {
        const image = i.querySelector("img");
        image.addEventListener("click", () => {
            const classList = i.classList;
            if (classList.contains("colapsed")) {
                classList.remove("colapsed");
            } else {
                classList.add("colapsed");
            }
        });
    }

    const parentDiv = (e: HTMLElement): HTMLDivElement => e instanceof HTMLDivElement ? e : parentDiv(e.parentElement);
    const docs = document.querySelectorAll(".close-docs");
    for (const doc of docs) {
        doc.addEventListener("click", (_) => {
            const par = parentDiv(doc.parentElement);
            if (par.classList.contains("doc-hidden")) {
                par.classList.remove("doc-hidden");
                for (const div of par.querySelectorAll(".method")) {
                    if (div.classList.contains("colapsed")) {
                        div.classList.remove("colapsed");
                    }
                }
                for (const div of par.querySelectorAll(".ref-element")) {
                    if (div.classList.contains("colapsed")) {
                        div.classList.remove("colapsed");
                    }
                }
            } else {
                par.classList.add("doc-hidden");
                for (const div of par.querySelectorAll(".method")) {
                    if (!div.classList.contains("colapsed")) {
                        div.classList.add("colapsed");
                    }
                }
                for (const div of par.querySelectorAll(".ref-element")) {
                    if (!div.classList.contains("colapsed")) {
                        div.classList.add("colapsed");
                    }
                }
            }
        })
    }
    const stypes: HTMLDivElement = document.getElementById("s-types") as HTMLDivElement;
    const sfuncs: HTMLDivElement = document.getElementById("s-funcs") as HTMLDivElement;
    const applyFilter = (e: HTMLDivElement, filter: string) => {
        if (e !== undefined) {
            const ule: HTMLUListElement = e.querySelector("ul") as HTMLUListElement;
            for (const child of ule.childNodes) {
                if (child instanceof HTMLLIElement) {
                    if (child.textContent.toLowerCase().includes(filter.toLowerCase())) {
                        child.style.display = "list-item";
                    } else {
                        child.style.display = "none";
                    }
                }
            }
        }
    };
    const sidebarSearchBar: HTMLInputElement = document.getElementById("sidebar-search") as HTMLInputElement;
    sidebarSearchBar.addEventListener("keydown", (e) => {
        const value = sidebarSearchBar.value;
        if (value.startsWith("type:") || value.startsWith("t:")) {
            if (stypes !== undefined) {
                stypes.style.display = "block";
            }
            if (sfuncs !== undefined) {
                sfuncs.style.display = "none";
            }
        } else if (value.startsWith("f:") || value.startsWith("fn:") || value.startsWith("func:") || value.startsWith("function:")) {
            if (stypes !== undefined) {
                stypes.style.display = "none";
            }
            if (sfuncs !== undefined) {
                sfuncs.style.display = "block";
            }
        } else {
            if (stypes !== undefined) {
                stypes.style.display = "block";
            }
            if (sfuncs !== undefined) {
                sfuncs.style.display = "block";
            }
        }
        const i = value.split(":");
        const after = i.length === 1 ? i[0] : i[1];
        applyFilter(stypes, after);
        applyFilter(sfuncs, after);
    });


    const ref_elements: NodeListOf<HTMLDivElement> = document.querySelectorAll(".ref-element");
    const methods: NodeListOf<HTMLDivElement> = document.querySelectorAll(".method");

    const refcont: HTMLDivElement = document.getElementById("ref-container") as HTMLDivElement;
    const methcont: HTMLDivElement = document.getElementById("method-container") as HTMLDivElement;
    const staticmethcont: HTMLDivElement = document.getElementById("static-method-container") as HTMLDivElement;

    const updateVis = (e: HTMLDivElement, b: string) => {
        for (const i of e.querySelectorAll(b)) {
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
    }

    const filterElements = (elements: NodeListOf<HTMLDivElement>, filter: string) => {
        if (elements !== undefined) {
            for (const el of elements) {
                const doc = el.querySelector(".doc");
                const definition = el.querySelector(".definition");
                if ((doc !== null && doc.innerText.includes(filter)) || (definition !== null && definition.innerText.includes(filter))) {
                    if (el.classList.contains("search-hidden")) {
                        el.classList.remove("search-hidden");
                    }
                } else {
                    if (!el.classList.contains("search-hidden")) {
                        el.classList.add("search-hidden");
                    }
                }
            }
        }
    };
    const mainSearchBar: HTMLInputElement = document.getElementById("main-search") as HTMLInputElement;
    mainSearchBar.addEventListener("keyup", (e) => {
        const value = mainSearchBar.value;
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