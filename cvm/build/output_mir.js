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
        v[0]=[5]
v[1]=[0]
v[2]=[9]
v[3]=[237]
v[4]=[3]
v[5]=[6]
v[6]=[4]
v[7]=[32,124,32]
v[8]=[69,110,116,101,114,32,97,32,112,111,115,105,116,105,111,110,32,116,111,32,112,108,97,121,32,105,110,32,58,32]
v[9]=[65,108,100,114,101,97,100,121,32,115,111,109,101,116,104,105,110,103,32,112,108,97,99,101,100,32,104,101,114,101,10]
v[10]=[49,50,51,52,53,54,55,56,57]
v[11]=[73,110,118,97,108,105,100,32,105,110,112,117,116,10]
v[12]=[79]
v[13]=[7]
v[14]=[2]
v[15]=[8]
v[16]=[32]
v[17]=[10]
v[18]=[45,45,45,45,45,45,45,45,45,10]
v[19]=[49]
v[20]=[1]
v[21]=[32,32,32,32,32,32,32,32,32]
v[22]=[88]
while (true) {
  v[23]=v[21].slice(v[14][0],v[14][0]+v[20][0]);
  v[24]=v[23].concat(v[17]);
  v[25]=v[7].concat(v[24]);
  v[26]=v[21].slice(v[20][0],v[20][0]+v[20][0]);
  v[27]=v[26].concat(v[25]);
  v[28]=v[7].concat(v[27]);
  v[29]=v[21].slice(v[1][0],v[1][0]+v[20][0]);
  v[30]=v[29].concat(v[28]);
  print(v[30])
  print(v[18])
  v[31]=v[21].slice(v[0][0],v[0][0]+v[20][0]);
  v[32]=v[31].concat(v[17]);
  v[33]=v[7].concat(v[32]);
  v[34]=v[21].slice(v[6][0],v[6][0]+v[20][0]);
  v[35]=v[34].concat(v[33]);
  v[36]=v[7].concat(v[35]);
  v[37]=v[21].slice(v[4][0],v[4][0]+v[20][0]);
  v[38]=v[37].concat(v[36]);
  print(v[38])
  print(v[18])
  v[39]=v[21].slice(v[15][0],v[15][0]+v[20][0]);
  v[40]=v[39].concat(v[17]);
  v[41]=v[7].concat(v[40]);
  v[42]=v[21].slice(v[13][0],v[13][0]+v[20][0]);
  v[43]=v[42].concat(v[41]);
  v[44]=v[7].concat(v[43]);
  v[45]=v[21].slice(v[5][0],v[5][0]+v[20][0]);
  v[46]=v[45].concat(v[44]);
  print(v[46])
  print(v[8])
  v[47]=await input();
  print(v[17])
  v[48]=[v[47].length];
  if (!arraysMatch(v[48],v[20])) {
    print(v[11])
    continue;
  }
  v[49]=[0]
  while (true) {
    if (arraysMatch(v[2],v[49])) {
      v[50]=[0]
      break;
    }
    v[51]=v[10].slice(v[49][0],v[49][0]+v[20][0]);
    v[49]=o(v[49],v[20],(a,b) => a+b);
    if (arraysMatch(v[51],v[47])) {
      v[50]=[1]
      break;
    }
  }
  if (arraysMatch(v[50],v[1])) {
    print(v[11])
    continue;
  }
  v[52]=o(v[47],v[19],(a,b) => a-b);
  v[53]=(function() {
    v[54]=v[21].slice(v[52][0],v[52][0]+v[20][0]);
    if (!arraysMatch(v[54],v[16])) {
      return v[20]
    }
    v[55]=[v[22].length];
    v[56]=o(v[52],v[55],(a,b) => a+b);
    v[57]=[v[21].length];
    v[58]=v[21].slice(v[56][0],v[56][0]+v[57][0]);
    v[59]=v[22].concat(v[58]);
    v[60]=o(v[52],v[1],(a,b) => a-b);
    v[61]=v[21].slice(v[1][0],v[1][0]+v[60][0]);
    v[62]=v[61].concat(v[59]);
    v[21]=v[62];
    v[63]=[0]
    while (true) {
      if (arraysMatch(v[63],v[4])) {
        v[64]=v[62].slice(v[15][0],v[15][0]+v[20][0]);
        v[65]=v[62].slice(v[6][0],v[6][0]+v[20][0]);
        v[66]=o(v[65],v[64],(a,b) => a+b);
        v[67]=v[62].slice(v[1][0],v[1][0]+v[20][0]);
        v[68]=o(v[67],v[66],(a,b) => a+b);
        if (arraysMatch(v[68],v[15])) {
          v[69]=[88,32,104,97,115,32,119,111,110,33,10]
          print(v[69])
          fail;
        }
        if (arraysMatch(v[68],v[3])) {
          v[69]=[79,32,104,97,115,32,119,111,110,33,10]
          print(v[69])
          fail;
        }
        v[70]=v[62].slice(v[5][0],v[5][0]+v[20][0]);
        v[71]=v[62].slice(v[6][0],v[6][0]+v[20][0]);
        v[72]=o(v[71],v[70],(a,b) => a+b);
        v[73]=v[62].slice(v[14][0],v[14][0]+v[20][0]);
        v[74]=o(v[73],v[72],(a,b) => a+b);
        if (arraysMatch(v[74],v[15])) {
          v[69]=[88,32,104,97,115,32,119,111,110,33,10]
          print(v[69])
          fail;
        }
        if (arraysMatch(v[74],v[3])) {
          v[69]=[79,32,104,97,115,32,119,111,110,33,10]
          print(v[69])
          fail;
        }
        v[75]=[v[62].length];
        v[76]=[0]
        while (true) {
          if (arraysMatch(v[75],v[76])) {
            v[77]=[78,111,98,111,100,121,32,119,111,110,33,10]
            print(v[77])
            fail;
          }
          v[78]=v[62].slice(v[76][0],v[76][0]+v[20][0]);
          v[76]=o(v[76],v[20],(a,b) => a+b);
          if (arraysMatch(v[78],v[16])) {
            return v[1]
          }
        }
      }
      v[79]=v[63];
      v[63]=o(v[63],v[20],(a,b) => a+b);
      v[80]=v[62].slice(v[79][0],v[79][0]+v[20][0]);
      if (arraysMatch(v[80],v[16])) {
        continue;
      }
      v[81]=o(v[79],v[5],(a,b) => a+b);
      v[82]=v[62].slice(v[81][0],v[81][0]+v[20][0]);
      v[83]=o(v[79],v[4],(a,b) => a+b);
      v[84]=v[62].slice(v[83][0],v[83][0]+v[20][0]);
      v[85]=o(v[84],v[82],(a,b) => a+b);
      v[86]=v[62].slice(v[79][0],v[79][0]+v[20][0]);
      v[87]=o(v[86],v[85],(a,b) => a+b);
      if (arraysMatch(v[87],v[15])) {
        v[69]=[88,32,104,97,115,32,119,111,110,33,10]
        print(v[69])
        fail;
      }
      if (arraysMatch(v[87],v[3])) {
        v[69]=[79,32,104,97,115,32,119,111,110,33,10]
        print(v[69])
        fail;
      }
      v[88]=o(v[79],v[4],(a,b) => a*b);
      v[89]=o(v[88],v[14],(a,b) => a+b);
      v[90]=v[62].slice(v[89][0],v[89][0]+v[20][0]);
      v[91]=o(v[88],v[20],(a,b) => a+b);
      v[92]=v[62].slice(v[91][0],v[91][0]+v[20][0]);
      v[93]=o(v[92],v[90],(a,b) => a+b);
      v[94]=v[62].slice(v[88][0],v[88][0]+v[20][0]);
      v[95]=o(v[94],v[93],(a,b) => a+b);
      if (arraysMatch(v[95],v[15])) {
        v[69]=[88,32,104,97,115,32,119,111,110,33,10]
        print(v[69])
        fail;
      }
      if (arraysMatch(v[95],v[3])) {
        v[69]=[79,32,104,97,115,32,119,111,110,33,10]
        print(v[69])
        fail;
      }
    }
  })();
  if (arraysMatch(v[53],v[20])) {
    print(v[9])
    continue;
  }
  if (arraysMatch(v[22],v[12])) {
    v[22]=[88]
  } else {
    v[22]=[79]
  }
}
    })();
})();