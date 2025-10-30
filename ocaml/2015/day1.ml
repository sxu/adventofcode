open Base
open Stdio

let follow_instruction floor = function
  | '(' -> floor + 1
  | ')' -> floor - 1
  | x -> failwith ("Unexpected character: " ^ String.of_char x)
;;

let follow_instruction_until_basement (floor, time) instruction =
  let next_floor = follow_instruction floor instruction in
  let open Base.Continue_or_stop in
  if next_floor < 0 then Stop time else Continue (next_floor, time + 1)
;;

let solve fname =
  let lines = In_channel.read_lines fname in
  let instructions = List.hd_exn lines |> String.to_sequence in
  let final_floor = Sequence.fold instructions ~init:0 ~f:follow_instruction in
  let time_to_basement =
    Sequence.fold_until
      instructions
      ~init:(0, 1)
      ~f:follow_instruction_until_basement
      ~finish:(fun _ -> failwith "Basement not reached")
  in
  assert (final_floor = 138);
  assert (time_to_basement = 1771)
;;
