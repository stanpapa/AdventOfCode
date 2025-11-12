import gleam/int
import gleam/io

import argv
import clip
import clip/help

import cli
import day1

pub fn main() -> Nil {
  // parse CLI options
  let result =
    cli.command()
    |> clip.help(help.simple("person", "Create a person"))
    |> clip.run(argv.load().arguments)

  // run specific day
  case result {
    Error(e) -> io.println_error(e)
    Ok(day) ->
      case day {
        1 -> day1.main()
        other ->
          case other < 1 || other > 24 {
            False ->
              io.println(
                "Day " <> int.to_string(other) <> " is not yet implemented",
              )
            True ->
              io.println_error(
                "Day " <> int.to_string(other) <> " is not part of AoC",
              )
          }
      }
  }
}
