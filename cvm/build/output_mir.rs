use std::io::BufRead;
use std::io::Write;

#[derive(Default)]
struct Stack {
    v39: Vec<u8>,
    v47: Vec<u8>,
    v34: Vec<u8>,
    v14: Vec<u8>,
    v29: Vec<u8>,
    v3: Vec<u8>,
    v55: Vec<u8>,
    v93: Vec<u8>,
    v90: Vec<u8>,
    v28: Vec<u8>,
    v38: Vec<u8>,
    v1: Vec<u8>,
    v82: Vec<u8>,
    v21: Vec<u8>,
    v11: Vec<u8>,
    v9: Vec<u8>,
    v32: Vec<u8>,
    v20: Vec<u8>,
    v31: Vec<u8>,
    v74: Vec<u8>,
    v81: Vec<u8>,
    v83: Vec<u8>,
    v46: Vec<u8>,
    v86: Vec<u8>,
    v33: Vec<u8>,
    v4: Vec<u8>,
    v5: Vec<u8>,
    v37: Vec<u8>,
    v17: Vec<u8>,
    v92: Vec<u8>,
    v77: Vec<u8>,
    v89: Vec<u8>,
    v71: Vec<u8>,
    v69: Vec<u8>,
    v94: Vec<u8>,
    v95: Vec<u8>,
    v85: Vec<u8>,
    v7: Vec<u8>,
    v61: Vec<u8>,
    v15: Vec<u8>,
    v44: Vec<u8>,
    v0: Vec<u8>,
    v51: Vec<u8>,
    v27: Vec<u8>,
    v41: Vec<u8>,
    v72: Vec<u8>,
    v42: Vec<u8>,
    v10: Vec<u8>,
    v25: Vec<u8>,
    v57: Vec<u8>,
    v64: Vec<u8>,
    v53: Vec<u8>,
    v73: Vec<u8>,
    v84: Vec<u8>,
    v8: Vec<u8>,
    v45: Vec<u8>,
    v65: Vec<u8>,
    v75: Vec<u8>,
    v48: Vec<u8>,
    v80: Vec<u8>,
    v88: Vec<u8>,
    v63: Vec<u8>,
    v56: Vec<u8>,
    v6: Vec<u8>,
    v68: Vec<u8>,
    v78: Vec<u8>,
    v91: Vec<u8>,
    v49: Vec<u8>,
    v96: Vec<u8>,
    v26: Vec<u8>,
    v19: Vec<u8>,
    v13: Vec<u8>,
    v50: Vec<u8>,
    v2: Vec<u8>,
    v24: Vec<u8>,
    v22: Vec<u8>,
    v16: Vec<u8>,
    v30: Vec<u8>,
    v36: Vec<u8>,
    v58: Vec<u8>,
    v60: Vec<u8>,
    v67: Vec<u8>,
    v43: Vec<u8>,
    v70: Vec<u8>,
    v79: Vec<u8>,
    v52: Vec<u8>,
    v40: Vec<u8>,
    v62: Vec<u8>,
    v66: Vec<u8>,
    v18: Vec<u8>,
    v35: Vec<u8>,
    v59: Vec<u8>,
    v54: Vec<u8>,
    v23: Vec<u8>,
    v76: Vec<u8>,
    v87: Vec<u8>,
    v12: Vec<u8>,
}

