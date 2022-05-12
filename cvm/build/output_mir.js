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
        v[0]=[125]
v[1]=[102,97,108,115,101]
v[2]=[41]
v[3]=[91]
v[4]=[40]
v[5]=[80,97,114,101,110,116,104,101,115,105,115,32,111,107,58,32]
v[6]=[10]
v[7]=[116,114,117,101]
v[8]=[0]
v[9]=[69,110,116,101,114,32,121,111,117,114,32,40,41,91,93,123,125,32,99,111,110,116,97,105,110,105,110,103,32,101,120,112,114,101,115,115,105,111,110,58,10]
v[10]=[123]
v[11]=[1]
v[12]=[93]
print(v[9])
v[13]=await input();
v[14]=[0]
v[15]=[0]
v[16]=[0]
v[17]=[v[13].length];
while (true) {
  if (arraysMatch(v[17],v[8])) {
    if (arraysMatch(v[15],v[8])) {
      v[18]=[1]
    } else {
      v[18]=[0]
    }
    if (arraysMatch(v[16],v[8])) {
      v[19]=[1]
    } else {
      v[19]=[0]
    }
    if (arraysMatch(v[14],v[8])) {
      v[20]=[1]
    } else {
      v[20]=[0]
    }
    v[21]=o(v[20],v[19],(a,b) => a*b);
    v[22]=o(v[21],v[18],(a,b) => a*b);
    v[23]=(function() {
      if (arraysMatch(v[22],v[11])) {
        return v[7]
      }
      return v[1]
    })();
    v[24]=v[23].concat(v[6]);
    v[25]=v[5].concat(v[24]);
    print(v[25])
    fail;
  }
  v[26]=o(v[17],v[11],(a,b) => a-b);
  v[17]=v[26];
  v[27]=v[13].slice(v[26][0],v[26][0]+v[11][0]);
  if (arraysMatch(v[27],v[4])) {
    v[14]=o(v[14],v[11],(a,b) => a+b);
    continue;
  }
  v[28]=v[13].slice(v[26][0],v[26][0]+v[11][0]);
  if (arraysMatch(v[28],v[2])) {
    v[14]=o(v[14],v[11],(a,b) => a-b);
    continue;
  }
  v[29]=v[13].slice(v[26][0],v[26][0]+v[11][0]);
  if (arraysMatch(v[29],v[10])) {
    v[16]=o(v[16],v[11],(a,b) => a+b);
    continue;
  }
  v[30]=v[13].slice(v[26][0],v[26][0]+v[11][0]);
  if (arraysMatch(v[30],v[3])) {
    v[15]=o(v[15],v[11],(a,b) => a+b);
    continue;
  }
  v[31]=v[13].slice(v[26][0],v[26][0]+v[11][0]);
  if (arraysMatch(v[31],v[12])) {
    v[15]=o(v[15],v[11],(a,b) => a+b);
    continue;
  }
  v[32]=v[13].slice(v[26][0],v[26][0]+v[11][0]);
  if (arraysMatch(v[32],v[0])) {
    v[16]=o(v[16],v[11],(a,b) => a-b);
    continue;
  }
}
    })();
})();