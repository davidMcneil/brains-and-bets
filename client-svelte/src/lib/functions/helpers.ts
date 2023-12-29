import type { Guess } from "$lib/datatypes/Guess";

export function getClosestGuess(guesses: Array<Guess>, answer: number): Guess | null { // guesses are sorted in decending order
    for (let i = 0; i < guesses.length; i++) {
        if (guesses[i].guess <= answer) {
            return guesses[i];
        }
    }
    return null;
}