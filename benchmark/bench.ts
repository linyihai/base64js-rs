import { Bench } from 'tinybench'

import { toByteArray, fromByteArray } from '../index'
import base64 from 'base64-js'

import fs from 'fs'

import path from 'path'
import util from 'util'

const TESTDATA = path.join(import.meta.dirname, './testdata')

function run_sample(filename: string) {
  const data_file = path.join(TESTDATA, filename)

  const buffer = fs.readFileSync(data_file)
  const uncompressed_data = new Uint8Array(buffer)
  const compressed_data = base64.fromByteArray(uncompressed_data)

  console.log('result for ', filename)
  const b = new Bench()
  b.add('base64-js compress (fromByteArray)', () => base64.fromByteArray(uncompressed_data))
  b.add('rust-base64 compress (fromByteArray)', () => fromByteArray(uncompressed_data))
  b.add('base64-js decompress (toByteArray)', () => base64.toByteArray(compressed_data))
  b.add('rust-base64 decompress (toByteArray)', () => toByteArray(compressed_data))
  b.runSync()
  console.table(b.table())
}

fs.readdirSync(TESTDATA).forEach(function (filename: string) {
  run_sample(filename)
})
