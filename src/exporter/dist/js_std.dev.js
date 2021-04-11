"use strict";

(function () {
  var htmlDiv = document.createElement("div");
  htmlDiv.innerHTML = "\n    <div style=\"position: absolute;height: 100vh; top: 0; left: 0;width: 100vw; display: flex;flex-direction: column\">\n        <pre id=\"textPane\" style=\"flex-grow: 1\"></pre>\n        <div style=\"display:flex;flex-direction:row\">\n            <input type=\"text\" id=\"inputField\" style=\"flex-grow:1\">\n            <button type=\"text\" id=\"inputButton\">OK</button>\n        </div>\n    </div>\n    ";
  document.body.appendChild(htmlDiv);
  var inputButton = document.getElementById("inputButton");
  var inputField = document.getElementById("inputField");
  var textPane = document.getElementById("textPane");

  var arraysMatch = function arraysMatch(a, b) {
    if (a.length !== b.length) return false;

    for (var i = 0; i < a.length; i++) {
      if (a[i] !== b[i]) return false;
    }

    return true;
  };

  var print = function print(e) {
    textPane.innerHTML += String.fromCharCode.apply(String, e);
  };

  function string2Bin(str) {
    var result = [];

    for (var i = 0; i < str.length; i++) {
      result.push(str.charCodeAt(i));
    }

    return result;
  }

  function o(reg1, reg2, lmb) {
    var reg3 = [];

    for (var y in reg1) {
      reg3.push(lmb(reg1[y], reg2[y % reg2.length]) % 256);
    }

    return reg3;
  }

  var currentResolver = undefined;
  inputButton.addEventListener("click", function (e) {
    if (currentResolver !== undefined) {
      currentResolver(string2Bin(inputField.value));
      currentResolver = undefined;
      inputField.value = "";
    }
  });
  inputField.addEventListener("keydown", function (event) {
    if (event.keyCode === 13) {
      inputButton.click();
    }
  });

  function input() {
    return regeneratorRuntime.async(function input$(_context) {
      while (1) {
        switch (_context.prev = _context.next) {
          case 0:
            _context.next = 2;
            return regeneratorRuntime.awrap(new Promise(function (resolve, _) {
              return currentResolver = resolve;
            }));

          case 2:
            return _context.abrupt("return", _context.sent);

          case 3:
          case "end":
            return _context.stop();
        }
      }
    });
  }

  (function _callee() {
    var v;
    return regeneratorRuntime.async(function _callee$(_context2) {
      while (1) {
        switch (_context2.prev = _context2.next) {
          case 0:
            v = {}; //%%CODE%%

          case 1:
          case "end":
            return _context2.stop();
        }
      }
    });
  })();
})();