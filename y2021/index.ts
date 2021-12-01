import { parse } from 'ts-command-line-args';
import { Part } from './Day';

interface AdventOfCodeArguments {
  day: number;
  part: number;
  help?: boolean;
}

const args = parse<AdventOfCodeArguments>(
  {
    day: {
      type: Number,
      optional: true,
      alias: 'd',
      description: 'Day between 1 and 25',
    },
    part: {
      type: Number,
      optional: true,
      alias: 'p',
      description: 'Part 1 or 2',
    },
    help: {
      type: Boolean,
      optional: true,
      alias: 'h',
      description: 'Prints this usage guide',
    },
  },
  {
    helpArg: 'help',
    headerContentSections: [{ header: 'Advent of Code', content: 'This project contains Matthias\' solutions to 2021\'s version of Advent of Code. Thank you for inspecting these solutions.\n\nExample usage: index.js -d 1 -p 1' }],
    footerContentSections: [{ header: 'Footer', content: 'Copyright: Matthias Aarnoutse' }],
  },
);

function validateArguments(): boolean {
  if (args.help) return true;

  if (args.day < 1 || args.day > 25) {
    throw new Error('Invalid day, must be between 1 and 25. Example: -d 1');
  }

  if (args.part !== 1 && args.part !== 2) {
    throw new Error('Invalid part, must be 1 or 2. Example: -p 1');
  }

  return true;
}

async function main() {
  if (validateArguments() && !args.help) {
    const { day } = args;
    const { part } = args;

    const name = `d${day.toString().padStart(2, '0')}`;
    const { default: Solution } = await import(`./solutions/${name}`);
    const result = new Solution(name).run(part === 1 ? Part.ONE : Part.TWO);

    console.log(result);
  }
}

(async () => {
  try {
    await main();
  } catch (e) {
    console.error(e);
  }
})();
