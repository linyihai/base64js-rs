import test from 'ava'

import { toByteArray, fromByteArray, byteLength } from '../index'
import base64 from 'base64-js'

test('compare base64-js and this library', (t) => {
  let s = "SGVsbG8gV29ybGQh";
  let tbuf = base64.toByteArray(s);
  let buf = toByteArray(s);
  t.deepEqual(buf, tbuf);
  const decoder = new TextDecoder('utf-8');
  const decodedString = decoder.decode(buf);
  t.is(decodedString, "Hello World!");

  let ds = fromByteArray(buf);
  let ts = base64.fromByteArray(tbuf);
  t.is(ds, ts)
})

test('test byteLength', (t) => {
  let s = "SGVsbG8gV29ybGQh";
  let js_len = base64.byteLength(s);
  let rs_len = byteLength(s);
  t.is(js_len, rs_len);
  t.is(rs_len, 12);
})