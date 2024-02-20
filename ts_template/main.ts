import { init_runner, ITestEnv, Key, Value } from "../ts_common/deno/main.ts";

// implements the ITestEnv interface
class TestEnv implements ITestEnv {
    // wrap your implementation here

    constructor() {
        // intialize the fake implementation
        // creating tree, storage, etc.
    }

    insert(k: Key, v: Value) {
    }

    remove(k: Key) {
    }

    get(k: Key): Value {
    }

    contains(k: Key): boolean {
    }

    check_root_hash(): Value {
    }
}


init_runner(() => {
    return new TestEnv();
});
