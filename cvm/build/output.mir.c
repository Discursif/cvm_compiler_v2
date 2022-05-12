#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MIN(a, b) ((a) < (b) ? (a) : (b))

void main()
{
    unsigned char v9[257];

unsigned char v7[257];

unsigned char v0[257];

unsigned char v25[257];

unsigned char v1[257];

unsigned char v17[257];

unsigned char v32[257];

unsigned char v5[257];

unsigned char v23[257];

unsigned char v28[257];

unsigned char v30[257];

unsigned char v6[257];

unsigned char v22[257];

unsigned char v8[257];

unsigned char v19[257];

unsigned char v31[257];

unsigned char v21[257];

unsigned char v16[257];

unsigned char v3[257];

unsigned char v11[257];

unsigned char v20[257];

unsigned char v24[257];

unsigned char v18[257];

unsigned char v10[257];

unsigned char v27[257];

unsigned char v13[257];

unsigned char v4[257];

unsigned char v12[257];

unsigned char v29[257];

unsigned char v15[257];

unsigned char v26[257];

unsigned char v2[257];

unsigned char v14[257];

    memcpy(v0, (char[2]){1, 125}, 2);
memcpy(v1, (char[6]){5, 102, 97, 108, 115, 101}, 6);
memcpy(v2, (char[2]){1, 41}, 2);
memcpy(v3, (char[2]){1, 91}, 2);
memcpy(v4, (char[2]){1, 40}, 2);
memcpy(v5, (char[17]){16, 80, 97, 114, 101, 110, 116, 104, 101, 115, 105, 115, 32, 111, 107, 58, 32}, 17);
memcpy(v6, (char[2]){1, 10}, 2);
memcpy(v7, (char[5]){4, 116, 114, 117, 101}, 5);
memcpy(v8, (char[2]){1, 0}, 2);
memcpy(v9, (char[42]){41, 69, 110, 116, 101, 114, 32, 121, 111, 117, 114, 32, 40, 41, 91, 93, 123, 125, 32, 99, 111, 110, 116, 97, 105, 110, 105, 110, 103, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 58, 10}, 42);
memcpy(v10, (char[2]){1, 123}, 2);
memcpy(v11, (char[2]){1, 1}, 2);
memcpy(v12, (char[2]){1, 93}, 2);
v9[v9[0]+1] = 0; printf("%s", v9+1);
fgets(v13+1, 255, stdin); v13[0] = strlen(v13+1)-1;
memcpy(v14, (char[2]){1, 0}, 2);
memcpy(v15, (char[2]){1, 0}, 2);
memcpy(v16, (char[2]){1, 0}, 2);
v17[0] = 1; v17[1] = v13[0];
while(1) {
if (v17[0] == v8[0] && memcmp(v17+1, v8+1, v17[0]) == 0) {
if (v15[0] == v8[0] && memcmp(v15+1, v8+1, v15[0]) == 0) {
memcpy(v18, (char[2]){1, 1}, 2);
} else { 
memcpy(v18, (char[2]){1, 0}, 2); 
}
if (v16[0] == v8[0] && memcmp(v16+1, v8+1, v16[0]) == 0) {
memcpy(v19, (char[2]){1, 1}, 2);
} else { 
memcpy(v19, (char[2]){1, 0}, 2); 
}
if (v14[0] == v8[0] && memcmp(v14+1, v8+1, v14[0]) == 0) {
memcpy(v20, (char[2]){1, 1}, 2);
} else { 
memcpy(v20, (char[2]){1, 0}, 2); 
}
v21[0] = v20[0]; for (unsigned char index = 0; index < v20[0]; index++) {
                            v21[index+1] = v20[index+1] * v19[(index % v19[0])+1];
                        }
v22[0] = v21[0]; for (unsigned char index = 0; index < v21[0]; index++) {
                            v22[index+1] = v21[index+1] * v18[(index % v18[0])+1];
                        }

                if (v22[0] == v11[0] && memcmp(v22+1, v11+1, v22[0]) == 0) {
memcpy(v23, v7, v7[0]+1); goto fn1;
} else { 
 
}
memcpy(v23, v1, v1[0]+1); goto fn1;
                fn1:
                
if (v23[0] + v6[0] > 255) abort();
                                    v24[0] = v23[0] + v6[0];
                                    memcpy(v24+1, v23+1, v23[0]);
                                    memcpy(v24+v23[0]+1, v6+1, v6[0]);
if (v5[0] + v24[0] > 255) abort();
                                    v25[0] = v5[0] + v24[0];
                                    memcpy(v25+1, v5+1, v5[0]);
                                    memcpy(v25+v5[0]+1, v24+1, v24[0]);
v25[v25[0]+1] = 0; printf("%s", v25+1);
exit(0);
} else { 
 
}
v26[0] = v17[0]; for (unsigned char index = 0; index < v17[0]; index++) {
                            v26[index+1] = v17[index+1] - v11[(index % v11[0])+1];
                        }
memcpy(v17, v26, v26[0]+1);
if (v26[0] != 1 || v11[0] != 1 || v26[1] >= v13[0]) abort();
                v27[0] = MIN(v13[0] - v26[1], v11[1]);
                memcpy(v27+1, v13+v26[1]+1, v27[0]);
if (v27[0] == v4[0] && memcmp(v27+1, v4+1, v27[0]) == 0) {
v14[0] = v14[0]; for (unsigned char index = 0; index < v14[0]; index++) {
                            v14[index+1] = v14[index+1] + v11[(index % v11[0])+1];
                        }
continue;
} else { 
 
}
if (v26[0] != 1 || v11[0] != 1 || v26[1] >= v13[0]) abort();
                v28[0] = MIN(v13[0] - v26[1], v11[1]);
                memcpy(v28+1, v13+v26[1]+1, v28[0]);
if (v28[0] == v2[0] && memcmp(v28+1, v2+1, v28[0]) == 0) {
v14[0] = v14[0]; for (unsigned char index = 0; index < v14[0]; index++) {
                            v14[index+1] = v14[index+1] - v11[(index % v11[0])+1];
                        }
continue;
} else { 
 
}
if (v26[0] != 1 || v11[0] != 1 || v26[1] >= v13[0]) abort();
                v29[0] = MIN(v13[0] - v26[1], v11[1]);
                memcpy(v29+1, v13+v26[1]+1, v29[0]);
if (v29[0] == v10[0] && memcmp(v29+1, v10+1, v29[0]) == 0) {
v16[0] = v16[0]; for (unsigned char index = 0; index < v16[0]; index++) {
                            v16[index+1] = v16[index+1] + v11[(index % v11[0])+1];
                        }
continue;
} else { 
 
}
if (v26[0] != 1 || v11[0] != 1 || v26[1] >= v13[0]) abort();
                v30[0] = MIN(v13[0] - v26[1], v11[1]);
                memcpy(v30+1, v13+v26[1]+1, v30[0]);
if (v30[0] == v3[0] && memcmp(v30+1, v3+1, v30[0]) == 0) {
v15[0] = v15[0]; for (unsigned char index = 0; index < v15[0]; index++) {
                            v15[index+1] = v15[index+1] + v11[(index % v11[0])+1];
                        }
continue;
} else { 
 
}
if (v26[0] != 1 || v11[0] != 1 || v26[1] >= v13[0]) abort();
                v31[0] = MIN(v13[0] - v26[1], v11[1]);
                memcpy(v31+1, v13+v26[1]+1, v31[0]);
if (v31[0] == v12[0] && memcmp(v31+1, v12+1, v31[0]) == 0) {
v15[0] = v15[0]; for (unsigned char index = 0; index < v15[0]; index++) {
                            v15[index+1] = v15[index+1] + v11[(index % v11[0])+1];
                        }
continue;
} else { 
 
}
if (v26[0] != 1 || v11[0] != 1 || v26[1] >= v13[0]) abort();
                v32[0] = MIN(v13[0] - v26[1], v11[1]);
                memcpy(v32+1, v13+v26[1]+1, v32[0]);
if (v32[0] == v0[0] && memcmp(v32+1, v0+1, v32[0]) == 0) {
v16[0] = v16[0]; for (unsigned char index = 0; index < v16[0]; index++) {
                            v16[index+1] = v16[index+1] - v11[(index % v11[0])+1];
                        }
continue;
} else { 
 
}
}
}