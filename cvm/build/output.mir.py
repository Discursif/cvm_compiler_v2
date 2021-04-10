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
v[1]=[8]
v[2]=[45,45,45,45,45,45,45,45,45,10]
v[3]=[32]
v[4]=[1]
v[5]=[49]
v[6]=[10]
v[7]=[237]
v[8]=[6]
v[9]=[7]
v[10]=[79]
v[11]=[2]
v[12]=[3]
v[13]=[32,124,32]
v[14]=[4]
v[15]=[0]
v[16]=[69,110,116,101,114,32,97,32,112,111,115,105,116,105,111,110,32,116,111,32,112,108,97,121,32,105,110,32,58,32]
v[17]=[65,108,100,114,101,97,100,121,32,115,111,109,101,116,104,105,110,103,32,112,108,97,99,101,100,32,104,101,114,101,10]
v[18]=[88]
v[19]=[73,110,118,97,108,105,100,32,105,110,112,117,116,10]
v[20]=[32,32,32,32,32,32,32,32,32]
v[21]=[88]
while True:
  v[22]=v[20][v[11][0]:v[11][0]+v[4][0]]
  v[23]=v[22]+v[6]
  v[24]=v[13]+v[23]
  v[25]=v[20][v[4][0]:v[4][0]+v[4][0]]
  v[26]=v[25]+v[24]
  v[27]=v[13]+v[26]
  v[28]=v[20][v[15][0]:v[15][0]+v[4][0]]
  v[29]=v[28]+v[27]
  p(v[29])
  p(v[2])
  v[30]=v[20][v[0][0]:v[0][0]+v[4][0]]
  v[31]=v[30]+v[6]
  v[32]=v[13]+v[31]
  v[33]=v[20][v[14][0]:v[14][0]+v[4][0]]
  v[34]=v[33]+v[32]
  v[35]=v[13]+v[34]
  v[36]=v[20][v[12][0]:v[12][0]+v[4][0]]
  v[37]=v[36]+v[35]
  p(v[37])
  p(v[2])
  v[38]=v[20][v[1][0]:v[1][0]+v[4][0]]
  v[39]=v[38]+v[6]
  v[40]=v[13]+v[39]
  v[41]=v[20][v[9][0]:v[9][0]+v[4][0]]
  v[42]=v[41]+v[40]
  v[43]=v[13]+v[42]
  v[44]=v[20][v[8][0]:v[8][0]+v[4][0]]
  v[45]=v[44]+v[43]
  p(v[45])
  p(v[16])
  v[46]=list(map(lambda a:ord(a),iter(input())))
  p(v[6])
  v[47]=[len(v[46])]
  if v[47]!=v[4]:
    p(v[19])
    continue
  def f0():
    v[49]=[49]
    if v[49]==v[46]:
      return v[4]
    v[49]=[50]
    if v[49]==v[46]:
      return v[4]
    v[49]=[51]
    if v[49]==v[46]:
      return v[4]
    v[49]=[52]
    if v[49]==v[46]:
      return v[4]
    v[49]=[53]
    if v[49]==v[46]:
      return v[4]
    v[49]=[54]
    if v[49]==v[46]:
      return v[4]
    v[49]=[55]
    if v[49]==v[46]:
      return v[4]
    v[49]=[56]
    if v[49]==v[46]:
      return v[4]
    v[49]=[57]
    if v[49]==v[46]:
      return v[4]
    return v[15]
  v[48]=f0()
  if v[48]==v[15]:
    p(v[19])
    continue
  v[50]=list(map(lambda i:(v[46][i]-v[5][i%len(v[5])])%256,iter(range(len(v[46])))))
  def f1():
    v[52]=v[20][v[50][0]:v[50][0]+v[4][0]]
    if v[52]!=v[3]:
      return v[4]
    v[53]=[len(v[21])]
    v[54]=list(map(lambda i:(v[50][i]+v[53][i%len(v[53])])%256,iter(range(len(v[50])))))
    v[55]=[len(v[20])]
    v[56]=v[20][v[54][0]:v[54][0]+v[55][0]]
    v[57]=v[21]+v[56]
    v[58]=list(map(lambda i:(v[50][i]-v[15][i%len(v[15])])%256,iter(range(len(v[50])))))
    v[59]=v[20][v[15][0]:v[15][0]+v[58][0]]
    v[60]=v[59]+v[57]
    v[20]=v[60]
    def f2():
      v[62]=[0]
      while True:
        if v[62]==v[12]:
          break
        v[63]=v[62]
        v[62]=list(map(lambda i:(v[62][i]+v[4][i%len(v[4])])%256,iter(range(len(v[62])))))
        v[64]=v[60][v[63][0]:v[63][0]+v[4][0]]
        if v[64]==v[3]:
          continue
        v[65]=list(map(lambda i:(v[63][i]+v[8][i%len(v[8])])%256,iter(range(len(v[63])))))
        v[66]=v[60][v[65][0]:v[65][0]+v[4][0]]
        v[67]=list(map(lambda i:(v[63][i]+v[12][i%len(v[12])])%256,iter(range(len(v[63])))))
        v[68]=v[60][v[67][0]:v[67][0]+v[4][0]]
        v[69]=list(map(lambda i:(v[68][i]+v[66][i%len(v[66])])%256,iter(range(len(v[68])))))
        v[70]=v[60][v[63][0]:v[63][0]+v[4][0]]
        v[71]=list(map(lambda i:(v[70][i]+v[69][i%len(v[69])])%256,iter(range(len(v[70])))))
        if v[71]==v[1]:
          return v[18]
        if v[71]==v[7]:
          return v[10]
        v[72]=list(map(lambda i:(v[63][i]*v[12][i%len(v[12])])%256,iter(range(len(v[63])))))
        v[73]=list(map(lambda i:(v[72][i]+v[11][i%len(v[11])])%256,iter(range(len(v[72])))))
        v[74]=v[60][v[73][0]:v[73][0]+v[4][0]]
        v[75]=list(map(lambda i:(v[72][i]+v[4][i%len(v[4])])%256,iter(range(len(v[72])))))
        v[76]=v[60][v[75][0]:v[75][0]+v[4][0]]
        v[77]=list(map(lambda i:(v[76][i]+v[74][i%len(v[74])])%256,iter(range(len(v[76])))))
        v[78]=v[60][v[72][0]:v[72][0]+v[4][0]]
        v[79]=list(map(lambda i:(v[78][i]+v[77][i%len(v[77])])%256,iter(range(len(v[78])))))
        if v[79]==v[1]:
          return v[18]
        if v[79]==v[7]:
          return v[10]
      v[80]=v[60][v[1][0]:v[1][0]+v[4][0]]
      v[81]=v[60][v[14][0]:v[14][0]+v[4][0]]
      v[82]=list(map(lambda i:(v[81][i]+v[80][i%len(v[80])])%256,iter(range(len(v[81])))))
      v[83]=v[60][v[15][0]:v[15][0]+v[4][0]]
      v[84]=list(map(lambda i:(v[83][i]+v[82][i%len(v[82])])%256,iter(range(len(v[83])))))
      if v[84]==v[1]:
        return v[18]
      if v[84]==v[7]:
        return v[10]
      v[85]=v[60][v[8][0]:v[8][0]+v[4][0]]
      v[86]=v[60][v[14][0]:v[14][0]+v[4][0]]
      v[87]=list(map(lambda i:(v[86][i]+v[85][i%len(v[85])])%256,iter(range(len(v[86])))))
      v[88]=v[60][v[11][0]:v[11][0]+v[4][0]]
      v[89]=list(map(lambda i:(v[88][i]+v[87][i%len(v[87])])%256,iter(range(len(v[88])))))
      if v[89]==v[1]:
        return v[18]
      if v[89]==v[7]:
        return v[10]
      return v[3]
    v[61]=f2()
    if v[61]!=v[3]:
      v[90]=[32,104,97,115,32,119,111,110,33,10]
      v[91]=v[61]+v[90]
      p(v[91])
      exit()
    def f3():
      v[93]=[len(v[60])]
      v[94]=[0]
      while True:
        if v[93]==v[94]:
          break
        v[95]=v[60][v[94][0]:v[94][0]+v[4][0]]
        v[94]=list(map(lambda i:(v[94][i]+v[4][i%len(v[4])])%256,iter(range(len(v[94])))))
        if v[95]==v[3]:
          return v[4]
      return v[15]
    v[92]=f3()
    if v[92]==v[15]:
      v[96]=[78,111,98,111,100,121,32,119,111,110,33,10]
      p(v[96])
      exit()
    return v[15]
  v[51]=f1()
  if v[51]==v[4]:
    p(v[17])
    continue
  if v[21]==v[10]:
    v[21]=[88]
  else:
    v[21]=[79]
exit()