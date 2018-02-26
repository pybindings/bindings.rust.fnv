# coding: utf-8
from __future__ import unicode_literals

import sys
import unittest

from bindings.rust import fnv


class TestFnvHasher(unittest.TestCase):

    def test_update(self):
        hasher = fnv.fnv()
        hasher.update(b'abc')
        hasher.update(bytearray(b'def'))

        self.assertRaises(TypeError, hasher.update, 1)
        self.assertRaises(TypeError, hasher.update, "hello")

        hasher2 = fnv.fnv()
        hasher2.update(b'abcdef')

        self.assertEqual(hasher.digest(), hasher.digest())
        self.assertEqual(hasher.hexdigest(), hasher.hexdigest())
        self.assertEqual(hasher.finish(), hasher.finish())

    def test_hexdigest(self):
        hasher = fnv.fnv()
        hasher.update(b'abcdef')
        self.assertEqual("{:x}".format(hasher.finish()), hasher.hexdigest())

    def test_digest(self):
        hasher = fnv.fnv()
        hasher.update(b'abcdef')

        h = hasher.finish()
        d = []
        while h:
            d.append(h & 0xFF)
            h = h >> 8

        if sys.byteorder != "little":
            d.reverse()

        self.assertEqual(bytearray(d), hasher.digest())
