open Base
open Stdio

type room =
  { name : string
  ; sector_id : int
  ; checksum : string
  }

let parse_room str =
  let open Angstrom in
  let names = sep_by (char '-') (take_while1 Char.is_lowercase) in
  let sector_id = take_while1 Char.is_digit in
  let checksum = char '[' *> take_while1 Char.is_lowercase <* char ']' in
  let ( let* ) = ( >>= ) in
  let parser =
    let* names = names in
    let* _ = char '-' in
    let* sector_id = sector_id in
    let* checksum = checksum in
    return
      { name = String.concat ~sep:" " names
      ; sector_id = Int.of_string sector_id
      ; checksum
      }
  in
  parse_string ~consume:Consume.All parser str |> Result.ok_or_failwith
;;

let count_letters cnts str =
  let inc cnts' c = Map.Poly.update cnts' c ~f:(fun x -> Option.value x ~default:0 + 1) in
  let f cnts' c = if Char.is_lowercase c then inc cnts' c else cnts' in
  String.fold str ~init:cnts ~f
;;

let compute_checksum name =
  let cnts = count_letters Map.Poly.empty name in
  let compare (c1, cnt1) (c2, cnt2) =
    match Int.compare cnt2 cnt1 with
    | 0 -> Char.compare c1 c2
    | x -> x
  in
  let sorted = List.sort (Map.Poly.to_alist cnts) ~compare in
  List.take sorted 5 |> List.map ~f:(fun (c, _) -> c) |> String.of_list
;;

let is_real room =
  let checksum = compute_checksum room.name in
  String.equal room.checksum checksum
;;

let a_ascii = Char.to_int 'a'

let decipher_room r =
  let decipher_letter = function
    | ' ' -> ' '
    | c ->
      let ascii = Char.to_int c in
      ((ascii - a_ascii + r.sector_id) % 26) + a_ascii |> Char.of_int_exn
  in
  let deciperhed_name = String.map r.name ~f:decipher_letter in
  { r with name = deciperhed_name }
;;

let solve fname =
  let lines = In_channel.read_lines fname in
  let real_rooms = List.map lines ~f:parse_room |> List.filter ~f:is_real in
  let sector_id_sum = List.fold real_rooms ~init:0 ~f:(fun acc r -> acc + r.sector_id) in
  let decipered_rooms = List.map real_rooms ~f:decipher_room in
  let target_room =
    List.find_exn decipered_rooms ~f:(fun r ->
      String.equal r.name "northpole object storage")
  in
  assert (sector_id_sum = 158835);
  assert (target_room.sector_id = 993)
;;
