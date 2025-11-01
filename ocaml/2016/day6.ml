open Base
open Stdio

let solve fname =
  let lines = In_channel.read_lines fname in
  let n = String.length (List.hd_exn lines) in
  let module H = Hashtbl.Poly in
  let update_freqs char_cnt_maps str =
    assert (Array.length char_cnt_maps = String.length str);
    let inc maybe_cnt = Option.value maybe_cnt ~default:0 + 1 in
    String.iteri str ~f:(fun i c -> H.update char_cnt_maps.(i) c ~f:inc)
  in
  let char_cnt_maps = Array.init n ~f:(fun _ -> H.create ()) in
  List.iter lines ~f:(fun l -> update_freqs char_cnt_maps l);
  let char_cnt_alists = Array.map char_cnt_maps ~f:H.to_alist in
  let compare (_, cnt1) (_, cnt2) = Int.compare cnt1 cnt2 in
  let compare_rev (_, cnt1) (_, cnt2) = Int.compare cnt2 cnt1 in
  let least_freq_char alist = List.sort alist ~compare |> List.hd_exn in
  let most_freq_char alist = List.sort alist ~compare:compare_rev |> List.hd_exn in
  let msg1 =
    Array.map char_cnt_alists ~f:most_freq_char
    |> Array.map ~f:(fun (c, _) -> c)
    |> String.of_array
  in
  let msg2 =
    Array.map char_cnt_alists ~f:least_freq_char
    |> Array.map ~f:(fun (c, _) -> c)
    |> String.of_array
  in
  assert (String.(msg1 = "gyvwpxaz"));
  assert (String.(msg2 = "jucfoary"))
;;