fn main() {
    let mut stack = &mut Stack::default();
    stack.v0 = vec![1];
    stack.v1 = vec![2];
    stack.v2 = vec![5];
    stack.v3 = vec![4];
    stack.v4 = vec![0];
    stack.v5 = vec![73, 110, 118, 97, 108, 105, 100, 32, 105, 110, 112, 117, 116, 10];
    stack.v6 = vec![8];
    stack.v7 = vec![6];
    stack.v8 = vec![10];
    stack.v9 = vec![32];
    stack.v10 = vec![69, 110, 116, 101, 114, 32, 97, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 116, 111, 32, 112, 108, 97, 121, 32, 105, 110, 32, 58, 32];
    stack.v11 = vec![49];
    stack.v12 = vec![7];
    stack.v13 = vec![88];
    stack.v14 = vec![3];
    stack.v15 = vec![237];
    stack.v16 = vec![65, 108, 100, 114, 101, 97, 100, 121, 32, 115, 111, 109, 101, 116, 104, 105, 110, 103, 32, 112, 108, 97, 99, 101, 100, 32, 104, 101, 114, 101, 10];
    stack.v17 = vec![79];
    stack.v18 = vec![45, 45, 45, 45, 45, 45, 45, 45, 45, 10];
    stack.v19 = vec![32, 124, 32];
    stack.v20 = vec![32, 32, 32, 32, 32, 32, 32, 32, 32];
    stack.v21 = vec![88];
    loop {
        stack.v22 = stack.v20.iter().skip(stack.v1[0] as usize).take(stack.v0[0] as usize).copied().collect();
        stack.v23 = stack.v22.iter().chain(stack.v8.iter()).copied().collect();
        stack.v24 = stack.v19.iter().chain(stack.v23.iter()).copied().collect();
        stack.v25 = stack.v20.iter().skip(stack.v0[0] as usize).take(stack.v0[0] as usize).copied().collect();
        stack.v26 = stack.v25.iter().chain(stack.v24.iter()).copied().collect();
        stack.v27 = stack.v19.iter().chain(stack.v26.iter()).copied().collect();
        stack.v28 = stack.v20.iter().skip(stack.v4[0] as usize).take(stack.v0[0] as usize).copied().collect();
        stack.v29 = stack.v28.iter().chain(stack.v27.iter()).copied().collect();
        std::io::stdout().lock().write(&stack.v29).unwrap();
        std::io::stdout().lock().flush().unwrap();
        std::io::stdout().lock().write(&stack.v18).unwrap();
        std::io::stdout().lock().flush().unwrap();
        stack.v30 = stack.v20.iter().skip(stack.v2[0] as usize).take(stack.v0[0] as usize).copied().collect();
        stack.v31 = stack.v30.iter().chain(stack.v8.iter()).copied().collect();
        stack.v32 = stack.v19.iter().chain(stack.v31.iter()).copied().collect();
        stack.v33 = stack.v20.iter().skip(stack.v3[0] as usize).take(stack.v0[0] as usize).copied().collect();
        stack.v34 = stack.v33.iter().chain(stack.v32.iter()).copied().collect();
        stack.v35 = stack.v19.iter().chain(stack.v34.iter()).copied().collect();
        stack.v36 = stack.v20.iter().skip(stack.v14[0] as usize).take(stack.v0[0] as usize).copied().collect();
        stack.v37 = stack.v36.iter().chain(stack.v35.iter()).copied().collect();
        std::io::stdout().lock().write(&stack.v37).unwrap();
        std::io::stdout().lock().flush().unwrap();
        std::io::stdout().lock().write(&stack.v18).unwrap();
        std::io::stdout().lock().flush().unwrap();
        stack.v38 = stack.v20.iter().skip(stack.v6[0] as usize).take(stack.v0[0] as usize).copied().collect();
        stack.v39 = stack.v38.iter().chain(stack.v8.iter()).copied().collect();
        stack.v40 = stack.v19.iter().chain(stack.v39.iter()).copied().collect();
        stack.v41 = stack.v20.iter().skip(stack.v12[0] as usize).take(stack.v0[0] as usize).copied().collect();
        stack.v42 = stack.v41.iter().chain(stack.v40.iter()).copied().collect();
        stack.v43 = stack.v19.iter().chain(stack.v42.iter()).copied().collect();
        stack.v44 = stack.v20.iter().skip(stack.v7[0] as usize).take(stack.v0[0] as usize).copied().collect();
        stack.v45 = stack.v44.iter().chain(stack.v43.iter()).copied().collect();
        std::io::stdout().lock().write(&stack.v45).unwrap();
        std::io::stdout().lock().flush().unwrap();
        std::io::stdout().lock().write(&stack.v10).unwrap();
        std::io::stdout().lock().flush().unwrap();
        stack.v46 = Vec::with_capacity(255);
        std::io::stdin().lock().read_until(10, &mut stack.v46).unwrap();
        while let Some(13) | Some(10) = stack.v46.last() {
            stack.v46.pop();
        }
        std::io::stdout().lock().write(&stack.v8).unwrap();
        std::io::stdout().lock().flush().unwrap();
        stack.v47 = vec![stack.v46.len() as u8];
        if stack.v47 != stack.v0 {
            std::io::stdout().lock().write(&stack.v5).unwrap();
            std::io::stdout().lock().flush().unwrap();
            continue;
        }
        fn f1(stack: &mut Stack) -> Vec<u8> {
            stack.v49 = vec![49];
            if stack.v49 == stack.v46 {
                return stack.v0.clone();
            }
            stack.v49 = vec![50];
            if stack.v49 == stack.v46 {
                return stack.v0.clone();
            }
            stack.v49 = vec![51];
            if stack.v49 == stack.v46 {
                return stack.v0.clone();
            }
            stack.v49 = vec![52];
            if stack.v49 == stack.v46 {
                return stack.v0.clone();
            }
            stack.v49 = vec![53];
            if stack.v49 == stack.v46 {
                return stack.v0.clone();
            }
            stack.v49 = vec![54];
            if stack.v49 == stack.v46 {
                return stack.v0.clone();
            }
            stack.v49 = vec![55];
            if stack.v49 == stack.v46 {
                return stack.v0.clone();
            }
            stack.v49 = vec![56];
            if stack.v49 == stack.v46 {
                return stack.v0.clone();
            }
            stack.v49 = vec![57];
            if stack.v49 == stack.v46 {
                return stack.v0.clone();
            }
            return stack.v4.clone();
        }
        stack.v48 = f1(stack);
        if stack.v48 == stack.v4 {
            std::io::stdout().lock().write(&stack.v5).unwrap();
            std::io::stdout().lock().flush().unwrap();
            continue;
        }
        stack.v50 = stack.v46.iter().zip(stack.v11.iter().cycle()).map(|(a,b)| a - b).collect();
        fn f2(stack: &mut Stack) -> Vec<u8> {
            stack.v52 = stack.v20.iter().skip(stack.v50[0] as usize).take(stack.v0[0] as usize).copied().collect();
            if stack.v52 != stack.v9 {
                return stack.v0.clone();
            }
            stack.v53 = vec![stack.v21.len() as u8];
            stack.v54 = stack.v50.iter().zip(stack.v53.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
            stack.v55 = vec![stack.v20.len() as u8];
            stack.v56 = stack.v20.iter().skip(stack.v54[0] as usize).take(stack.v55[0] as usize).copied().collect();
            stack.v57 = stack.v21.iter().chain(stack.v56.iter()).copied().collect();
            stack.v58 = stack.v50.iter().zip(stack.v4.iter().cycle()).map(|(a,b)| a - b).collect();
            stack.v59 = stack.v20.iter().skip(stack.v4[0] as usize).take(stack.v58[0] as usize).copied().collect();
            stack.v60 = stack.v59.iter().chain(stack.v57.iter()).copied().collect();
            stack.v20 = stack.v60.clone();
            fn f3(stack: &mut Stack) -> Vec<u8> {
                stack.v62 = vec![0];
                loop {
                    if stack.v62 == stack.v14 {
                        break;
                    }
                    stack.v63 = stack.v62.clone();
                    stack.v62 = stack.v62.iter().zip(stack.v0.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                    stack.v64 = stack.v60.iter().skip(stack.v63[0] as usize).take(stack.v0[0] as usize).copied().collect();
                    if stack.v64 == stack.v9 {
                        continue;
                    }
                    stack.v65 = stack.v63.iter().zip(stack.v7.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                    stack.v66 = stack.v60.iter().skip(stack.v65[0] as usize).take(stack.v0[0] as usize).copied().collect();
                    stack.v67 = stack.v63.iter().zip(stack.v14.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                    stack.v68 = stack.v60.iter().skip(stack.v67[0] as usize).take(stack.v0[0] as usize).copied().collect();
                    stack.v69 = stack.v68.iter().zip(stack.v66.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                    stack.v70 = stack.v60.iter().skip(stack.v63[0] as usize).take(stack.v0[0] as usize).copied().collect();
                    stack.v71 = stack.v70.iter().zip(stack.v69.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                    if stack.v71 == stack.v6 {
                        return stack.v13.clone();
                    }
                    if stack.v71 == stack.v15 {
                        return stack.v17.clone();
                    }
                    stack.v72 = stack.v63.iter().zip(stack.v14.iter().cycle()).map(|(a,b)| (*a).wrapping_mul(*b)).collect();
                    stack.v73 = stack.v72.iter().zip(stack.v1.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                    stack.v74 = stack.v60.iter().skip(stack.v73[0] as usize).take(stack.v0[0] as usize).copied().collect();
                    stack.v75 = stack.v72.iter().zip(stack.v0.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                    stack.v76 = stack.v60.iter().skip(stack.v75[0] as usize).take(stack.v0[0] as usize).copied().collect();
                    stack.v77 = stack.v76.iter().zip(stack.v74.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                    stack.v78 = stack.v60.iter().skip(stack.v72[0] as usize).take(stack.v0[0] as usize).copied().collect();
                    stack.v79 = stack.v78.iter().zip(stack.v77.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                    if stack.v79 == stack.v6 {
                        return stack.v13.clone();
                    }
                    if stack.v79 == stack.v15 {
                        return stack.v17.clone();
                    }
                }
                stack.v80 = stack.v60.iter().skip(stack.v6[0] as usize).take(stack.v0[0] as usize).copied().collect();
                stack.v81 = stack.v60.iter().skip(stack.v3[0] as usize).take(stack.v0[0] as usize).copied().collect();
                stack.v82 = stack.v81.iter().zip(stack.v80.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                stack.v83 = stack.v60.iter().skip(stack.v4[0] as usize).take(stack.v0[0] as usize).copied().collect();
                stack.v84 = stack.v83.iter().zip(stack.v82.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                if stack.v84 == stack.v6 {
                    return stack.v13.clone();
                }
                if stack.v84 == stack.v15 {
                    return stack.v17.clone();
                }
                stack.v85 = stack.v60.iter().skip(stack.v7[0] as usize).take(stack.v0[0] as usize).copied().collect();
                stack.v86 = stack.v60.iter().skip(stack.v3[0] as usize).take(stack.v0[0] as usize).copied().collect();
                stack.v87 = stack.v86.iter().zip(stack.v85.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                stack.v88 = stack.v60.iter().skip(stack.v1[0] as usize).take(stack.v0[0] as usize).copied().collect();
                stack.v89 = stack.v88.iter().zip(stack.v87.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                if stack.v89 == stack.v6 {
                    return stack.v13.clone();
                }
                if stack.v89 == stack.v15 {
                    return stack.v17.clone();
                }
                return stack.v9.clone();
            }
            stack.v61 = f3(stack);
            if stack.v61 != stack.v9 {
                stack.v90 = vec![32, 104, 97, 115, 32, 119, 111, 110, 33, 10];
                stack.v91 = stack.v61.iter().chain(stack.v90.iter()).copied().collect();
                std::io::stdout().lock().write(&stack.v91).unwrap();
                std::io::stdout().lock().flush().unwrap();
                std::process::exit(0);
            }
            fn f4(stack: &mut Stack) -> Vec<u8> {
                stack.v93 = vec![stack.v60.len() as u8];
                stack.v94 = vec![0];
                loop {
                    if stack.v93 == stack.v94 {
                        break;
                    }
                    stack.v95 = stack.v60.iter().skip(stack.v94[0] as usize).take(stack.v0[0] as usize).copied().collect();
                    stack.v94 = stack.v94.iter().zip(stack.v0.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                    if stack.v95 == stack.v9 {
                        return stack.v0.clone();
                    }
                }
                return stack.v4.clone();
            }
            stack.v92 = f4(stack);
            if stack.v92 == stack.v4 {
                stack.v96 = vec![78, 111, 98, 111, 100, 121, 32, 119, 111, 110, 33, 10];
                std::io::stdout().lock().write(&stack.v96).unwrap();
                std::io::stdout().lock().flush().unwrap();
                std::process::exit(0);
            }
            return stack.v4.clone();
        }
        stack.v51 = f2(stack);
        if stack.v51 == stack.v0 {
            std::io::stdout().lock().write(&stack.v16).unwrap();
            std::io::stdout().lock().flush().unwrap();
            continue;
        }
        if stack.v21 == stack.v17 {
            stack.v21 = vec![88];
        } else {
            stack.v21 = vec![79];
        }
    }
    std::process::exit(0);

}
