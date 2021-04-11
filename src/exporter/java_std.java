package com.ccgauche;

import java.util.Scanner;

public class Main {

    public static void main(String[] args) {
        //%%CODE%%
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
