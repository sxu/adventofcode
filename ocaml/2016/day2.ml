open Base
open Stdio

let swap (x, y) = y, x

let keypad1_alist =
  [ '1', (-1, 1)
  ; '2', (0, 1)
  ; '3', (1, 1)
  ; '4', (-1, 0)
  ; '5', (0, 0)
  ; '6', (1, 0)
  ; '7', (-1, -1)
  ; '8', (0, -1)
  ; '9', (1, -1)
  ]
;;

let keybad1_map = Map.Poly.of_alist_exn keypad1_alist
let keypad1_map_rev = Map.Poly.of_alist_exn (List.map keypad1_alist ~f:swap)

let keypad2_alist =
  [ '1', (0, 2)
  ; '2', (-1, 1)
  ; '3', (0, 1)
  ; '4', (1, 1)
  ; '5', (-2, 0)
  ; '6', (-1, 0)
  ; '7', (0, 0)
  ; '8', (1, 0)
  ; '9', (2, 0)
  ; 'A', (-1, -1)
  ; 'B', (0, -1)
  ; 'C', (1, -1)
  ; 'D', (0, -2)
  ]
;;

let keybad2_map = Map.Poly.of_alist_exn keypad2_alist
let keypad2_map_rev = Map.Poly.of_alist_exn (List.map keypad2_alist ~f:swap)

let move map map_rev cur dir =
  let x, y = Map.Poly.find_exn map cur in
  let new_pos =
    match dir with
    | 'U' -> x, y + 1
    | 'D' -> x, y - 1
    | 'L' -> x - 1, y
    | 'R' -> x + 1, y
    | _ -> failwith (Printf.sprintf "Unexpected instruction: %c" dir)
  in
  match Map.Poly.find map_rev new_pos with
  | Some next -> next
  | None -> cur
;;

let follow move num dirs = String.fold dirs ~init:num ~f:move

let solve fname =
  let instrs = In_channel.read_lines fname in
  let move_on_keypad1 = move keybad1_map keypad1_map_rev in
  let f (num, code) dirs =
    let c = follow move_on_keypad1 num dirs in
    c, code ^ String.make 1 c
  in
  let _, code1 = List.fold instrs ~init:('5', "") ~f in
  let move_on_keypad2 = move keybad2_map keypad2_map_rev in
  let f (num, code) dirs =
    let c = follow move_on_keypad2 num dirs in
    c, code ^ String.make 1 c
  in
  let _, code2 = List.fold instrs ~init:('5', "") ~f in
  assert (String.equal code1 "56855");
  assert (String.equal code2 "B3C27")
;;
