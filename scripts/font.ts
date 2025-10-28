import { Font } from 'fonteditor-core';
import { promises as fs } from 'node:fs';
import path from 'node:path';

const chars = new Set<number>(
  '0123456789:休息中，后解锁即将紧急退出休息结束，移动点击鼠标解锁~'
    .split('')
    .map((s) => s.charCodeAt(0)),
);

'BEEMO'.split('').forEach((c) => {
  chars.add(c.charCodeAt(0));
});

const inputFontPath = path.resolve(__dirname, './OPPO Sans 4.0.ttf');
const outputFontPath = path.resolve(__dirname, '../src-tauri/src/opposans.ttf');

const font = Font.create(await fs.readFile(inputFontPath), {
  type: 'ttf',
  subset: [...chars],
});

// 创建子集
const subsetBuffer = font.write({
  type: 'ttf', // 输出为 TTF 文件
  toBuffer: true,
});
await fs.writeFile(outputFontPath, new Uint8Array(subsetBuffer));

console.log(`生成的新字体已保存到: ${outputFontPath}`);
