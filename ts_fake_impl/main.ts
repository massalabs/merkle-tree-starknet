import { init_runner, ITestEnv, Key, Value } from "../ts_common/deno/main.ts";

class TestEnv implements ITestEnv {
    // some fake implementation
    // real test must import the real implementation
    count: number = 1;

    constructor() {
        // intialize the fake implementation
        // creating tree, storage, etc.
    }

    insert(k: Key, v: Value) {
        console.log(`    insert impl ` + this.count);

        // fake action
        this.count++;
    }

    remove(k: Key) {
        console.log(`    remove impl`);

        // fake action
        this.count++;
    }

    get(k: Key): Value {
        console.log(`    get impl`);

        // fake action
        this.count++;
        // get the value for the key, return it
        return new Uint8Array(0);
    }

    contains(k: Key): boolean {
        console.log(`    contains impl`);

        // fake action
        this.count++;
        // check if the key exists, return true if it does
        return true;
    }

    check_root_hash(): Value {
        console.log(`    check_root_hash impl`);

        // fake action
        this.count++;
        // return the root hash
        return new Uint8Array(0);
    }
}


init_runner(() => {
    return new TestEnv();
});
