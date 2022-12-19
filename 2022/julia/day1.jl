f = open(ARGS[1], "r")
input = read(f, String)
close(f)

calories = Vector{Int64}()
for line in split(input, "\n\n")
    push!(calories, sum([parse(Int64, ss) for ss in split(line, "\n")]))
end

println("part 1: $(maximum(calories))")
println("part 2: $(sum(last(sort!(calories), 3)))")