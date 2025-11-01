open Base
open Stdio

let valid_hash_seq door_id =
  let module MD5 = Stdlib.Digest.MD5 in
  let hash_with idx = door_id ^ Int.to_string idx |> MD5.string |> MD5.to_hex in
  let rec f idx =
    let hash = hash_with idx in
    if String.is_prefix hash ~prefix:"00000" then Some (hash, idx + 1) else f (idx + 1)
  in
  Sequence.unfold ~init:0 ~f
;;

let search_passwords hash_seq =
  let rec aux pw filled_pos seq =
    let hash, seq = Sequence.next seq |> Option.value_exn in
    let c = String.get hash 5 in
    let is_0_to_7 =
      match c with
      | '0' .. '7' -> true
      | _ -> false
    in
    if is_0_to_7 && not (Set.Poly.mem filled_pos c)
    then (
      let filled_pos = Set.Poly.add filled_pos c in
      let pos = Char.to_int c - Char.to_int '0' in
      pw.(pos) <- String.get hash 6;
      if Set.Poly.length filled_pos = 8 then String.of_array pw else aux pw filled_pos seq)
    else aux pw filled_pos seq
  in
  aux (Array.create ~len:8 ' ') Set.Poly.empty hash_seq
;;

let solve fname =
  let door_id = In_channel.read_all fname |> String.strip in
  let first_8, seq = Sequence.split_n (valid_hash_seq door_id) 8 in
  let pw1 = List.map first_8 ~f:(fun h -> String.get h 5) |> String.of_char_list in
  let pw2 = search_passwords (Sequence.append (Sequence.of_list first_8) seq) in
  assert (String.(pw1 = "4543c154"));
  assert (String.(pw2 = "1050cbbd"))
;;
