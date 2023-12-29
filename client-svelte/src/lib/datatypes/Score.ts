export class Score {
   public player: string;
   public score: number; 

   constructor(player: string, score: number) {
    this.player = player;
    this.score = score;
   }
}

export function compare(a: Score, b: Score): number{
    return b.score - a.score;
}