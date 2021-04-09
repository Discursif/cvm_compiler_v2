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
v[0]=[0]
v[1]=[48]
v[2]=[116,117,114,116,108,101,46,102,111,114,119,97,114,100,40]
v[3]=[1]
v[4]=[116,117,114,116,108,101,46,114,105,103,104,116,40]
v[5]=[41]
v[6]=[10]
v[7]=[1,116,117,114,116,108,101,46,101,120,105,116,111,110,99,108,105,99,107,40,41]
v[8]=[100]
v[9]=[]
while True:
  v[10]=list(map(lambda i:(v[8][i]%v[6][i%len(v[6])])%256,iter(range(len(v[8])))))
  v[11]=list(map(lambda i:(v[10][i]+v[1][i%len(v[1])])%256,iter(range(len(v[10])))))
  v[9]=v[11]+v[9]
  v[12]=list(map(lambda i:floor(v[8][i]/v[6][i%len(v[6])])%256,iter(range(len(v[8])))))
  v[8]=v[12]
  if v[12]==v[0]:
    break
v[13]=v[9]+v[5]
v[14]=v[2]+v[13]
v[15]=v[3]+v[14]
p(v[15])
v[16]=[90]
v[17]=[]
while True:
  v[18]=list(map(lambda i:(v[16][i]%v[6][i%len(v[6])])%256,iter(range(len(v[16])))))
  v[19]=list(map(lambda i:(v[18][i]+v[1][i%len(v[1])])%256,iter(range(len(v[18])))))
  v[17]=v[19]+v[17]
  v[20]=list(map(lambda i:floor(v[16][i]/v[6][i%len(v[6])])%256,iter(range(len(v[16])))))
  v[16]=v[20]
  if v[20]==v[0]:
    break
v[21]=v[17]+v[5]
v[22]=v[4]+v[21]
v[23]=v[3]+v[22]
p(v[23])
v[24]=[50]
v[25]=[]
while True:
  v[26]=list(map(lambda i:(v[24][i]%v[6][i%len(v[6])])%256,iter(range(len(v[24])))))
  v[27]=list(map(lambda i:(v[26][i]+v[1][i%len(v[1])])%256,iter(range(len(v[26])))))
  v[25]=v[27]+v[25]
  v[28]=list(map(lambda i:floor(v[24][i]/v[6][i%len(v[6])])%256,iter(range(len(v[24])))))
  v[24]=v[28]
  if v[28]==v[0]:
    break
v[29]=v[25]+v[5]
v[30]=v[2]+v[29]
v[31]=v[3]+v[30]
p(v[31])
v[32]=[90]
v[33]=[]
while True:
  v[34]=list(map(lambda i:(v[32][i]%v[6][i%len(v[6])])%256,iter(range(len(v[32])))))
  v[35]=list(map(lambda i:(v[34][i]+v[1][i%len(v[1])])%256,iter(range(len(v[34])))))
  v[33]=v[35]+v[33]
  v[36]=list(map(lambda i:floor(v[32][i]/v[6][i%len(v[6])])%256,iter(range(len(v[32])))))
  v[32]=v[36]
  if v[36]==v[0]:
    break
v[37]=v[33]+v[5]
v[38]=v[4]+v[37]
v[39]=v[3]+v[38]
p(v[39])
v[40]=[100]
v[41]=[]
while True:
  v[42]=list(map(lambda i:(v[40][i]%v[6][i%len(v[6])])%256,iter(range(len(v[40])))))
  v[43]=list(map(lambda i:(v[42][i]+v[1][i%len(v[1])])%256,iter(range(len(v[42])))))
  v[41]=v[43]+v[41]
  v[44]=list(map(lambda i:floor(v[40][i]/v[6][i%len(v[6])])%256,iter(range(len(v[40])))))
  v[40]=v[44]
  if v[44]==v[0]:
    break
v[45]=v[41]+v[5]
v[46]=v[2]+v[45]
v[47]=v[3]+v[46]
p(v[47])
v[48]=[90]
v[49]=[]
while True:
  v[50]=list(map(lambda i:(v[48][i]%v[6][i%len(v[6])])%256,iter(range(len(v[48])))))
  v[51]=list(map(lambda i:(v[50][i]+v[1][i%len(v[1])])%256,iter(range(len(v[50])))))
  v[49]=v[51]+v[49]
  v[52]=list(map(lambda i:floor(v[48][i]/v[6][i%len(v[6])])%256,iter(range(len(v[48])))))
  v[48]=v[52]
  if v[52]==v[0]:
    break
v[53]=v[49]+v[5]
v[54]=v[4]+v[53]
v[55]=v[3]+v[54]
p(v[55])
v[56]=[50]
v[57]=[]
while True:
  v[58]=list(map(lambda i:(v[56][i]%v[6][i%len(v[6])])%256,iter(range(len(v[56])))))
  v[59]=list(map(lambda i:(v[58][i]+v[1][i%len(v[1])])%256,iter(range(len(v[58])))))
  v[57]=v[59]+v[57]
  v[60]=list(map(lambda i:floor(v[56][i]/v[6][i%len(v[6])])%256,iter(range(len(v[56])))))
  v[56]=v[60]
  if v[60]==v[0]:
    break
v[61]=v[57]+v[5]
v[62]=v[2]+v[61]
v[63]=v[3]+v[62]
p(v[63])
v[64]=[90]
v[65]=[]
while True:
  v[66]=list(map(lambda i:(v[64][i]%v[6][i%len(v[6])])%256,iter(range(len(v[64])))))
  v[67]=list(map(lambda i:(v[66][i]+v[1][i%len(v[1])])%256,iter(range(len(v[66])))))
  v[65]=v[67]+v[65]
  v[68]=list(map(lambda i:floor(v[64][i]/v[6][i%len(v[6])])%256,iter(range(len(v[64])))))
  v[64]=v[68]
  if v[68]==v[0]:
    break
v[69]=v[65]+v[5]
v[70]=v[4]+v[69]
v[71]=v[3]+v[70]
p(v[71])
p(v[7])