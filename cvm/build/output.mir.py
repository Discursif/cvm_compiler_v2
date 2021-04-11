###
### Compiled for CVM bytecode
### CVM is an open-source low level assembly with targets such as JIT, Native, C, Python, JS, Rust
### This python code as been auto-generated
###
### CVM 2020 - 2021 - All rights reserved 
### CVM is a Laurent Gaucheron software
###
### CVM 2.0.3.1741
###

from math import * 
import turtle

def p(a):
    if a[0] == 1:
        eval("".join(list(map(lambda a:chr(a),a[1:]))))
    else:
        print("".join(list(map(lambda a:chr(a),a))), end='', flush=True)

v = {}
v[0]=[1]
v[1]=[0]
if v[2]==v[1]:
  break
v[2]=list(map(lambda i:(v[2][i]-v[0][i%len(v[0])])%256,iter(range(len(v[2])))))
exit()