package com.ccgauche;

import java.util.Scanner;

public class Main {

    public static void main(String[] args) {
                final Reg v26 = new Reg();
        final Reg v29 = new Reg();
        final Reg v5 = new Reg();
        final Reg v16 = new Reg();
        final Reg v17 = new Reg();
        final Reg v30 = new Reg();
        final Reg v11 = new Reg();
        final Reg v6 = new Reg();
        final Reg v31 = new Reg();
        final Reg v3 = new Reg();
        final Reg v19 = new Reg();
        final Reg v25 = new Reg();
        final Reg v21 = new Reg();
        final Reg v8 = new Reg();
        final Reg v4 = new Reg();
        final Reg v15 = new Reg();
        final Reg v7 = new Reg();
        final Reg v10 = new Reg();
        final Reg v22 = new Reg();
        final Reg v14 = new Reg();
        final Reg v27 = new Reg();
        final Reg v20 = new Reg();
        final Reg v2 = new Reg();
        final Reg v9 = new Reg();
        final Reg v18 = new Reg();
        final Reg v23 = new Reg();
        final Reg v1 = new Reg();
        final Reg v24 = new Reg();
        final Reg v28 = new Reg();
        final Reg v32 = new Reg();
        final Reg v12 = new Reg();
        final Reg v13 = new Reg();
        final Reg v0 = new Reg();

        v0.cst(125);
        v1.cst(102, 97, 108, 115, 101);
        v2.cst(41);
        v3.cst(91);
        v4.cst(40);
        v5.cst(80, 97, 114, 101, 110, 116, 104, 101, 115, 105, 115, 32, 111, 107, 58, 32);
        v6.cst(10);
        v7.cst(116, 114, 117, 101);
        v8.cst(0);
        v9.cst(69, 110, 116, 101, 114, 32, 121, 111, 117, 114, 32, 40, 41, 91, 93, 123, 125, 32, 99, 111, 110, 116, 97, 105, 110, 105, 110, 103, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 58, 10);
        v10.cst(123);
        v11.cst(1);
        v12.cst(93);
        v9.print();
        v13.input();
        v14.cst(0);
        v15.cst(0);
        v16.cst(0);
        v17.len(v13);
        while (true) {
            if (v17.equals(v8)) {
                if (v15.equals(v8)) {
                    v18.cst(1);
                } else {
                    v18.cst(0);
                }
                if (v16.equals(v8)) {
                    v19.cst(1);
                } else {
                    v19.cst(0);
                }
                if (v14.equals(v8)) {
                    v20.cst(1);
                } else {
                    v20.cst(0);
                }
                v21.op(v20, v19, (a, b) -> a * b);
                v22.op(v21, v18, (a, b) -> a * b);
                v23.mov(((Func) () -> {
                    if (v22.equals(v11)) {
                        return v7;
                    }
                    return v1;
                }).run());
                v24.merge(v23, v6);
                v25.merge(v5, v24);
                v25.print();
                System.exit(0);
            }
            v26.op(v17, v11, (a, b) -> a - b);
            v17.mov(v26);
            v27.read(v13, v26, v11);
            if (v27.equals(v4)) {
                v14.op(v14, v11, (a, b) -> a + b);
                continue;
            }
            v28.read(v13, v26, v11);
            if (v28.equals(v2)) {
                v14.op(v14, v11, (a, b) -> a - b);
                continue;
            }
            v29.read(v13, v26, v11);
            if (v29.equals(v10)) {
                v16.op(v16, v11, (a, b) -> a + b);
                continue;
            }
            v30.read(v13, v26, v11);
            if (v30.equals(v3)) {
                v15.op(v15, v11, (a, b) -> a + b);
                continue;
            }
            v31.read(v13, v26, v11);
            if (v31.equals(v12)) {
                v15.op(v15, v11, (a, b) -> a + b);
                continue;
            }
            v32.read(v13, v26, v11);
            if (v32.equals(v0)) {
                v16.op(v16, v11, (a, b) -> a - b);
                continue;
            }
        }

    }

    interface Operation {
        int run(int a, int b);
    }

    interface Func {
        Reg run();
    }

    static class Reg {
        private final int[] register = new int[255];
        private int len = 0;

        public Reg() {
        }

        public void op(Reg a, Reg b, Operation op) {
            this.len = a.len;
            for (int index = 0; index < a.len; index++)
                this.register[index] = op.run(a.register[index], b.register[index % b.len]) % 256;
        }

        public void merge(Reg a, Reg b) {
            this.len = a.len + b.len;
            if (a.len >= 0) System.arraycopy(a.register, 0, this.register, 0, a.len);
            if (b.len >= 0) System.arraycopy(b.register, 0, this.register, a.len, b.len);
        }

        public void mov(Reg a) {
            this.len = a.len;
            System.arraycopy(a.register, 0, this.register, 0, a.len);
        }

        public void cst(int... bytes) {
            this.len = bytes.length;
            System.arraycopy(bytes, 0, this.register, 0, bytes.length);
        }

        public void read(Reg a, Reg b, Reg c) {
            this.len = Math.min(a.len - b.register[0], c.register[0]);
            System.arraycopy(a.register, b.register[0], this.register, 0, this.len);
        }

        public boolean equals(Reg a) {
            if (this.len != a.len) return false;
            for (int index = 0; index < this.len; index++) if (this.register[index] != a.register[index]) return false;
            return true;
        }
        
        public void len(Reg a) {
            this.len = 1;
            this.register[0] = a.len;
        }

        public void input() {
            byte[] input = new Scanner(System.in).nextLine().getBytes();
            for (int index = 0; index < input.length; index++) this.register[index] = Byte.toUnsignedInt(input[index]);
            this.len = input.length;
        }

        public void print() {
            StringBuilder builder = new StringBuilder(this.len);
            for (int index = 0; index < this.len; index++) builder.append((char) this.register[index]);
            System.out.print(builder);
            System.out.flush();
        }
    }


}
