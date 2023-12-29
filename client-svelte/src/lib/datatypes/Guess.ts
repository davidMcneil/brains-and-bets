export class Guess {
   public player: string;
   public guess: number; 

   constructor(player: string, guess: number) {
    this.player = player;
    this.guess = guess;
   }
}

export function compare(a: Guess, b: Guess): number{
    return b.guess - a.guess;
}