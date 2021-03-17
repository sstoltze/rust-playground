#!/usr/bin/fish

for n in (cat greedy.task | jq '.people | keys | .[]')
    echo (cat greedy.task | jq ".people.$n") > (string trim -c '"' $n).json
end
