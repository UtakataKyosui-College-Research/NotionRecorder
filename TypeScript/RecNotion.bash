# bun run index.ts
# bun build index.ts --outdir .
# bun nexe -i ファイル名.js -o ファイル名.exe -t windows-x64-14.15.3 -r .env
bun nexe -i index.js -o RecNote.exe -t linux-x64-14.15.3 --resource ./.env
 