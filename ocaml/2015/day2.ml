open Base
open Stdio
module L = List

let solve fname =
  let lines = In_channel.read_lines fname in
  let dims = L.map lines ~f:(fun s -> String.split s ~on:'x' |> L.map ~f:Int.of_string) in
  let get_wrapper_area = function
    | [ l; w; h ] ->
      let a1 = l * w in
      let a2 = w * h in
      let a3 = l * h in
      let smallest_side = Int.min a1 a2 |> Int.min a3 in
      ((a1 + a2 + a3) * 2) + smallest_side
    | _ -> failwith "Malformed input"
  in
  let area = L.map dims ~f:get_wrapper_area |> L.fold ~init:0 ~f:( + ) in
  let get_ribbon_len = function
    | [ l; w; h ] ->
      let s1 = l + w in
      let s2 = w + h in
      let s3 = l + h in
      let smallest_side = (Int.min s1 s2 |> Int.min s3) * 2 in
      smallest_side + (l * w * h)
    | _ -> failwith "Malformed input"
  in
  let len = L.map dims ~f:get_ribbon_len |> L.fold ~init:0 ~f:( + ) in
  assert (area = 1588178);
  assert (len = 3783758)
;;
