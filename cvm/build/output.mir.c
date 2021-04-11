#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MIN(a, b) ((a) < (b) ? (a) : (b))

void main()
{
    unsigned char v88[257];

unsigned char v5[257];

unsigned char v15[257];

unsigned char v35[257];

unsigned char v38[257];

unsigned char v23[257];

unsigned char v45[257];

unsigned char v46[257];

unsigned char v48[257];

unsigned char v52[257];

unsigned char v57[257];

unsigned char v68[257];

unsigned char v79[257];

unsigned char v84[257];

unsigned char v42[257];

unsigned char v85[257];

unsigned char v18[257];

unsigned char v72[257];

unsigned char v8[257];

unsigned char v62[257];

unsigned char v81[257];

unsigned char v89[257];

unsigned char v37[257];

unsigned char v9[257];

unsigned char v69[257];

unsigned char v10[257];

unsigned char v96[257];

unsigned char v61[257];

unsigned char v24[257];

unsigned char v31[257];

unsigned char v47[257];

unsigned char v58[257];

unsigned char v4[257];

unsigned char v78[257];

unsigned char v71[257];

unsigned char v51[257];

unsigned char v19[257];

unsigned char v55[257];

unsigned char v32[257];

unsigned char v22[257];

unsigned char v14[257];

unsigned char v2[257];

unsigned char v13[257];

unsigned char v43[257];

unsigned char v59[257];

unsigned char v11[257];

unsigned char v54[257];

unsigned char v50[257];

unsigned char v77[257];

unsigned char v90[257];

unsigned char v66[257];

unsigned char v74[257];

unsigned char v80[257];

unsigned char v40[257];

unsigned char v70[257];

unsigned char v75[257];

unsigned char v20[257];

unsigned char v87[257];

unsigned char v65[257];

unsigned char v63[257];

unsigned char v41[257];

unsigned char v91[257];

unsigned char v56[257];

unsigned char v0[257];

unsigned char v7[257];

unsigned char v49[257];

unsigned char v6[257];

unsigned char v94[257];

unsigned char v92[257];

unsigned char v95[257];

unsigned char v1[257];

unsigned char v36[257];

unsigned char v27[257];

unsigned char v3[257];

unsigned char v86[257];

unsigned char v44[257];

unsigned char v60[257];

unsigned char v76[257];

unsigned char v30[257];

unsigned char v83[257];

unsigned char v93[257];

unsigned char v17[257];

unsigned char v53[257];

unsigned char v16[257];

unsigned char v26[257];

unsigned char v29[257];

unsigned char v33[257];

unsigned char v73[257];

unsigned char v25[257];

unsigned char v12[257];

unsigned char v64[257];

unsigned char v28[257];

unsigned char v82[257];

unsigned char v21[257];

unsigned char v34[257];

unsigned char v39[257];

unsigned char v67[257];

    memcpy(v0, (char[2]){1, 1}, 2);
memcpy(v1, (char[2]){1, 2}, 2);
memcpy(v2, (char[2]){1, 5}, 2);
memcpy(v3, (char[2]){1, 4}, 2);
memcpy(v4, (char[2]){1, 0}, 2);
memcpy(v5, (char[15]){14, 73, 110, 118, 97, 108, 105, 100, 32, 105, 110, 112, 117, 116, 10}, 15);
memcpy(v6, (char[2]){1, 8}, 2);
memcpy(v7, (char[2]){1, 6}, 2);
memcpy(v8, (char[2]){1, 10}, 2);
memcpy(v9, (char[2]){1, 32}, 2);
memcpy(v10, (char[31]){30, 69, 110, 116, 101, 114, 32, 97, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 116, 111, 32, 112, 108, 97, 121, 32, 105, 110, 32, 58, 32}, 31);
memcpy(v11, (char[2]){1, 49}, 2);
memcpy(v12, (char[2]){1, 7}, 2);
memcpy(v13, (char[2]){1, 88}, 2);
memcpy(v14, (char[2]){1, 3}, 2);
memcpy(v15, (char[2]){1, 237}, 2);
memcpy(v16, (char[32]){31, 65, 108, 100, 114, 101, 97, 100, 121, 32, 115, 111, 109, 101, 116, 104, 105, 110, 103, 32, 112, 108, 97, 99, 101, 100, 32, 104, 101, 114, 101, 10}, 32);
memcpy(v17, (char[2]){1, 79}, 2);
memcpy(v18, (char[11]){10, 45, 45, 45, 45, 45, 45, 45, 45, 45, 10}, 11);
memcpy(v19, (char[4]){3, 32, 124, 32}, 4);
memcpy(v20, (char[10]){9, 32, 32, 32, 32, 32, 32, 32, 32, 32}, 10);
memcpy(v21, (char[2]){1, 88}, 2);
while(1) {
if (v1[0] != 1 || v0[0] != 1 || v1[1] >= v20[0]) abort();
                v22[0] = MIN(v20[0] - v1[1], v0[1]);
                memcpy(v22+1, v20+v1[1]+1, v22[0]);
if (v22[0] + v8[0] > 255) abort();
                                    v23[0] = v22[0] + v8[0];
                                    memcpy(v23+1, v22+1, v22[0]);
                                    memcpy(v23+v22[0]+1, v8+1, v8[0]);
if (v19[0] + v23[0] > 255) abort();
                                    v24[0] = v19[0] + v23[0];
                                    memcpy(v24+1, v19+1, v19[0]);
                                    memcpy(v24+v19[0]+1, v23+1, v23[0]);
if (v0[0] != 1 || v0[0] != 1 || v0[1] >= v20[0]) abort();
                v25[0] = MIN(v20[0] - v0[1], v0[1]);
                memcpy(v25+1, v20+v0[1]+1, v25[0]);
if (v25[0] + v24[0] > 255) abort();
                                    v26[0] = v25[0] + v24[0];
                                    memcpy(v26+1, v25+1, v25[0]);
                                    memcpy(v26+v25[0]+1, v24+1, v24[0]);
if (v19[0] + v26[0] > 255) abort();
                                    v27[0] = v19[0] + v26[0];
                                    memcpy(v27+1, v19+1, v19[0]);
                                    memcpy(v27+v19[0]+1, v26+1, v26[0]);
if (v4[0] != 1 || v0[0] != 1 || v4[1] >= v20[0]) abort();
                v28[0] = MIN(v20[0] - v4[1], v0[1]);
                memcpy(v28+1, v20+v4[1]+1, v28[0]);
if (v28[0] + v27[0] > 255) abort();
                                    v29[0] = v28[0] + v27[0];
                                    memcpy(v29+1, v28+1, v28[0]);
                                    memcpy(v29+v28[0]+1, v27+1, v27[0]);
v29[v29[0]+1] = 0; printf("%s", v29+1);
v18[v18[0]+1] = 0; printf("%s", v18+1);
if (v2[0] != 1 || v0[0] != 1 || v2[1] >= v20[0]) abort();
                v30[0] = MIN(v20[0] - v2[1], v0[1]);
                memcpy(v30+1, v20+v2[1]+1, v30[0]);
if (v30[0] + v8[0] > 255) abort();
                                    v31[0] = v30[0] + v8[0];
                                    memcpy(v31+1, v30+1, v30[0]);
                                    memcpy(v31+v30[0]+1, v8+1, v8[0]);
if (v19[0] + v31[0] > 255) abort();
                                    v32[0] = v19[0] + v31[0];
                                    memcpy(v32+1, v19+1, v19[0]);
                                    memcpy(v32+v19[0]+1, v31+1, v31[0]);
if (v3[0] != 1 || v0[0] != 1 || v3[1] >= v20[0]) abort();
                v33[0] = MIN(v20[0] - v3[1], v0[1]);
                memcpy(v33+1, v20+v3[1]+1, v33[0]);
if (v33[0] + v32[0] > 255) abort();
                                    v34[0] = v33[0] + v32[0];
                                    memcpy(v34+1, v33+1, v33[0]);
                                    memcpy(v34+v33[0]+1, v32+1, v32[0]);
if (v19[0] + v34[0] > 255) abort();
                                    v35[0] = v19[0] + v34[0];
                                    memcpy(v35+1, v19+1, v19[0]);
                                    memcpy(v35+v19[0]+1, v34+1, v34[0]);
if (v14[0] != 1 || v0[0] != 1 || v14[1] >= v20[0]) abort();
                v36[0] = MIN(v20[0] - v14[1], v0[1]);
                memcpy(v36+1, v20+v14[1]+1, v36[0]);
if (v36[0] + v35[0] > 255) abort();
                                    v37[0] = v36[0] + v35[0];
                                    memcpy(v37+1, v36+1, v36[0]);
                                    memcpy(v37+v36[0]+1, v35+1, v35[0]);
v37[v37[0]+1] = 0; printf("%s", v37+1);
v18[v18[0]+1] = 0; printf("%s", v18+1);
if (v6[0] != 1 || v0[0] != 1 || v6[1] >= v20[0]) abort();
                v38[0] = MIN(v20[0] - v6[1], v0[1]);
                memcpy(v38+1, v20+v6[1]+1, v38[0]);
if (v38[0] + v8[0] > 255) abort();
                                    v39[0] = v38[0] + v8[0];
                                    memcpy(v39+1, v38+1, v38[0]);
                                    memcpy(v39+v38[0]+1, v8+1, v8[0]);
if (v19[0] + v39[0] > 255) abort();
                                    v40[0] = v19[0] + v39[0];
                                    memcpy(v40+1, v19+1, v19[0]);
                                    memcpy(v40+v19[0]+1, v39+1, v39[0]);
if (v12[0] != 1 || v0[0] != 1 || v12[1] >= v20[0]) abort();
                v41[0] = MIN(v20[0] - v12[1], v0[1]);
                memcpy(v41+1, v20+v12[1]+1, v41[0]);
if (v41[0] + v40[0] > 255) abort();
                                    v42[0] = v41[0] + v40[0];
                                    memcpy(v42+1, v41+1, v41[0]);
                                    memcpy(v42+v41[0]+1, v40+1, v40[0]);
if (v19[0] + v42[0] > 255) abort();
                                    v43[0] = v19[0] + v42[0];
                                    memcpy(v43+1, v19+1, v19[0]);
                                    memcpy(v43+v19[0]+1, v42+1, v42[0]);
if (v7[0] != 1 || v0[0] != 1 || v7[1] >= v20[0]) abort();
                v44[0] = MIN(v20[0] - v7[1], v0[1]);
                memcpy(v44+1, v20+v7[1]+1, v44[0]);
if (v44[0] + v43[0] > 255) abort();
                                    v45[0] = v44[0] + v43[0];
                                    memcpy(v45+1, v44+1, v44[0]);
                                    memcpy(v45+v44[0]+1, v43+1, v43[0]);
v45[v45[0]+1] = 0; printf("%s", v45+1);
v10[v10[0]+1] = 0; printf("%s", v10+1);
fgets(v46+1, 255, stdin); v46[0] = strlen(v46+1)-1;
v8[v8[0]+1] = 0; printf("%s", v8+1);
v47[0] = 1; v47[1] = v46[0];
if (v47[0] == v0[0] && memcmp(v47+1, v0+1, v47[0]) == 0) {

} else { 
v5[v5[0]+1] = 0; printf("%s", v5+1);
continue; 
}

                memcpy(v49, (char[2]){1, 49}, 2);
if (v49[0] == v46[0] && memcmp(v49+1, v46+1, v49[0]) == 0) {
memcpy(v48, v0, v0[0]+1); goto fn1;
} else { 
 
}
memcpy(v49, (char[2]){1, 50}, 2);
if (v49[0] == v46[0] && memcmp(v49+1, v46+1, v49[0]) == 0) {
memcpy(v48, v0, v0[0]+1); goto fn1;
} else { 
 
}
memcpy(v49, (char[2]){1, 51}, 2);
if (v49[0] == v46[0] && memcmp(v49+1, v46+1, v49[0]) == 0) {
memcpy(v48, v0, v0[0]+1); goto fn1;
} else { 
 
}
memcpy(v49, (char[2]){1, 52}, 2);
if (v49[0] == v46[0] && memcmp(v49+1, v46+1, v49[0]) == 0) {
memcpy(v48, v0, v0[0]+1); goto fn1;
} else { 
 
}
memcpy(v49, (char[2]){1, 53}, 2);
if (v49[0] == v46[0] && memcmp(v49+1, v46+1, v49[0]) == 0) {
memcpy(v48, v0, v0[0]+1); goto fn1;
} else { 
 
}
memcpy(v49, (char[2]){1, 54}, 2);
if (v49[0] == v46[0] && memcmp(v49+1, v46+1, v49[0]) == 0) {
memcpy(v48, v0, v0[0]+1); goto fn1;
} else { 
 
}
memcpy(v49, (char[2]){1, 55}, 2);
if (v49[0] == v46[0] && memcmp(v49+1, v46+1, v49[0]) == 0) {
memcpy(v48, v0, v0[0]+1); goto fn1;
} else { 
 
}
memcpy(v49, (char[2]){1, 56}, 2);
if (v49[0] == v46[0] && memcmp(v49+1, v46+1, v49[0]) == 0) {
memcpy(v48, v0, v0[0]+1); goto fn1;
} else { 
 
}
memcpy(v49, (char[2]){1, 57}, 2);
if (v49[0] == v46[0] && memcmp(v49+1, v46+1, v49[0]) == 0) {
memcpy(v48, v0, v0[0]+1); goto fn1;
} else { 
 
}
memcpy(v48, v4, v4[0]+1); goto fn1;
                fn1:
                
if (v48[0] == v4[0] && memcmp(v48+1, v4+1, v48[0]) == 0) {
v5[v5[0]+1] = 0; printf("%s", v5+1);
continue;
} else { 
 
}
v50[0] = v46[0]; for (unsigned char index = 0; index < v46[0]; index++) {
                            v50[index+1] = v46[index+1] - v11[(index % v11[0])+1];
                        }

                if (v50[0] != 1 || v0[0] != 1 || v50[1] >= v20[0]) abort();
                v52[0] = MIN(v20[0] - v50[1], v0[1]);
                memcpy(v52+1, v20+v50[1]+1, v52[0]);
if (v52[0] == v9[0] && memcmp(v52+1, v9+1, v52[0]) == 0) {

} else { 
memcpy(v51, v0, v0[0]+1); goto fn2; 
}
v53[0] = 1; v53[1] = v21[0];
v54[0] = v50[0]; for (unsigned char index = 0; index < v50[0]; index++) {
                            v54[index+1] = v50[index+1] + v53[(index % v53[0])+1];
                        }
v55[0] = 1; v55[1] = v20[0];
if (v54[0] != 1 || v55[0] != 1 || v54[1] >= v20[0]) abort();
                v56[0] = MIN(v20[0] - v54[1], v55[1]);
                memcpy(v56+1, v20+v54[1]+1, v56[0]);
if (v21[0] + v56[0] > 255) abort();
                                    v57[0] = v21[0] + v56[0];
                                    memcpy(v57+1, v21+1, v21[0]);
                                    memcpy(v57+v21[0]+1, v56+1, v56[0]);
v58[0] = v50[0]; for (unsigned char index = 0; index < v50[0]; index++) {
                            v58[index+1] = v50[index+1] - v4[(index % v4[0])+1];
                        }
if (v4[0] != 1 || v58[0] != 1 || v4[1] >= v20[0]) abort();
                v59[0] = MIN(v20[0] - v4[1], v58[1]);
                memcpy(v59+1, v20+v4[1]+1, v59[0]);
if (v59[0] + v57[0] > 255) abort();
                                    v60[0] = v59[0] + v57[0];
                                    memcpy(v60+1, v59+1, v59[0]);
                                    memcpy(v60+v59[0]+1, v57+1, v57[0]);
memcpy(v20, v60, v60[0]+1);

                memcpy(v62, (char[2]){1, 0}, 2);
while(1) {
if (v62[0] == v14[0] && memcmp(v62+1, v14+1, v62[0]) == 0) {
break;
} else { 
 
}
memcpy(v63, v62, v62[0]+1);
v62[0] = v62[0]; for (unsigned char index = 0; index < v62[0]; index++) {
                            v62[index+1] = v62[index+1] + v0[(index % v0[0])+1];
                        }
if (v63[0] != 1 || v0[0] != 1 || v63[1] >= v60[0]) abort();
                v64[0] = MIN(v60[0] - v63[1], v0[1]);
                memcpy(v64+1, v60+v63[1]+1, v64[0]);
if (v64[0] == v9[0] && memcmp(v64+1, v9+1, v64[0]) == 0) {
continue;
} else { 
 
}
v65[0] = v63[0]; for (unsigned char index = 0; index < v63[0]; index++) {
                            v65[index+1] = v63[index+1] + v7[(index % v7[0])+1];
                        }
if (v65[0] != 1 || v0[0] != 1 || v65[1] >= v60[0]) abort();
                v66[0] = MIN(v60[0] - v65[1], v0[1]);
                memcpy(v66+1, v60+v65[1]+1, v66[0]);
v67[0] = v63[0]; for (unsigned char index = 0; index < v63[0]; index++) {
                            v67[index+1] = v63[index+1] + v14[(index % v14[0])+1];
                        }
if (v67[0] != 1 || v0[0] != 1 || v67[1] >= v60[0]) abort();
                v68[0] = MIN(v60[0] - v67[1], v0[1]);
                memcpy(v68+1, v60+v67[1]+1, v68[0]);
v69[0] = v68[0]; for (unsigned char index = 0; index < v68[0]; index++) {
                            v69[index+1] = v68[index+1] + v66[(index % v66[0])+1];
                        }
if (v63[0] != 1 || v0[0] != 1 || v63[1] >= v60[0]) abort();
                v70[0] = MIN(v60[0] - v63[1], v0[1]);
                memcpy(v70+1, v60+v63[1]+1, v70[0]);
v71[0] = v70[0]; for (unsigned char index = 0; index < v70[0]; index++) {
                            v71[index+1] = v70[index+1] + v69[(index % v69[0])+1];
                        }
if (v71[0] == v6[0] && memcmp(v71+1, v6+1, v71[0]) == 0) {
memcpy(v61, v13, v13[0]+1); goto fn3;
} else { 
 
}
if (v71[0] == v15[0] && memcmp(v71+1, v15+1, v71[0]) == 0) {
memcpy(v61, v17, v17[0]+1); goto fn3;
} else { 
 
}
v72[0] = v63[0]; for (unsigned char index = 0; index < v63[0]; index++) {
                            v72[index+1] = v63[index+1] * v14[(index % v14[0])+1];
                        }
v73[0] = v72[0]; for (unsigned char index = 0; index < v72[0]; index++) {
                            v73[index+1] = v72[index+1] + v1[(index % v1[0])+1];
                        }
if (v73[0] != 1 || v0[0] != 1 || v73[1] >= v60[0]) abort();
                v74[0] = MIN(v60[0] - v73[1], v0[1]);
                memcpy(v74+1, v60+v73[1]+1, v74[0]);
v75[0] = v72[0]; for (unsigned char index = 0; index < v72[0]; index++) {
                            v75[index+1] = v72[index+1] + v0[(index % v0[0])+1];
                        }
if (v75[0] != 1 || v0[0] != 1 || v75[1] >= v60[0]) abort();
                v76[0] = MIN(v60[0] - v75[1], v0[1]);
                memcpy(v76+1, v60+v75[1]+1, v76[0]);
v77[0] = v76[0]; for (unsigned char index = 0; index < v76[0]; index++) {
                            v77[index+1] = v76[index+1] + v74[(index % v74[0])+1];
                        }
if (v72[0] != 1 || v0[0] != 1 || v72[1] >= v60[0]) abort();
                v78[0] = MIN(v60[0] - v72[1], v0[1]);
                memcpy(v78+1, v60+v72[1]+1, v78[0]);
v79[0] = v78[0]; for (unsigned char index = 0; index < v78[0]; index++) {
                            v79[index+1] = v78[index+1] + v77[(index % v77[0])+1];
                        }
if (v79[0] == v6[0] && memcmp(v79+1, v6+1, v79[0]) == 0) {
memcpy(v61, v13, v13[0]+1); goto fn3;
} else { 
 
}
if (v79[0] == v15[0] && memcmp(v79+1, v15+1, v79[0]) == 0) {
memcpy(v61, v17, v17[0]+1); goto fn3;
} else { 
 
}
}
if (v6[0] != 1 || v0[0] != 1 || v6[1] >= v60[0]) abort();
                v80[0] = MIN(v60[0] - v6[1], v0[1]);
                memcpy(v80+1, v60+v6[1]+1, v80[0]);
if (v3[0] != 1 || v0[0] != 1 || v3[1] >= v60[0]) abort();
                v81[0] = MIN(v60[0] - v3[1], v0[1]);
                memcpy(v81+1, v60+v3[1]+1, v81[0]);
v82[0] = v81[0]; for (unsigned char index = 0; index < v81[0]; index++) {
                            v82[index+1] = v81[index+1] + v80[(index % v80[0])+1];
                        }
if (v4[0] != 1 || v0[0] != 1 || v4[1] >= v60[0]) abort();
                v83[0] = MIN(v60[0] - v4[1], v0[1]);
                memcpy(v83+1, v60+v4[1]+1, v83[0]);
v84[0] = v83[0]; for (unsigned char index = 0; index < v83[0]; index++) {
                            v84[index+1] = v83[index+1] + v82[(index % v82[0])+1];
                        }
if (v84[0] == v6[0] && memcmp(v84+1, v6+1, v84[0]) == 0) {
memcpy(v61, v13, v13[0]+1); goto fn3;
} else { 
 
}
if (v84[0] == v15[0] && memcmp(v84+1, v15+1, v84[0]) == 0) {
memcpy(v61, v17, v17[0]+1); goto fn3;
} else { 
 
}
if (v7[0] != 1 || v0[0] != 1 || v7[1] >= v60[0]) abort();
                v85[0] = MIN(v60[0] - v7[1], v0[1]);
                memcpy(v85+1, v60+v7[1]+1, v85[0]);
if (v3[0] != 1 || v0[0] != 1 || v3[1] >= v60[0]) abort();
                v86[0] = MIN(v60[0] - v3[1], v0[1]);
                memcpy(v86+1, v60+v3[1]+1, v86[0]);
v87[0] = v86[0]; for (unsigned char index = 0; index < v86[0]; index++) {
                            v87[index+1] = v86[index+1] + v85[(index % v85[0])+1];
                        }
if (v1[0] != 1 || v0[0] != 1 || v1[1] >= v60[0]) abort();
                v88[0] = MIN(v60[0] - v1[1], v0[1]);
                memcpy(v88+1, v60+v1[1]+1, v88[0]);
v89[0] = v88[0]; for (unsigned char index = 0; index < v88[0]; index++) {
                            v89[index+1] = v88[index+1] + v87[(index % v87[0])+1];
                        }
if (v89[0] == v6[0] && memcmp(v89+1, v6+1, v89[0]) == 0) {
memcpy(v61, v13, v13[0]+1); goto fn3;
} else { 
 
}
if (v89[0] == v15[0] && memcmp(v89+1, v15+1, v89[0]) == 0) {
memcpy(v61, v17, v17[0]+1); goto fn3;
} else { 
 
}
memcpy(v61, v9, v9[0]+1); goto fn3;
                fn3:
                
if (v61[0] == v9[0] && memcmp(v61+1, v9+1, v61[0]) == 0) {

} else { 
memcpy(v90, (char[11]){10, 32, 104, 97, 115, 32, 119, 111, 110, 33, 10}, 11);
if (v61[0] + v90[0] > 255) abort();
                                    v91[0] = v61[0] + v90[0];
                                    memcpy(v91+1, v61+1, v61[0]);
                                    memcpy(v91+v61[0]+1, v90+1, v90[0]);
v91[v91[0]+1] = 0; printf("%s", v91+1);
exit(0); 
}

                v93[0] = 1; v93[1] = v60[0];
memcpy(v94, (char[2]){1, 0}, 2);
while(1) {
if (v93[0] == v94[0] && memcmp(v93+1, v94+1, v93[0]) == 0) {
break;
} else { 
 
}
if (v94[0] != 1 || v0[0] != 1 || v94[1] >= v60[0]) abort();
                v95[0] = MIN(v60[0] - v94[1], v0[1]);
                memcpy(v95+1, v60+v94[1]+1, v95[0]);
v94[0] = v94[0]; for (unsigned char index = 0; index < v94[0]; index++) {
                            v94[index+1] = v94[index+1] + v0[(index % v0[0])+1];
                        }
if (v95[0] == v9[0] && memcmp(v95+1, v9+1, v95[0]) == 0) {
memcpy(v92, v0, v0[0]+1); goto fn4;
} else { 
 
}
}
memcpy(v92, v4, v4[0]+1); goto fn4;
                fn4:
                
if (v92[0] == v4[0] && memcmp(v92+1, v4+1, v92[0]) == 0) {
memcpy(v96, (char[13]){12, 78, 111, 98, 111, 100, 121, 32, 119, 111, 110, 33, 10}, 13);
v96[v96[0]+1] = 0; printf("%s", v96+1);
exit(0);
} else { 
 
}
memcpy(v51, v4, v4[0]+1); goto fn2;
                fn2:
                
if (v51[0] == v0[0] && memcmp(v51+1, v0+1, v51[0]) == 0) {
v16[v16[0]+1] = 0; printf("%s", v16+1);
continue;
} else { 
 
}
if (v21[0] == v17[0] && memcmp(v21+1, v17+1, v21[0]) == 0) {
memcpy(v21, (char[2]){1, 88}, 2);
} else { 
memcpy(v21, (char[2]){1, 79}, 2); 
}
}
exit(0);
}