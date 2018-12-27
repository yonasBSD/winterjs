// Copyright (c) 2012 Ecma International.  All rights reserved.
// This code is governed by the BSD license found in the LICENSE file.

/*---
esid: sec-array.prototype.lastindexof
es5id: 15.4.4.15-2-18
description: >
    Array.prototype.lastIndexOf applied to String object which
    implements its own property get method
---*/

        var str = new String("012");

            String.prototype[3] = "3";

assert.sameValue(Array.prototype.lastIndexOf.call(str, "2"), 2, 'Array.prototype.lastIndexOf.call(str, "2")');
assert.sameValue(Array.prototype.lastIndexOf.call(str, "3"), -1, 'Array.prototype.lastIndexOf.call(str, "3")');

reportCompare(0, 0);