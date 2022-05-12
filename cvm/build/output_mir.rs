use std::io::BufRead;
use std::io::Write;

#[derive(Default)]
struct Stack {
    v24: Vec<u8>,
    v17: Vec<u8>,
    v5: Vec<u8>,
    v9: Vec<u8>,
    v28: Vec<u8>,
    v30: Vec<u8>,
    v31: Vec<u8>,
    v20: Vec<u8>,
    v14: Vec<u8>,
    v11: Vec<u8>,
    v10: Vec<u8>,
    v27: Vec<u8>,
    v1: Vec<u8>,
    v19: Vec<u8>,
    v6: Vec<u8>,
    v12: Vec<u8>,
    v23: Vec<u8>,
    v15: Vec<u8>,
    v21: Vec<u8>,
    v4: Vec<u8>,
    v16: Vec<u8>,
    v26: Vec<u8>,
    v25: Vec<u8>,
    v18: Vec<u8>,
    v32: Vec<u8>,
    v22: Vec<u8>,
    v7: Vec<u8>,
    v8: Vec<u8>,
    v13: Vec<u8>,
    v29: Vec<u8>,
    v0: Vec<u8>,
    v3: Vec<u8>,
    v2: Vec<u8>,
}

fn main() {
    let mut stack = &mut Stack::default();
    stack.v0 = vec![125];
    stack.v1 = vec![102, 97, 108, 115, 101];
    stack.v2 = vec![41];
    stack.v3 = vec![91];
    stack.v4 = vec![40];
    stack.v5 = vec![80, 97, 114, 101, 110, 116, 104, 101, 115, 105, 115, 32, 111, 107, 58, 32];
    stack.v6 = vec![10];
    stack.v7 = vec![116, 114, 117, 101];
    stack.v8 = vec![0];
    stack.v9 = vec![69, 110, 116, 101, 114, 32, 121, 111, 117, 114, 32, 40, 41, 91, 93, 123, 125, 32, 99, 111, 110, 116, 97, 105, 110, 105, 110, 103, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 58, 10];
    stack.v10 = vec![123];
    stack.v11 = vec![1];
    stack.v12 = vec![93];
    std::io::stdout().lock().write(&stack.v9).unwrap();
    std::io::stdout().lock().flush().unwrap();
    stack.v13 = Vec::with_capacity(255);
    std::io::stdin().lock().read_until(10, &mut stack.v13).unwrap();
    while let Some(13) | Some(10) = stack.v13.last() {
        stack.v13.pop();
    }
    stack.v14 = vec![0];
    stack.v15 = vec![0];
    stack.v16 = vec![0];
    stack.v17 = vec![stack.v13.len() as u8];
    loop {
        if stack.v17 == stack.v8 {
            if stack.v15 == stack.v8 {
                stack.v18 = vec![1];
            } else {
                stack.v18 = vec![0];
            }
            if stack.v16 == stack.v8 {
                stack.v19 = vec![1];
            } else {
                stack.v19 = vec![0];
            }
            if stack.v14 == stack.v8 {
                stack.v20 = vec![1];
            } else {
                stack.v20 = vec![0];
            }
            stack.v21 = stack.v20.iter().zip(stack.v19.iter().cycle()).map(|(a,b)| (*a).wrapping_mul(*b)).collect();
            stack.v22 = stack.v21.iter().zip(stack.v18.iter().cycle()).map(|(a,b)| (*a).wrapping_mul(*b)).collect();
            fn f1(stack: &mut Stack) -> Vec<u8> {
                if stack.v22 == stack.v11 {
                    return stack.v7.clone();
                }
                return stack.v1.clone();
            }
            stack.v23 = f1(stack);
            stack.v24 = stack.v23.iter().chain(stack.v6.iter()).copied().collect();
            stack.v25 = stack.v5.iter().chain(stack.v24.iter()).copied().collect();
            std::io::stdout().lock().write(&stack.v25).unwrap();
            std::io::stdout().lock().flush().unwrap();
            std::process::exit(0);
        }
        stack.v26 = stack.v17.iter().zip(stack.v11.iter().cycle()).map(|(a,b)| a - b).collect();
        stack.v17 = stack.v26.clone();
        stack.v27 = stack.v13.iter().skip(stack.v26[0] as usize).take(stack.v11[0] as usize).copied().collect();
        if stack.v27 == stack.v4 {
            stack.v14 = stack.v14.iter().zip(stack.v11.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
            continue;
        }
        stack.v28 = stack.v13.iter().skip(stack.v26[0] as usize).take(stack.v11[0] as usize).copied().collect();
        if stack.v28 == stack.v2 {
            stack.v14 = stack.v14.iter().zip(stack.v11.iter().cycle()).map(|(a,b)| a - b).collect();
            continue;
        }
        stack.v29 = stack.v13.iter().skip(stack.v26[0] as usize).take(stack.v11[0] as usize).copied().collect();
        if stack.v29 == stack.v10 {
            stack.v16 = stack.v16.iter().zip(stack.v11.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
            continue;
        }
        stack.v30 = stack.v13.iter().skip(stack.v26[0] as usize).take(stack.v11[0] as usize).copied().collect();
        if stack.v30 == stack.v3 {
            stack.v15 = stack.v15.iter().zip(stack.v11.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
            continue;
        }
        stack.v31 = stack.v13.iter().skip(stack.v26[0] as usize).take(stack.v11[0] as usize).copied().collect();
        if stack.v31 == stack.v12 {
            stack.v15 = stack.v15.iter().zip(stack.v11.iter().cycle()).map(|(a,b)| (*a).wrapping_add(*b)).collect();
            continue;
        }
        stack.v32 = stack.v13.iter().skip(stack.v26[0] as usize).take(stack.v11[0] as usize).copied().collect();
        if stack.v32 == stack.v0 {
            stack.v16 = stack.v16.iter().zip(stack.v11.iter().cycle()).map(|(a,b)| a - b).collect();
            continue;
        }
    }

}
