(*
Compiled for CVM bytecode
CVM is an open-source low level assembly with targets such as JIT, Native, C, Python, JS, Rust
This python code as been auto-generated

CVM 2020 - 2022- All rights reserved 
CVM is a Laurent Gaucheron software

CVM 2.0.3.1745
*)

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
(*%%DEFS%%*)

  let rec e line = 
    match line with
(*%%CODE%%*)
    | _ -> exit 0;
      ; e (line + 1)
  in e 0;;
main ();;