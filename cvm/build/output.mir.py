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
while True:
  v[23]=v[21][v[14][0]:v[14][0]+v[20][0]]
  v[24]=v[23]+v[17]
  v[25]=v[7]+v[24]
  v[26]=v[21][v[20][0]:v[20][0]+v[20][0]]
  v[27]=v[26]+v[25]
  v[28]=v[7]+v[27]
  v[29]=v[21][v[1][0]:v[1][0]+v[20][0]]
  v[30]=v[29]+v[28]
  p(v[30])
  p(v[18])
  v[31]=v[21][v[0][0]:v[0][0]+v[20][0]]
  v[32]=v[31]+v[17]
  v[33]=v[7]+v[32]
  v[34]=v[21][v[6][0]:v[6][0]+v[20][0]]
  v[35]=v[34]+v[33]
  v[36]=v[7]+v[35]
  v[37]=v[21][v[4][0]:v[4][0]+v[20][0]]
  v[38]=v[37]+v[36]
  p(v[38])
  p(v[18])
  v[39]=v[21][v[15][0]:v[15][0]+v[20][0]]
  v[40]=v[39]+v[17]
  v[41]=v[7]+v[40]
  v[42]=v[21][v[13][0]:v[13][0]+v[20][0]]
  v[43]=v[42]+v[41]
  v[44]=v[7]+v[43]
  v[45]=v[21][v[5][0]:v[5][0]+v[20][0]]
  v[46]=v[45]+v[44]
  p(v[46])
  p(v[8])
  v[47]=list(map(lambda a:ord(a),iter(input())))
  p(v[17])
  v[48]=[len(v[47])]
  if v[48]!=v[20]:
    p(v[11])
    continue
  v[49]=[0]
  while True:
    if v[2]==v[49]:
      v[50]=[0]
      break
    v[51]=v[10][v[49][0]:v[49][0]+v[20][0]]
    v[49]=list(map(lambda i:(v[49][i]+v[20][i%len(v[20])])%256,iter(range(len(v[49])))))
    if v[51]==v[47]:
      v[50]=[1]
      break
  if v[50]==v[1]:
    p(v[11])
    continue
  v[52]=list(map(lambda i:(v[47][i]-v[19][i%len(v[19])])%256,iter(range(len(v[47])))))
  def f0():
    v[54]=v[21][v[52][0]:v[52][0]+v[20][0]]
    if v[54]!=v[16]:
      return v[20]
    v[55]=[len(v[22])]
    v[56]=list(map(lambda i:(v[52][i]+v[55][i%len(v[55])])%256,iter(range(len(v[52])))))
    v[57]=[len(v[21])]
    v[58]=v[21][v[56][0]:v[56][0]+v[57][0]]
    v[59]=v[22]+v[58]
    v[60]=list(map(lambda i:(v[52][i]-v[1][i%len(v[1])])%256,iter(range(len(v[52])))))
    v[61]=v[21][v[1][0]:v[1][0]+v[60][0]]
    v[62]=v[61]+v[59]
    v[21]=v[62]
    v[63]=[0]
    while True:
      if v[63]==v[4]:
        v[64]=v[62][v[15][0]:v[15][0]+v[20][0]]
        v[65]=v[62][v[6][0]:v[6][0]+v[20][0]]
        v[66]=list(map(lambda i:(v[65][i]+v[64][i%len(v[64])])%256,iter(range(len(v[65])))))
        v[67]=v[62][v[1][0]:v[1][0]+v[20][0]]
        v[68]=list(map(lambda i:(v[67][i]+v[66][i%len(v[66])])%256,iter(range(len(v[67])))))
        if v[68]==v[15]:
          v[69]=[88,32,104,97,115,32,119,111,110,33,10]
          p(v[69])
          exit()
        if v[68]==v[3]:
          v[69]=[79,32,104,97,115,32,119,111,110,33,10]
          p(v[69])
          exit()
        v[70]=v[62][v[5][0]:v[5][0]+v[20][0]]
        v[71]=v[62][v[6][0]:v[6][0]+v[20][0]]
        v[72]=list(map(lambda i:(v[71][i]+v[70][i%len(v[70])])%256,iter(range(len(v[71])))))
        v[73]=v[62][v[14][0]:v[14][0]+v[20][0]]
        v[74]=list(map(lambda i:(v[73][i]+v[72][i%len(v[72])])%256,iter(range(len(v[73])))))
        if v[74]==v[15]:
          v[69]=[88,32,104,97,115,32,119,111,110,33,10]
          p(v[69])
          exit()
        if v[74]==v[3]:
          v[69]=[79,32,104,97,115,32,119,111,110,33,10]
          p(v[69])
          exit()
        v[75]=[len(v[62])]
        v[76]=[0]
        while True:
          if v[75]==v[76]:
            v[77]=[78,111,98,111,100,121,32,119,111,110,33,10]
            p(v[77])
            exit()
          v[78]=v[62][v[76][0]:v[76][0]+v[20][0]]
          v[76]=list(map(lambda i:(v[76][i]+v[20][i%len(v[20])])%256,iter(range(len(v[76])))))
          if v[78]==v[16]:
            return v[1]
      v[79]=v[63]
      v[63]=list(map(lambda i:(v[63][i]+v[20][i%len(v[20])])%256,iter(range(len(v[63])))))
      v[80]=v[62][v[79][0]:v[79][0]+v[20][0]]
      if v[80]==v[16]:
        continue
      v[81]=list(map(lambda i:(v[79][i]+v[5][i%len(v[5])])%256,iter(range(len(v[79])))))
      v[82]=v[62][v[81][0]:v[81][0]+v[20][0]]
      v[83]=list(map(lambda i:(v[79][i]+v[4][i%len(v[4])])%256,iter(range(len(v[79])))))
      v[84]=v[62][v[83][0]:v[83][0]+v[20][0]]
      v[85]=list(map(lambda i:(v[84][i]+v[82][i%len(v[82])])%256,iter(range(len(v[84])))))
      v[86]=v[62][v[79][0]:v[79][0]+v[20][0]]
      v[87]=list(map(lambda i:(v[86][i]+v[85][i%len(v[85])])%256,iter(range(len(v[86])))))
      if v[87]==v[15]:
        v[69]=[88,32,104,97,115,32,119,111,110,33,10]
        p(v[69])
        exit()
      if v[87]==v[3]:
        v[69]=[79,32,104,97,115,32,119,111,110,33,10]
        p(v[69])
        exit()
      v[88]=list(map(lambda i:(v[79][i]*v[4][i%len(v[4])])%256,iter(range(len(v[79])))))
      v[89]=list(map(lambda i:(v[88][i]+v[14][i%len(v[14])])%256,iter(range(len(v[88])))))
      v[90]=v[62][v[89][0]:v[89][0]+v[20][0]]
      v[91]=list(map(lambda i:(v[88][i]+v[20][i%len(v[20])])%256,iter(range(len(v[88])))))
      v[92]=v[62][v[91][0]:v[91][0]+v[20][0]]
      v[93]=list(map(lambda i:(v[92][i]+v[90][i%len(v[90])])%256,iter(range(len(v[92])))))
      v[94]=v[62][v[88][0]:v[88][0]+v[20][0]]
      v[95]=list(map(lambda i:(v[94][i]+v[93][i%len(v[93])])%256,iter(range(len(v[94])))))
      if v[95]==v[15]:
        v[69]=[88,32,104,97,115,32,119,111,110,33,10]
        p(v[69])
        exit()
      if v[95]==v[3]:
        v[69]=[79,32,104,97,115,32,119,111,110,33,10]
        p(v[69])
        exit()
  v[53]=f0()
  if v[53]==v[20]:
    p(v[9])
    continue
  if v[22]==v[12]:
    v[22]=[88]
  else:
    v[22]=[79]