open Base
open Stdio

type instr =
  | Rect of int * int
  | Rotate_row of int * int
  | Rotate_col of int * int

let parse_instruction s =
  let open Angstrom in
  let ( let* ) = ( >>= ) in
  let int = take_while1 Char.is_digit >>= fun s -> Int.of_string s |> return in
  let rect =
    let* width = string "rect " *> int in
    let* height = char 'x' *> int in
    return @@ Rect (width, height)
  in
  let rotate_row =
    let* row = string "rotate row y=" *> int in
    let* amount = string " by " *> int in
    return @@ Rotate_row (row, amount)
  in
  let rotate_col =
    let* col = string "rotate column x=" *> int in
    let* amount = string " by " *> int in
    return @@ Rotate_col (col, amount)
  in
  let parser = rect <|> rotate_row <|> rotate_col in
  parse_string ~consume:Consume.All parser s |> Result.ok_or_failwith
;;

module Screen = struct
  type t =
    { width : int
    ; height : int
    ; pixels : int Array.t
    }

  let create width height =
    { width; height; pixels = Array.create ~len:(width * height) 0 }
  ;;

  let index s x y =
    assert (x <= s.width && y <= s.height);
    (y * s.width) + x
  ;;

  let display s =
    let open Out_channel in
    for y = 0 to s.height - 1 do
      for x = 0 to s.width - 1 do
        let idx = index s x y in
        output_char stdout @@ if s.pixels.(idx) > 0 then '#' else ' '
      done;
      output_char stdout '\n'
    done
  ;;

  let rect s width height =
    for x = 0 to width - 1 do
      for y = 0 to height - 1 do
        let idx = index s x y in
        s.pixels.(idx) <- 1
      done
    done
  ;;

  let _rotate s row_or_col amount size index' =
    assert (amount < size);
    let prefix_n = size - amount in
    let tmp = Array.init prefix_n ~f:(fun i -> s.pixels.(index' s row_or_col i)) in
    for i = 0 to amount - 1 do
      let src = index' s row_or_col (prefix_n + i) in
      let dst = index' s row_or_col i in
      s.pixels.(dst) <- s.pixels.(src)
    done;
    for i = 0 to prefix_n - 1 do
      let dst = index' s row_or_col (amount + i) in
      s.pixels.(dst) <- tmp.(i)
    done
  ;;

  let rotate_row s row amount =
    _rotate s row amount s.width (fun s row i -> index s i row)
  ;;

  let rotate_col s col amount =
    _rotate s col amount s.height (fun s col i -> index s col i)
  ;;
end

let solve fname =
  let inputs = In_channel.read_lines fname in
  let is = List.map inputs ~f:parse_instruction in
  let exec s = function
    | Rect (w, h) -> Screen.rect s w h
    | Rotate_row (r, x) -> Screen.rotate_row s r x
    | Rotate_col (c, x) -> Screen.rotate_col s c x
  in
  let screen = Screen.create 50 6 in
  List.iter is ~f:(fun i -> exec screen i);
  let n_lit = Array.fold screen.pixels ~init:0 ~f:( + ) in
  assert (n_lit = 128);
  print_endline " Expected: EOARGPHYAO";
  Screen.display screen
;;
