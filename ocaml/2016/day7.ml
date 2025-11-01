open Base
open Stdio

type addr_seg =
  | Bracketed of string
  | Unbracketed of string

let parse_addr str =
  let open Angstrom in
  let ( let* ) = ( >>= ) in
  let seq = take_while1 Char.is_lowercase in
  let bracketed =
    let* s = char '[' *> seq <* char ']' in
    return (Bracketed s)
  in
  let unbracketed =
    let* s = seq in
    return (Unbracketed s)
  in
  let parser = many1 (bracketed <|> unbracketed) in
  parse_string ~consume:Consume.All parser str |> Result.ok_or_failwith
;;

let support_tls addr =
  let rec has_abba = function
    | [] -> false
    | a :: b :: c :: d :: _ when Char.(a = d && b = c && not (a = b)) -> true
    | _ :: tl -> has_abba tl
  in
  let rec check found = function
    | [] -> found
    | Bracketed s :: tl -> if String.to_list s |> has_abba then false else check found tl
    | Unbracketed s :: tl ->
      let found = found || String.to_list s |> has_abba in
      check found tl
  in
  check false addr
;;

let support_ssl addr =
  let module S = Set.Poly in
  let rec find_aba set = function
    | [] -> set
    | a :: (b :: c :: _ as tl) when Char.(a = c && not (a = b)) ->
      find_aba (S.add set (a, b)) tl
    | _ :: tl -> find_aba set tl
  in
  let rec collect aba bab = function
    | [] -> aba, bab
    | Bracketed s :: tl -> collect aba (find_aba bab (String.to_list s)) tl
    | Unbracketed s :: tl -> collect (find_aba aba (String.to_list s)) bab tl
  in
  let check (aba, bab) =
    let f acc (a, b) = acc || S.mem bab (b, a) in
    S.fold aba ~init:false ~f
  in
  collect S.empty S.empty addr |> check
;;

let solve fname =
  let inputs = In_channel.read_lines fname in
  let addrs = List.map inputs ~f:parse_addr in
  let tls_addrs = List.filter addrs ~f:support_tls in
  let ssl_addrs = List.filter addrs ~f:support_ssl in
  assert (List.length tls_addrs = 110);
  assert (List.length ssl_addrs = 242)
;;
