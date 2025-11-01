open Base
open Stdio

let count_if_valid x y z =
  let max = Int.(max (max x y) z) in
  if max < x + y + z - max then 1 else 0
;;

let apply_to_sides f = function
  | [ x; y; z ] -> f x y z
  | _ -> failwith "Expect exactly three side lengths"
;;

let count_valid1 triangles =
  List.fold triangles ~init:0 ~f:(fun acc t -> acc + apply_to_sides count_if_valid t)
;;

let count_valid2 triangles =
  let rec transpose = function
    | [] | [] :: _ -> []
    | rows -> List.map ~f:List.hd_exn rows :: transpose (List.map ~f:List.tl_exn rows)
  in
  let rec aux cnt = function
    | [] -> cnt
    | t1 :: t2 :: t3 :: ts ->
      let ts' = transpose [ t1; t2; t3 ] in
      let cnt' =
        List.fold ts' ~init:0 ~f:(fun acc t -> acc + apply_to_sides count_if_valid t)
      in
      aux (cnt + cnt') ts
    | _ -> failwith "Expect the number of elements to be a multiple of 3"
  in
  aux 0 triangles
;;

let solve fname =
  let lines = In_channel.read_lines fname in
  let parse_line l =
    String.split l ~on:' '
    |> List.filter_map ~f:(fun x -> String.strip x |> Int.of_string_opt)
  in
  let triangles = List.map lines ~f:parse_line in
  assert (count_valid1 triangles = 982);
  assert (count_valid2 triangles = 1826);
;;
