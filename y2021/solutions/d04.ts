import { Day } from '../Day';

interface BingoCard {
  numbers: number[];
}

export default class Day4 extends Day {
  private _input = this.input()
    .split('\n\n');

  private _numbers = this._input[0].split(',').map((x) => parseInt(x, 10));

  private _cardLength = 5;

  private _cards = this._input.slice(1)
    .map((l) => l.split('\n')
      .filter((li) => li.length > 0)
      .map((li) => li.trim().split(' ')
        .filter((lii) => lii.length > 0)
        .map((lii) => parseInt(lii, 10)))
      .flat() as number[])
    .map((l) => ({ numbers: l } as BingoCard));

  public partOne(): string {
    const bingo = this.findBingo();
    return this.score(bingo.card, bingo.drawnNumbers).toString();
  }

  public partTwo(): string {
    const bingo = this.findLastBingo();
    return this.score(bingo.card, bingo.drawnNumbers).toString();
  }

  private score(card: BingoCard, drawnNumbers: number[]): number {
    const unmarkedNumbers = card.numbers.filter((x) => drawnNumbers.findIndex((y) => y === x) === -1);

    // sum of unmarked numbers
    return (unmarkedNumbers.reduce((acc, cur) => acc + cur, 0) * drawnNumbers[drawnNumbers.length - 1]);
  }

  private findBingo(): { drawnNumbers: number[], card: BingoCard } {
    const drawnNumbers: number[] = [];

    for (let i = 0; i < this._numbers.length; i += 1) {
      drawnNumbers.push(this._numbers[i]);
      const isBingo = this._cards.map((card) => this.checkRow(card, drawnNumbers) || this.checkColumn(card, drawnNumbers));

      if (isBingo.some((x) => x)) {
        return { drawnNumbers, card: this._cards[isBingo.findIndex((x) => x)] };
      }
    }

    return undefined;
  }

  private findLastBingo(): { drawnNumbers: number[], card: BingoCard } {
    const drawnNumbers: number[] = [];
    const bingos: { drawnNumbers: number[], card: BingoCard }[] = [];
    const remainingCards = this._cards.slice();

    for (let i = 0; i < this._numbers.length; i += 1) {
      drawnNumbers.push(this._numbers[i]);
      remainingCards.forEach((card) => {
        if (this.checkRow(card, drawnNumbers) || this.checkColumn(card, drawnNumbers)) {
          bingos.push({ drawnNumbers: drawnNumbers.slice(), card });
          remainingCards.splice(remainingCards.indexOf(card), 1);
        }
      });

      if (remainingCards.length === 0) {
        break;
      }
    }

    return bingos[bingos.length - 1];
  }

  private getRows(arr: number[], size: number): number[][] {
    const chunks: number[][] = [];
    for (let i = 0; i < arr.length; i += size) {
      chunks.push(arr.slice(i, i + size));
    }
    return chunks;
  }

  private getColumns(arr: number[], rowLength: number): number[][] {
    const cols: number[][] = [];
    for (let i = 0; i < arr.length; i += 1) {
      cols[i % rowLength] = [...(cols[i % rowLength] ?? []), arr[i]];
    }
    return cols;
  }

  private checkRow(card: BingoCard, drawnNumbers: number[]): boolean {
    return this.getRows(card.numbers, this._cardLength)
      .map((row) => (row.every((x) => drawnNumbers.includes(x)))).some((x) => x);
  }

  private checkColumn(card: BingoCard, drawnNumbers: number[]): boolean {
    return this.getColumns(card.numbers, this._cardLength)
      .map((row) => (row.every((x) => drawnNumbers.includes(x)))).some((x) => x);
  }
}
