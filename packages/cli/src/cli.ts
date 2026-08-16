#!/usr/bin/env node
// packages/cli/src/cli.ts
import { parseArgs } from 'node:util';
import { configCommand, loginCommand, logoutCommand, publishCommand, whoamiCommand } from './commands.js';
import { printError } from './output.js';

const VERSION = '0.1.0';

function printHelp(): void {
  process.stdout.write(`fpm ${VERSION}\n\nUsage:\n  fpm login [--registry <url>]\n  fpm logout\n  fpm whoami\n  fpm config\n  fpm publish [directory] [--dry-run] [--json] [--registry <url>]\n`);
}

function parseCommandArgs(args: string[]) {
  return parseArgs({
    args,
    options: {
      registry: { type: 'string' },
      'dry-run': { type: 'boolean', default: false },
      json: { type: 'boolean', default: false },
      help: { type: 'boolean', short: 'h', default: false },
      version: { type: 'boolean', short: 'v', default: false },
    },
    allowPositionals: true,
    strict: true,
  });
}

async function run(): Promise<void> {
  const [command = 'help', ...rest] = process.argv.slice(2);
  const parsed = parseCommandArgs(rest);
  if (parsed.values.version) {
    process.stdout.write(`${VERSION}\n`);
    return;
  }
  if (parsed.values.help || command === 'help') {
    printHelp();
    return;
  }
  const registry = parsed.values.registry;
  if (registry !== undefined && typeof registry !== 'string') throw new Error('--registry requires a URL.');

  switch (command) {
    case 'login':
      await loginCommand(registry);
      return;
    case 'logout':
      await logoutCommand();
      return;
    case 'whoami':
      await whoamiCommand();
      return;
    case 'config':
      await configCommand();
      return;
    case 'publish':
      await publishCommand(parsed.positionals[0] ?? '.', {
        dryRun: parsed.values['dry-run'] === true,
        json: parsed.values.json === true,
        ...(registry ? { registry } : {}),
      });
      return;
    default:
      throw new Error(`Unknown command: ${command}. Run \`fpm help\` for usage.`);
  }
}

try {
  await run();
} catch (error) {
  printError(error);
  process.exitCode = 1;
}
