use std::io::BufRead;
use std::io::Write;

#[derive(Default)]
struct Stack {
    v38: Vec<u8>,
    v55: Vec<u8>,
    v59: Vec<u8>,
    v78: Vec<u8>,
    v89: Vec<u8>,
    v46: Vec<u8>,
    v60: Vec<u8>,
    v1: Vec<u8>,
    v54: Vec<u8>,
    v69: Vec<u8>,
    v91: Vec<u8>,
    v20: Vec<u8>,
    v39: Vec<u8>,
    v50: Vec<u8>,
    v53: Vec<u8>,
    v80: Vec<u8>,
    v92: Vec<u8>,
    v48: Vec<u8>,
    v37: Vec<u8>,
    v75: Vec<u8>,
    v63: Vec<u8>,
    v81: Vec<u8>,
    v57: Vec<u8>,
    v29: Vec<u8>,
    v83: Vec<u8>,
    v90: Vec<u8>,
    v9: Vec<u8>,
    v13: Vec<u8>,
    v43: Vec<u8>,
    v28: Vec<u8>,
    v34: Vec<u8>,
    v19: Vec<u8>,
    v47: Vec<u8>,
    v30: Vec<u8>,
    v21: Vec<u8>,
    v24: Vec<u8>,
    v65: Vec<u8>,
    v66: Vec<u8>,
    v68: Vec<u8>,
    v49: Vec<u8>,
    v72: Vec<u8>,
    v3: Vec<u8>,
    v88: Vec<u8>,
    v94: Vec<u8>,
    v71: Vec<u8>,
    v35: Vec<u8>,
    v62: Vec<u8>,
    v93: Vec<u8>,
    v45: Vec<u8>,
    v12: Vec<u8>,
    v41: Vec<u8>,
    v4: Vec<u8>,
    v25: Vec<u8>,
    v2: Vec<u8>,
    v51: Vec<u8>,
    v77: Vec<u8>,
    v40: Vec<u8>,
    v5: Vec<u8>,
    v42: Vec<u8>,
    v67: Vec<u8>,
    v87: Vec<u8>,
    v33: Vec<u8>,
    v74: Vec<u8>,
    v32: Vec<u8>,
    v82: Vec<u8>,
    v6: Vec<u8>,
    v58: Vec<u8>,
    v85: Vec<u8>,
    v22: Vec<u8>,
    v0: Vec<u8>,
    v14: Vec<u8>,
    v76: Vec<u8>,
    v79: Vec<u8>,
    v95: Vec<u8>,
    v16: Vec<u8>,
    v73: Vec<u8>,
    v11: Vec<u8>,
    v27: Vec<u8>,
    v23: Vec<u8>,
    v84: Vec<u8>,
    v61: Vec<u8>,
    v31: Vec<u8>,
    v44: Vec<u8>,
    v8: Vec<u8>,
    v52: Vec<u8>,
    v64: Vec<u8>,
    v15: Vec<u8>,
    v17: Vec<u8>,
    v36: Vec<u8>,
    v86: Vec<u8>,
    v26: Vec<u8>,
    v56: Vec<u8>,
    v70: Vec<u8>,
    v18: Vec<u8>,
    v10: Vec<u8>,
    v7: Vec<u8>,
}

