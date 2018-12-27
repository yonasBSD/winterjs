// |reftest| skip -- class-fields-private,class-fields-public is not supported
// Copyright (C) 2017 Valerie Young. All rights reserved.
// This code is governed by the BSD license found in the LICENSE file.
/*---
esid: sec-scripts-static-semantics-early-errors
description: Early error when referencing privatename that has not been declared in class.
info: |
  Static Semantics: Early Errors
  Script : ScriptBody
    1. Let names be an empty List.
    2. If Script is parsed directly from PerformEval,
        a. Let env be the running execution context's PrivateNameEnvironment.
        b. Repeat while env is not null,
            i. For each binding named N in env,
                1. If names does not contain N, append N to names.
            ii. Let env be env's outer environment reference.
    3. If AllPrivateNamesValid of ScriptBody with the argument names is false, throw a SyntaxError exception.
features: [class, class-fields-private, class-fields-public]
---*/

var executed = false;

class C {
  y = eval("executed = true; this.#x;")
}

assert.throws(SyntaxError, function() {
  new C();
});

assert.sameValue(executed, false);

reportCompare(0, 0);