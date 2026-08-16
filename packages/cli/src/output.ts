// packages/cli/src/output.ts
import type { PublishResult } from './types.js';

export function printPublishResult(result: PublishResult, jsonOutput: boolean): void {
  if (jsonOutput) {
    process.stdout.write(`${JSON.stringify(result)}\n`);
    return;
  }
  process.stdout.write(`Published ${result.name}@${result.version}\n`);
  process.stdout.write(`Tarball: ${result.tarball}\n`);
  process.stdout.write(`SHA-256: ${result.sha256}\n`);
}

export function printError(error: unknown): void {
  const message = error instanceof Error ? error.message : String(error);
  process.stderr.write(`fpm: ${message}\n`);
}
