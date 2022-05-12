let v n = if n < 0 then n + 256 else n mod 256;;

let rec reg_update a b fn i = 
  if i = List.length a then
    []
  else
    v (fn (List.nth a i) ((i mod (List.length b)) |> List.nth b)) :: reg_update a b fn (i+1);;

let rec init i fn = 
  match i with 
  | 0 -> []
  | x -> init (x-1) fn @ [x - 1 |> fn];;

let filteri p l =
  let rec aux i acc = function
  | [] -> List.rev acc
  | x::l -> aux (i + 1) (if p i x then x::acc else acc) l
  in
  aux 0 [] l;;

let main () = 
  let r7 = ref [] in
  let r23 = ref [] in
  let r32 = ref [] in
  let r27 = ref [] in
  let r13 = ref [] in
  let r5 = ref [] in
  let r6 = ref [] in
  let r17 = ref [] in
  let r25 = ref [] in
  let r16 = ref [] in
  let r15 = ref [] in
  let r18 = ref [] in
  let r2 = ref [] in
  let r31 = ref [] in
  let r24 = ref [] in
  let r14 = ref [] in
  let r20 = ref [] in
  let r21 = ref [] in
  let r9 = ref [] in
  let r11 = ref [] in
  let r12 = ref [] in
  let r8 = ref [] in
  let r1 = ref [] in
  let r10 = ref [] in
  let r26 = ref [] in
  let r3 = ref [] in
  let r30 = ref [] in
  let r0 = ref [] in
  let r4 = ref [] in
  let r22 = ref [] in
  let r28 = ref [] in
  let r19 = ref [] in
  let r29 = ref [] in

  let rec e line = 
    match line with
    | 0 -> r0 := [125]
    | 1 -> r1 := [102;97;108;115;101]
    | 2 -> r2 := [41]
    | 3 -> r3 := [91]
    | 4 -> r4 := [40]
    | 5 -> r5 := [80;97;114;101;110;116;104;101;115;105;115;32;111;107;58;32]
    | 6 -> r6 := [10]
    | 7 -> r7 := [116;114;117;101]
    | 8 -> r8 := [0]
    | 9 -> r9 := [69;110;116;101;114;32;121;111;117;114;32;40;41;91;93;123;125;32;99;111;110;116;97;105;110;105;110;103;32;101;120;112;114;101;115;115;105;111;110;58;10]
    | 10 -> r10 := [123]
    | 11 -> r11 := [1]
    | 12 -> r12 := [93]
    | 13 -> print_string (String.init (List.length !r9) (fun n -> (List.nth !r9 n |> Char.chr)))
    | 14 -> r13 := let k = read_line () in init (String.length k) (fun a -> Char.code k.[a])
    | 15 -> r14 := [0]
    | 16 -> r15 := [0]
    | 17 -> r16 := [0]
    | 18 -> r17 := [List.length !r13]
    | 19 -> if r17 = r8 then e 21 else e 20
    | 20 -> e 48
    | 21 -> if r15 = r8 then e 23 else e 22
    | 22 -> e 25
    | 23 -> r18 := [1]
    | 24 -> e 26
    | 25 -> r18 := [0]
    | 26 -> if r16 = r8 then e 28 else e 27
    | 27 -> e 30
    | 28 -> r19 := [1]
    | 29 -> e 31
    | 30 -> r19 := [0]
    | 31 -> if r14 = r8 then e 33 else e 32
    | 32 -> e 35
    | 33 -> r20 := [1]
    | 34 -> e 36
    | 35 -> r20 := [0]
    | 36 -> r21 := reg_update !r20 !r19 ( * ) 0
    | 37 -> r22 := reg_update !r21 !r18 ( * ) 0
    | 38 -> if r22 = r11 then e 40 else e 39
    | 39 -> e 42
    | 40 -> r23 := !r7
    | 41 -> e 44
    | 42 -> r23 := !r1
    | 43 -> e 44
    | 44 -> r24 := !r23 @ !r6
    | 45 -> r25 := !r5 @ !r24
    | 46 -> print_string (String.init (List.length !r25) (fun n -> (List.nth !r25 n |> Char.chr)))
    | 47 -> e (-1)
    | 48 -> r26 := reg_update !r17 !r11 (-) 0
    | 49 -> r17 := !r26
    | 50 -> r27 := filteri (fun i _ -> i >= (List.nth !r26 0) && i < ((List.nth !r26 0) + (List.nth !r11 0))) !r13
    | 51 -> if r27 = r4 then e 53 else e 52
    | 52 -> e 55
    | 53 -> r14 := reg_update !r14 !r11 (+) 0
    | 54 -> e 19
    | 55 -> r28 := filteri (fun i _ -> i >= (List.nth !r26 0) && i < ((List.nth !r26 0) + (List.nth !r11 0))) !r13
    | 56 -> if r28 = r2 then e 58 else e 57
    | 57 -> e 60
    | 58 -> r14 := reg_update !r14 !r11 (-) 0
    | 59 -> e 19
    | 60 -> r29 := filteri (fun i _ -> i >= (List.nth !r26 0) && i < ((List.nth !r26 0) + (List.nth !r11 0))) !r13
    | 61 -> if r29 = r10 then e 63 else e 62
    | 62 -> e 65
    | 63 -> r16 := reg_update !r16 !r11 (+) 0
    | 64 -> e 19
    | 65 -> r30 := filteri (fun i _ -> i >= (List.nth !r26 0) && i < ((List.nth !r26 0) + (List.nth !r11 0))) !r13
    | 66 -> if r30 = r3 then e 68 else e 67
    | 67 -> e 70
    | 68 -> r15 := reg_update !r15 !r11 (+) 0
    | 69 -> e 19
    | 70 -> r31 := filteri (fun i _ -> i >= (List.nth !r26 0) && i < ((List.nth !r26 0) + (List.nth !r11 0))) !r13
    | 71 -> if r31 = r12 then e 73 else e 72
    | 72 -> e 75
    | 73 -> r15 := reg_update !r15 !r11 (+) 0
    | 74 -> e 19
    | 75 -> r32 := filteri (fun i _ -> i >= (List.nth !r26 0) && i < ((List.nth !r26 0) + (List.nth !r11 0))) !r13
    | 76 -> if r32 = r0 then e 78 else e 77
    | 77 -> e 80
    | 78 -> r16 := reg_update !r16 !r11 (-) 0
    | 79 -> e 19
    | 80 -> e 19
    | _ -> exit 0;
      ; e (line + 1)
  in e 0;;
main ();;