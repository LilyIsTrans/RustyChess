# RustyChess
My attempt at creating a chess program in Rust. I doubt it will ever be the best chess engine out there, 
but I hope I can learn a lot about optimization as well as balancing accuracy with speed for problems like chess
where it is infeasible to compute the exact optimal answer.

# Goals

## Minimal goals
I will consider the project to be minimally complete when (in no particular order):
* The program implements UCI with enough functionality for a standard UCI interface program such as [Cute Chess](https://github.com/cutechess/cutechess) to be used to either allow a human to play against the engine(s), or pit the engine(s) against other UCI engines.
* The program never attempts an illegal move
* At least one of the implemented engines is capable of consistently (>90% of the time) checkmating an opponent which selects it's moves randomly using a uniform distribution over all legal moves
* At least one of the implemented engines is capable of regularly (>50% of the time) checkmating me (I consider myself a decent approximation of an average chess player. I can handily defeat naive engines and inexperienced players, but struggle against peers and consistently lose to anyone who's actually studied)
* At least one engine satisfying both the above requirements can do so while taking less than 2 seconds to compute it's moves for typical positions 

## Potential extended goals
In no particular order:
* My own UCI compatible interface
* Multiple configurable difficulty levels
