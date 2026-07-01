# Solution notes: Run-length encode

## Approach

Iterates through each character in the input string.
If the last tuple in the result vector has the same character, it increments the count.
Otherwise, it pushes a new (char, 1) tuple.
Returns an empty Vec for empty input automatica
## Edge cases handled


## Anything special

_Tricks, alternatives you considered, performance notes, etc._
