open Base
open Stdio

let all_days =
  [| Day1.solve; Day2.solve; Day3.solve; Day4.solve; Day5.solve; Day6.solve |]
;;

let exec_day day =
  let day_str = Int.to_string @@ (day + 1) in
  print_string @@ "Day " ^ day_str ^ "..";
  all_days.(day) @@ "../../inputs/2016/day" ^ day_str;
  print_endline " OK"
;;

let () =
  let argv = Sys.get_argv () in
  let chosen_days =
    if Array.length argv > 1
    then [| Int.of_string argv.(1) - 1 |]
    else Array.init (Array.length all_days) ~f:(fun x -> x)
  in
  Array.iter chosen_days ~f:exec_day
;;
