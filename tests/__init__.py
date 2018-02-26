# coding: utf-8
from __future__ import absolute_import

import os
import sys

build_dir = os.path.abspath(os.path.join(__file__, '..', 'build', 'lib'))
sys.path.insert(0, build_dir)

from . import test_fnv
