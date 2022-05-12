###
### Compiled for CVM bytecode
### CVM is an open-source low level assembly with targets such as JIT, Native, C, Python, JS, Rust
### This python code as been auto-generated
###
### CVM 2020 - 2022- All rights reserved 
### CVM is a Laurent Gaucheron software
###
### CVM 2.0.3.1745
###

from math import * 
import turtle

def p(a):
    if a[0] == 1:
        eval("".join(list(map(lambda a:chr(a),a[1:]))))
    else:
        print("".join(list(map(lambda a:chr(a),a))), end='', flush=True)

v = {}
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
p(v[9])
v[13]=list(map(lambda a:ord(a),iter(input())))
v[14]=[0]
v[15]=[0]
v[16]=[0]
v[17]=[len(v[13])]
while True:
  if v[17]==v[8]:
    if v[15]==v[8]:
      v[18]=[1]
    else:
      v[18]=[0]
    if v[16]==v[8]:
      v[19]=[1]
    else:
      v[19]=[0]
    if v[14]==v[8]:
      v[20]=[1]
    else:
      v[20]=[0]
    v[21]=list(map(lambda i:(v[20][i]*v[19][i%len(v[19])])%256,iter(range(len(v[20])))))
    v[22]=list(map(lambda i:(v[21][i]*v[18][i%len(v[18])])%256,iter(range(len(v[21])))))
    def f0():
      if v[22]==v[11]:
        return v[7]
      return v[1]
    v[23]=f0()
    v[24]=v[23]+v[6]
    v[25]=v[5]+v[24]
    p(v[25])
    exit()
  v[26]=list(map(lambda i:(v[17][i]-v[11][i%len(v[11])])%256,iter(range(len(v[17])))))
  v[17]=v[26]
  v[27]=v[13][v[26][0]:v[26][0]+v[11][0]]
  if v[27]==v[4]:
    v[14]=list(map(lambda i:(v[14][i]+v[11][i%len(v[11])])%256,iter(range(len(v[14])))))
    continue
  v[28]=v[13][v[26][0]:v[26][0]+v[11][0]]
  if v[28]==v[2]:
    v[14]=list(map(lambda i:(v[14][i]-v[11][i%len(v[11])])%256,iter(range(len(v[14])))))
    continue
  v[29]=v[13][v[26][0]:v[26][0]+v[11][0]]
  if v[29]==v[10]:
    v[16]=list(map(lambda i:(v[16][i]+v[11][i%len(v[11])])%256,iter(range(len(v[16])))))
    continue
  v[30]=v[13][v[26][0]:v[26][0]+v[11][0]]
  if v[30]==v[3]:
    v[15]=list(map(lambda i:(v[15][i]+v[11][i%len(v[11])])%256,iter(range(len(v[15])))))
    continue
  v[31]=v[13][v[26][0]:v[26][0]+v[11][0]]
  if v[31]==v[12]:
    v[15]=list(map(lambda i:(v[15][i]+v[11][i%len(v[11])])%256,iter(range(len(v[15])))))
    continue
  v[32]=v[13][v[26][0]:v[26][0]+v[11][0]]
  if v[32]==v[0]:
    v[16]=list(map(lambda i:(v[16][i]-v[11][i%len(v[11])])%256,iter(range(len(v[16])))))
    continue