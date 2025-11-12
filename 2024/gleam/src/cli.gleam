import clip.{type Command}
import clip/opt.{type Opt}

fn day_opt() -> Opt(Int) {
  opt.new("day") |> opt.int |> opt.help("Which AoC day to run")
}

pub fn command() -> Command(Int) {
  clip.command({
    use day <- clip.parameter

    day
  })
  |> clip.opt(day_opt())
}
