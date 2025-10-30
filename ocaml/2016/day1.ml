open Base
open Stdio

type turn_direction =
  | Left
  | Right

type direction =
  | East
  | South
  | West
  | North

module Int_pair = struct
  module T = struct
    type t = int * int [@@deriving compare, hash, sexp_of]
  end

  include T
  include Comparator.Make (T)
end

type state =
  { x : int
  ; y : int
  ; dir : direction
  ; past_coordinates : Set.M(Int_pair).t
  ; first_revisited_coordinate : (int * int) option
  }

let step dist state =
  match state.dir with
  | East -> state.x + dist, state.y
  | South -> state.x, state.y - dist
  | West -> state.x - dist, state.y
  | North -> state.x, state.y + dist
;;

let rec walk dist state =
  match dist with
  | 0 -> state
  | _ ->
    let ((x, y) as next_coord) = step 1 state in
    let first_revisited_coordinate =
      match state.first_revisited_coordinate with
      | Some _ as coord -> coord
      | None ->
        if Set.mem state.past_coordinates next_coord then Some next_coord else None
    in
    let past_coordinates = Set.add state.past_coordinates next_coord in
    walk (dist - 1) { state with x; y; past_coordinates; first_revisited_coordinate }
;;

let turn lr state =
  let dir =
    match lr, state.dir with
    | Left, East -> North
    | Left, South -> East
    | Left, West -> South
    | Left, North -> West
    | Right, East -> South
    | Right, South -> West
    | Right, West -> North
    | Right, North -> East
  in
  { state with dir }
;;

let l1_dist (x, y) = Int.(abs x + abs y)

let solve fname =
  let inputs =
    In_channel.read_all fname |> String.split ~on:',' |> List.map ~f:String.strip
  in
  let parse_turn = function
    | "L" -> Left
    | "R" -> Right
    | x -> failwith @@ Printf.sprintf "Failed to parse turn: %s" x
  in
  let dirs =
    List.map inputs ~f:(fun s ->
      String.(prefix s 1 |> parse_turn, drop_prefix s 1 |> Int.of_string))
  in
  let follow state (lr, dist) = state |> turn lr |> walk dist in
  let init_state =
    { x = 0
    ; y = 0
    ; dir = North
    ; past_coordinates = Set.empty (module Int_pair)
    ; first_revisited_coordinate = None
    }
  in
  let final_state = List.fold dirs ~init:init_state ~f:follow in
  let final_coord_dist = l1_dist (final_state.x, final_state.y) in
  let first_revisited_coord_dist =
    Option.map final_state.first_revisited_coordinate ~f:l1_dist
  in
  assert (final_coord_dist = 250);
  assert (Option.equal Int.equal first_revisited_coord_dist (Some 151))
;;
