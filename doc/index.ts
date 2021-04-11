
(function () {
    const arrows = document.querySelectorAll(".colapsable");

    for (const i of arrows) {
        const image = i.querySelector("img");
        image.addEventListener("click", () => {
            if (image.classList.contains("open")) {
                image.classList.remove("open");
                for (const childs of i.childNodes) {
                    if (childs instanceof HTMLDivElement && !childs.classList.contains("definition")) {
                        childs.style.display = "none";
                    }
                }
            } else {
                image.classList.add("open");
                let count = true;
                for (const childs of i.childNodes) {
                    if (count) {
                        count = false;
                        continue
                    }
                    if (childs instanceof HTMLDivElement && !childs.classList.contains("definition")) {
                        childs.style.display = "block";
                    }
                }
            }
        });
    }
    const docs = document.querySelectorAll(".close-docs");
    for (const doc of docs) {
        doc.addEventListener("click", (_) => {
            doc.parentElement.parentElement.childNodes.forEach((e) => {
                if (e instanceof HTMLDivElement) {
                    e.querySelectorAll(".colapsable img").forEach((r) => {
                        if (doc.textContent.includes("-")) {
                            r.classList.remove("open");
                        } else {
                            r.classList.add("open");
                        }
                    })
                    e.querySelectorAll(".colapsable > div").forEach((r) => {
                        if (!r.classList.contains("definition")) {
                            if (doc.textContent.includes("-")) {
                                r.style.display = "none";
                            } else {
                                r.style.display = "block";
                            }
                        }
                    })
                }
            })
            if (doc.textContent.includes("-")) {
                doc.textContent = "[ + ]";
            } else {
                doc.textContent = "[ - ]";
            }
        })
    }
})();