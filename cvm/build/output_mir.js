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
        v[0]=[2]
v[1]=[237]
v[2]=[8]
v[3]=[49]
v[4]=[0]
v[5]=[5]
v[6]=[45,45,45,45,45,45,45,45,45,10]
v[7]=[6]
v[8]=[88]
v[9]=[32]
v[10]=[32,124,32]
v[11]=[73,110,118,97,108,105,100,32,105,110,112,117,116,10]
v[12]=[65,108,100,114,101,97,100,121,32,115,111,109,101,116,104,105,110,103,32,112,108,97,99,101,100,32,104,101,114,101,10]
v[13]=[7]
v[14]=[4]
v[15]=[69,110,116,101,114,32,97,32,112,111,115,105,116,105,111,110,32,116,111,32,112,108,97,121,32,105,110,32,58,32]
v[16]=[79]
v[17]=[3]
v[18]=[10]
v[19]=[1]
v[20]=[32,32,32,32,32,32,32,32,32]
v[21]=[88]
while (true) {
  v[22]=v[20].slice(v[0][0],v[0][0]+v[19][0]);
  v[23]=v[22].concat(v[18]);
  v[24]=v[10].concat(v[23]);
  v[25]=v[20].slice(v[19][0],v[19][0]+v[19][0]);
  v[26]=v[25].concat(v[24]);
  v[27]=v[10].concat(v[26]);
  v[28]=v[20].slice(v[4][0],v[4][0]+v[19][0]);
  v[29]=v[28].concat(v[27]);
  print(v[29])
  print(v[6])
  v[30]=v[20].slice(v[5][0],v[5][0]+v[19][0]);
  v[31]=v[30].concat(v[18]);
  v[32]=v[10].concat(v[31]);
  v[33]=v[20].slice(v[14][0],v[14][0]+v[19][0]);
  v[34]=v[33].concat(v[32]);
  v[35]=v[10].concat(v[34]);
  v[36]=v[20].slice(v[17][0],v[17][0]+v[19][0]);
  v[37]=v[36].concat(v[35]);
  print(v[37])
  print(v[6])
  v[38]=v[20].slice(v[2][0],v[2][0]+v[19][0]);
  v[39]=v[38].concat(v[18]);
  v[40]=v[10].concat(v[39]);
  v[41]=v[20].slice(v[13][0],v[13][0]+v[19][0]);
  v[42]=v[41].concat(v[40]);
  v[43]=v[10].concat(v[42]);
  v[44]=v[20].slice(v[7][0],v[7][0]+v[19][0]);
  v[45]=v[44].concat(v[43]);
  print(v[45])
  print(v[15])
  v[46]=await input();
  print(v[18])
  v[47]=[v[46].length];
  if (!arraysMatch(v[47],v[19])) {
    print(v[11])
    continue;
  }
  v[48]=(function() {
    v[49]=[49]
    if (arraysMatch(v[49],v[46])) {
      return v[19]
    }
    v[49]=[50]
    if (arraysMatch(v[49],v[46])) {
      return v[19]
    }
    v[49]=[51]
    if (arraysMatch(v[49],v[46])) {
      return v[19]
    }
    v[49]=[52]
    if (arraysMatch(v[49],v[46])) {
      return v[19]
    }
    v[49]=[53]
    if (arraysMatch(v[49],v[46])) {
      return v[19]
    }
    v[49]=[54]
    if (arraysMatch(v[49],v[46])) {
      return v[19]
    }
    v[49]=[55]
    if (arraysMatch(v[49],v[46])) {
      return v[19]
    }
    v[49]=[56]
    if (arraysMatch(v[49],v[46])) {
      return v[19]
    }
    v[49]=[57]
    if (arraysMatch(v[49],v[46])) {
      return v[19]
    }
    return v[4]
  })();
  if (arraysMatch(v[48],v[4])) {
    print(v[11])
    continue;
  }
  v[50]=o(v[46],v[3],(a,b) => a-b);
  v[51]=(function() {
    v[52]=v[20].slice(v[50][0],v[50][0]+v[19][0]);
    if (!arraysMatch(v[52],v[9])) {
      return v[19]
    }
    v[53]=[v[21].length];
    v[54]=o(v[50],v[53],(a,b) => a+b);
    v[55]=[v[20].length];
    v[56]=v[20].slice(v[54][0],v[54][0]+v[55][0]);
    v[57]=v[21].concat(v[56]);
    v[58]=o(v[50],v[4],(a,b) => a-b);
    v[59]=v[20].slice(v[4][0],v[4][0]+v[58][0]);
    v[60]=v[59].concat(v[57]);
    v[20]=v[60];
    v[61]=(function() {
      v[62]=[0]
      while (true) {
        if (arraysMatch(v[62],v[17])) {
          break;
        }
        v[63]=v[62];
        v[62]=o(v[62],v[19],(a,b) => a+b);
        v[64]=v[60].slice(v[63][0],v[63][0]+v[19][0]);
        if (arraysMatch(v[64],v[9])) {
          continue;
        }
        v[65]=o(v[63],v[7],(a,b) => a+b);
        v[66]=v[60].slice(v[65][0],v[65][0]+v[19][0]);
        v[67]=o(v[63],v[17],(a,b) => a+b);
        v[68]=v[60].slice(v[67][0],v[67][0]+v[19][0]);
        v[69]=o(v[68],v[66],(a,b) => a+b);
        v[70]=v[60].slice(v[63][0],v[63][0]+v[19][0]);
        v[71]=o(v[70],v[69],(a,b) => a+b);
        if (arraysMatch(v[71],v[2])) {
          return v[8]
        }
        if (arraysMatch(v[71],v[1])) {
          return v[16]
        }
        v[72]=o(v[63],v[17],(a,b) => a*b);
        v[73]=o(v[72],v[0],(a,b) => a+b);
        v[74]=v[60].slice(v[73][0],v[73][0]+v[19][0]);
        v[75]=o(v[72],v[19],(a,b) => a+b);
        v[76]=v[60].slice(v[75][0],v[75][0]+v[19][0]);
        v[77]=o(v[76],v[74],(a,b) => a+b);
        v[78]=v[60].slice(v[72][0],v[72][0]+v[19][0]);
        v[79]=o(v[78],v[77],(a,b) => a+b);
        if (arraysMatch(v[79],v[2])) {
          return v[8]
        }
        if (arraysMatch(v[79],v[1])) {
          return v[16]
        }
      }
      v[80]=v[60].slice(v[2][0],v[2][0]+v[19][0]);
      v[81]=v[60].slice(v[14][0],v[14][0]+v[19][0]);
      v[82]=o(v[81],v[80],(a,b) => a+b);
      v[83]=v[60].slice(v[4][0],v[4][0]+v[19][0]);
      v[84]=o(v[83],v[82],(a,b) => a+b);
      if (arraysMatch(v[84],v[2])) {
        return v[8]
      }
      if (arraysMatch(v[84],v[1])) {
        return v[16]
      }
      v[85]=v[60].slice(v[7][0],v[7][0]+v[19][0]);
      v[86]=v[60].slice(v[14][0],v[14][0]+v[19][0]);
      v[87]=o(v[86],v[85],(a,b) => a+b);
      v[88]=v[60].slice(v[0][0],v[0][0]+v[19][0]);
      v[89]=o(v[88],v[87],(a,b) => a+b);
      if (arraysMatch(v[89],v[2])) {
        return v[8]
      }
      if (arraysMatch(v[89],v[1])) {
        return v[16]
      }
      return v[9]
    })();
    if (!arraysMatch(v[61],v[9])) {
      v[90]=[32,104,97,115,32,119,111,110,33,10]
      v[91]=v[61].concat(v[90]);
      print(v[91])
      fail;
    }
    v[92]=(function() {
      v[93]=[v[60].length];
      v[94]=[0]
      while (true) {
        if (arraysMatch(v[93],v[94])) {
          break;
        }
        v[95]=v[60].slice(v[94][0],v[94][0]+v[19][0]);
        v[94]=o(v[94],v[19],(a,b) => a+b);
        if (arraysMatch(v[95],v[9])) {
          return v[19]
        }
      }
      return v[4]
    })();
    if (arraysMatch(v[92],v[4])) {
      v[96]=[78,111,98,111,100,121,32,119,111,110,33,10]
      print(v[96])
      fail;
    }
    return v[4]
  })();
  if (arraysMatch(v[51],v[19])) {
    print(v[12])
    continue;
  }
  if (arraysMatch(v[21],v[16])) {
    v[21]=[88]
  } else {
    v[21]=[79]
  }
}
fail;
    })();
})();