fn main() {
    let mut stack = &mut Stack::default();
    stack.v0 = vec![5];
    stack.v1 = vec![0];
    stack.v2 = vec![9];
    stack.v3 = vec![237];
    stack.v4 = vec![3];
    stack.v5 = vec![6];
    stack.v6 = vec![4];
    stack.v7 = vec![32, 124, 32];
    stack.v8 = vec![69, 110, 116, 101, 114, 32, 97, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 116, 111, 32, 112, 108, 97, 121, 32, 105, 110, 32, 58, 32];
    stack.v9 = vec![65, 108, 100, 114, 101, 97, 100, 121, 32, 115, 111, 109, 101, 116, 104, 105, 110, 103, 32, 112, 108, 97, 99, 101, 100, 32, 104, 101, 114, 101, 10];
    stack.v10 = vec![49, 50, 51, 52, 53, 54, 55, 56, 57];
    stack.v11 = vec![73, 110, 118, 97, 108, 105, 100, 32, 105, 110, 112, 117, 116, 10];
    stack.v12 = vec![79];
    stack.v13 = vec![7];
    stack.v14 = vec![2];
    stack.v15 = vec![8];
    stack.v16 = vec![32];
    stack.v17 = vec![10];
    stack.v18 = vec![45, 45, 45, 45, 45, 45, 45, 45, 45, 10];
    stack.v19 = vec![49];
    stack.v20 = vec![1];
    stack.v21 = vec![32, 32, 32, 32, 32, 32, 32, 32, 32];
    stack.v22 = vec![88];
    loop {
        stack.v23 = stack.v21.iter().skip(stack.v14[0] as usize).take(stack.v20[0] as usize).copied().collect();
        stack.v24 = stack.v23.iter().chain(stack.v17.iter()).copied().collect();
        stack.v25 = stack.v7.iter().chain(stack.v24.iter()).copied().collect();
        stack.v26 = stack.v21.iter().skip(stack.v20[0] as usize).take(stack.v20[0] as usize).copied().collect();
        stack.v27 = stack.v26.iter().chain(stack.v25.iter()).copied().collect();
        stack.v28 = stack.v7.iter().chain(stack.v27.iter()).copied().collect();
        stack.v29 = stack.v21.iter().skip(stack.v1[0] as usize).take(stack.v20[0] as usize).copied().collect();
        stack.v30 = stack.v29.iter().chain(stack.v28.iter()).copied().collect();
        std::io::stdout().lock().write(&stack.v30).unwrap();
        std::io::stdout().lock().flush().unwrap();
        std::io::stdout().lock().write(&stack.v18).unwrap();
        std::io::stdout().lock().flush().unwrap();
        stack.v31 = stack.v21.iter().skip(stack.v0[0] as usize).take(stack.v20[0] as usize).copied().collect();
        stack.v32 = stack.v31.iter().chain(stack.v17.iter()).copied().collect();
        stack.v33 = stack.v7.iter().chain(stack.v32.iter()).copied().collect();
        stack.v34 = stack.v21.iter().skip(stack.v6[0] as usize).take(stack.v20[0] as usize).copied().collect();
        stack.v35 = stack.v34.iter().chain(stack.v33.iter()).copied().collect();
        stack.v36 = stack.v7.iter().chain(stack.v35.iter()).copied().collect();
        stack.v37 = stack.v21.iter().skip(stack.v4[0] as usize).take(stack.v20[0] as usize).copied().collect();
        stack.v38 = stack.v37.iter().chain(stack.v36.iter()).copied().collect();
        std::io::stdout().lock().write(&stack.v38).unwrap();
        std::io::stdout().lock().flush().unwrap();
        std::io::stdout().lock().write(&stack.v18).unwrap();
        std::io::stdout().lock().flush().unwrap();
        stack.v39 = stack.v21.iter().skip(stack.v15[0] as usize).take(stack.v20[0] as usize).copied().collect();
        stack.v40 = stack.v39.iter().chain(stack.v17.iter()).copied().collect();
        stack.v41 = stack.v7.iter().chain(stack.v40.iter()).copied().collect();
        stack.v42 = stack.v21.iter().skip(stack.v13[0] as usize).take(stack.v20[0] as usize).copied().collect();
        stack.v43 = stack.v42.iter().chain(stack.v41.iter()).copied().collect();
        stack.v44 = stack.v7.iter().chain(stack.v43.iter()).copied().collect();
        stack.v45 = stack.v21.iter().skip(stack.v5[0] as usize).take(stack.v20[0] as usize).copied().collect();
        stack.v46 = stack.v45.iter().chain(stack.v44.iter()).copied().collect();
        std::io::stdout().lock().write(&stack.v46).unwrap();
        std::io::stdout().lock().flush().unwrap();
        std::io::stdout().lock().write(&stack.v8).unwrap();
        std::io::stdout().lock().flush().unwrap();
        stack.v47 = Vec::with_capacity(255);
        std::io::stdin().lock().read_until(10, &mut stack.v47).unwrap();
        while let Some(13) | Some(10) = stack.v47.last() {
            stack.v47.pop();
        }
        std::io::stdout().lock().write(&stack.v17).unwrap();
        std::io::stdout().lock().flush().unwrap();
        stack.v48 = vec![stack.v47.len() as u8];
        if stack.v48 != stack.v20 {
            std::io::stdout().lock().write(&stack.v11).unwrap();
            std::io::stdout().lock().flush().unwrap();
            continue;
        }
        stack.v49 = vec![0];
        loop {
            if stack.v2 == stack.v49 {
                stack.v50 = vec![0];
                break;
            }
            stack.v51 = stack.v10.iter().skip(stack.v49[0] as usize).take(stack.v20[0] as usize).copied().collect();
            stack.v49 = stack.v49.iter().zip(stack.v20.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
            if stack.v51 == stack.v47 {
                stack.v50 = vec![1];
                break;
            }
        }
        if stack.v50 == stack.v1 {
            std::io::stdout().lock().write(&stack.v11).unwrap();
            std::io::stdout().lock().flush().unwrap();
            continue;
        }
        stack.v52 = stack.v47.iter().zip(stack.v19.iter().cycle()).map(|(a,b)| a - b).collect();
        fn f1(stack: &mut Stack) -> Vec<u8> {
            stack.v54 = stack.v21.iter().skip(stack.v52[0] as usize).take(stack.v20[0] as usize).copied().collect();
            if stack.v54 != stack.v16 {
                return stack.v20.clone();
            }
            stack.v55 = vec![stack.v22.len() as u8];
            stack.v56 = stack.v52.iter().zip(stack.v55.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
            stack.v57 = vec![stack.v21.len() as u8];
            stack.v58 = stack.v21.iter().skip(stack.v56[0] as usize).take(stack.v57[0] as usize).copied().collect();
            stack.v59 = stack.v22.iter().chain(stack.v58.iter()).copied().collect();
            stack.v60 = stack.v52.iter().zip(stack.v1.iter().cycle()).map(|(a,b)| a - b).collect();
            stack.v61 = stack.v21.iter().skip(stack.v1[0] as usize).take(stack.v60[0] as usize).copied().collect();
            stack.v62 = stack.v61.iter().chain(stack.v59.iter()).copied().collect();
            stack.v21 = stack.v62.clone();
            stack.v63 = vec![0];
            loop {
                if stack.v63 == stack.v4 {
                    stack.v64 = stack.v62.iter().skip(stack.v15[0] as usize).take(stack.v20[0] as usize).copied().collect();
                    stack.v65 = stack.v62.iter().skip(stack.v6[0] as usize).take(stack.v20[0] as usize).copied().collect();
                    stack.v66 = stack.v65.iter().zip(stack.v64.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                    stack.v67 = stack.v62.iter().skip(stack.v1[0] as usize).take(stack.v20[0] as usize).copied().collect();
                    stack.v68 = stack.v67.iter().zip(stack.v66.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                    if stack.v68 == stack.v15 {
                        stack.v69 = vec![88, 32, 104, 97, 115, 32, 119, 111, 110, 33, 10];
                        std::io::stdout().lock().write(&stack.v69).unwrap();
                        std::io::stdout().lock().flush().unwrap();
                        std::process::exit(0);
                    }
                    if stack.v68 == stack.v3 {
                        stack.v69 = vec![79, 32, 104, 97, 115, 32, 119, 111, 110, 33, 10];
                        std::io::stdout().lock().write(&stack.v69).unwrap();
                        std::io::stdout().lock().flush().unwrap();
                        std::process::exit(0);
                    }
                    stack.v70 = stack.v62.iter().skip(stack.v5[0] as usize).take(stack.v20[0] as usize).copied().collect();
                    stack.v71 = stack.v62.iter().skip(stack.v6[0] as usize).take(stack.v20[0] as usize).copied().collect();
                    stack.v72 = stack.v71.iter().zip(stack.v70.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                    stack.v73 = stack.v62.iter().skip(stack.v14[0] as usize).take(stack.v20[0] as usize).copied().collect();
                    stack.v74 = stack.v73.iter().zip(stack.v72.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                    if stack.v74 == stack.v15 {
                        stack.v69 = vec![88, 32, 104, 97, 115, 32, 119, 111, 110, 33, 10];
                        std::io::stdout().lock().write(&stack.v69).unwrap();
                        std::io::stdout().lock().flush().unwrap();
                        std::process::exit(0);
                    }
                    if stack.v74 == stack.v3 {
                        stack.v69 = vec![79, 32, 104, 97, 115, 32, 119, 111, 110, 33, 10];
                        std::io::stdout().lock().write(&stack.v69).unwrap();
                        std::io::stdout().lock().flush().unwrap();
                        std::process::exit(0);
                    }
                    stack.v75 = vec![stack.v62.len() as u8];
                    stack.v76 = vec![0];
                    loop {
                        if stack.v75 == stack.v76 {
                            stack.v77 = vec![78, 111, 98, 111, 100, 121, 32, 119, 111, 110, 33, 10];
                            std::io::stdout().lock().write(&stack.v77).unwrap();
                            std::io::stdout().lock().flush().unwrap();
                            std::process::exit(0);
                        }
                        stack.v78 = stack.v62.iter().skip(stack.v76[0] as usize).take(stack.v20[0] as usize).copied().collect();
                        stack.v76 = stack.v76.iter().zip(stack.v20.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                        if stack.v78 == stack.v16 {
                            return stack.v1.clone();
                        }
                    }
                }
                stack.v79 = stack.v63.clone();
                stack.v63 = stack.v63.iter().zip(stack.v20.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                stack.v80 = stack.v62.iter().skip(stack.v79[0] as usize).take(stack.v20[0] as usize).copied().collect();
                if stack.v80 == stack.v16 {
                    continue;
                }
                stack.v81 = stack.v79.iter().zip(stack.v5.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                stack.v82 = stack.v62.iter().skip(stack.v81[0] as usize).take(stack.v20[0] as usize).copied().collect();
                stack.v83 = stack.v79.iter().zip(stack.v4.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                stack.v84 = stack.v62.iter().skip(stack.v83[0] as usize).take(stack.v20[0] as usize).copied().collect();
                stack.v85 = stack.v84.iter().zip(stack.v82.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                stack.v86 = stack.v62.iter().skip(stack.v79[0] as usize).take(stack.v20[0] as usize).copied().collect();
                stack.v87 = stack.v86.iter().zip(stack.v85.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                if stack.v87 == stack.v15 {
                    stack.v69 = vec![88, 32, 104, 97, 115, 32, 119, 111, 110, 33, 10];
                    std::io::stdout().lock().write(&stack.v69).unwrap();
                    std::io::stdout().lock().flush().unwrap();
                    std::process::exit(0);
                }
                if stack.v87 == stack.v3 {
                    stack.v69 = vec![79, 32, 104, 97, 115, 32, 119, 111, 110, 33, 10];
                    std::io::stdout().lock().write(&stack.v69).unwrap();
                    std::io::stdout().lock().flush().unwrap();
                    std::process::exit(0);
                }
                stack.v88 = stack.v79.iter().zip(stack.v4.iter().cycle()).map(|(a,b)| (*a).wrapping_mul(*b)).collect();
                stack.v89 = stack.v88.iter().zip(stack.v14.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                stack.v90 = stack.v62.iter().skip(stack.v89[0] as usize).take(stack.v20[0] as usize).copied().collect();
                stack.v91 = stack.v88.iter().zip(stack.v20.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                stack.v92 = stack.v62.iter().skip(stack.v91[0] as usize).take(stack.v20[0] as usize).copied().collect();
                stack.v93 = stack.v92.iter().zip(stack.v90.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                stack.v94 = stack.v62.iter().skip(stack.v88[0] as usize).take(stack.v20[0] as usize).copied().collect();
                stack.v95 = stack.v94.iter().zip(stack.v93.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
                if stack.v95 == stack.v15 {
                    stack.v69 = vec![88, 32, 104, 97, 115, 32, 119, 111, 110, 33, 10];
                    std::io::stdout().lock().write(&stack.v69).unwrap();
                    std::io::stdout().lock().flush().unwrap();
                    std::process::exit(0);
                }
                if stack.v95 == stack.v3 {
                    stack.v69 = vec![79, 32, 104, 97, 115, 32, 119, 111, 110, 33, 10];
                    std::io::stdout().lock().write(&stack.v69).unwrap();
                    std::io::stdout().lock().flush().unwrap();
                    std::process::exit(0);
                }
            }
        }
        stack.v53 = f1(stack);
        if stack.v53 == stack.v20 {
            std::io::stdout().lock().write(&stack.v9).unwrap();
            std::io::stdout().lock().flush().unwrap();
            continue;
        }
        if stack.v22 == stack.v12 {
            stack.v22 = vec![88];
        } else {
            stack.v22 = vec![79];
        }
    }

}
