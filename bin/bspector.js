#!/usr/bin/env node
const { spawnSync } = require('node:child_process');
const path = require('node:path');
const os = require('node:os');
const fs = require('node:fs');
const candidates = [
  path.join(os.homedir(), '.cargo', 'bin', 'bspector'),
  '/usr/local/bin/bspector',
  'bspector',
];
let exe = candidates.find((p) => p === 'bspector' || (fs.existsSync(p) && fs.statSync(p).isFile())) || 'bspector';
const result = spawnSync(exe, process.argv.slice(2), { stdio: 'inherit' });
process.exit(result.status ?? 1);
