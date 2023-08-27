import { readFileSync, writeFileSync } from 'fs';
import { resolve } from 'path';

const packagePath = resolve('rust-wasm', 'pkg', 'package.json');

const pkg = JSON.parse(readFileSync(packagePath, 'utf8'));

pkg.type = 'module';

writeFileSync(packagePath, JSON.stringify(pkg, null, 2));
