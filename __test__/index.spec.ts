import test from 'ava'

import { getSelectedText } from '../index'

test('sync function from native code', (t) => {
  t.is(typeof getSelectedText() === 'string', true)
})
