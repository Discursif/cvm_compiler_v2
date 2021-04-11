(function() {
    const htmlDiv = document.createElement("div");
    htmlDiv.innerHTML = `
    <div style="position: absolute;height: 100vh; top: 0; left: 0;width: 100vw; display: flex;flex-direction: column">
        <pre id="textPane" style="flex-grow: 1"></pre>
        <div style="display:flex;flex-direction:row">
            <input type="text" id="inputField" style="flex-grow:1">
            <button type="text" id="inputButton">OK</button>
        </div>
    </div>
    `;
    document.body.appendChild(htmlDiv);

    const inputButton = document.getElementById("inputButton");
    const inputField = document.getElementById("inputField");
    const textPane = document.getElementById("textPane");

    const arraysMatch = (a, b) => {
        if (a.length !== b.length) return false;
        for (var i = 0; i < a.length; i++) {
            if (a[i] !== b[i]) return false;
        }
        return true;
    
    };

    const print = (e) => {
        textPane.innerHTML += String.fromCharCode.apply(String, e);
    }

    function string2Bin(str) {
        var result = [];
        for (var i = 0; i < str.length; i++) {
            result.push(str.charCodeAt(i));
        }
        return result;
    }

    function o(reg1,reg2,lmb) {
        const reg3 = [];
        for (const y in reg1) {
            reg3.push(lmb(reg1[y], reg2[y % reg2.length]) % 256);
        }
        return reg3
    }

    let currentResolver = undefined;

    inputButton.addEventListener("click", (e) => {
        if (currentResolver !== undefined) {
            currentResolver(string2Bin(inputField.value));
            currentResolver = undefined;
            inputField.value = "";
        }
    });
    inputField.addEventListener("keydown", (event) => {
        if (event.keyCode === 13) {
            inputButton.click();
        }
    });

    async function input() {
        return await new Promise((resolve, _) => currentResolver = resolve);
    }

    (async function() {
        const v = {};
        //%%CODE%%
    })();
})();