import gleam/int
import gleam/io
import gleam/list
import gleam/pair
import gleam/result
import gleam/set
import gleam/string
import simplifile

fn part_1(left: List(Int), right: List(Int)) -> Int {
  // compute distance between ids
  left
  |> list.zip(right)
  |> list.fold(0, fn(acc, x) {
    acc + int.absolute_value(pair.first(x) - pair.second(x))
  })
}

fn part_2(left: List(Int), right: List(Int)) -> Int {
  left
  |> set.from_list
  |> set.to_list
  |> list.map(fn(id) { #(id, list.count(right, fn(x) { x == id })) })
  |> list.fold(0, fn(acc, x) { acc + pair.first(x) * pair.second(x) })
}

pub fn main() {
  // parse ids
  let #(left, right) =
    simplifile.read("../input/day1.txt")
    |> result.unwrap("")
    |> string.trim_end
    |> string.split("\n")
    |> list.map(fn(pair: String) {
      pair |> string.split_once("   ") |> result.unwrap(#("", ""))
    })
    |> list.unzip

  // sort lists
  let left =
    left
    |> list.try_map(fn(id) { int.parse(id) })
    |> result.unwrap([])
    |> list.sort(int.compare)
  let right =
    right
    |> list.try_map(fn(id) { int.parse(id) })
    |> result.unwrap([])
    |> list.sort(int.compare)

  // print solutions
  io.println("Part 1: " <> part_1(left, right) |> int.to_string)
  io.println("Part 2: " <> part_2(left, right) |> int.to_string)
}
