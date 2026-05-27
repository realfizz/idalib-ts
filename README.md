# @idalib/ts

typescript sdk for ida pro. wraps [idalib-rs](https://github.com/idalib-rs/idalib) through [napi-rs](https://napi.rs).

## install

```bash
npm install @idalib/ts
```

## requirements

- node.js >= 16
- ida pro 9.x
- `IDADIR` pointing to your ida install

## usage

```typescript
import { Database } from '@idalib/ts'

const db = await Database.open('/path/to/binary.exe')
const bookmarks = await db.bookmarks.list()
const functions = await db.functions.list()
await db.close()
```

## building from source

```bash
export IDADIR="/path/to/IDA Professional 9.3.app/Contents/MacOS"
npm install
npm run build
```

needs rust toolchain and the ida sdk.

## platforms

| os | arch | package |
|---|---|---|
| macos | arm64 | `@idalib/ts-darwin-arm64` |
| macos | x64 | `@idalib/ts-darwin-x64` |
| linux | x64 (glibc) | `@idalib/ts-linux-x64-gnu` |
| windows | x64 (msvc) | `@idalib/ts-win32-x64-msvc` |

## license

mit
