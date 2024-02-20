#include "test_tree.h"
#include <assert.h>
#include <dlfcn.h>
#include <stdio.h>

// Define a struct that implements the ITestTree
typedef struct {
  ITestTree interface;

  static_assert(false, "Add your implementation here");

} FakeTestTree;

static FakeTestTree *to_impl(ITestTree *self) { return (FakeTestTree *)self; }

// Implement the functions for the FakeTestTree struct
Result fake_test_tree_insert(ITestTree *tree, const Key *key,
                             const Value *value) {
  FakeTestTree *impl = to_impl(tree);

  static_assert(false, "Add your implementation here");

  return OK_RESULT();
}

Result fake_test_tree_remove(ITestTree *tree, const Key *key) {
  FakeTestTree *impl = to_impl(tree);

  static_assert(false, "Add your implementation here");

  return OK_RESULT();
}

Result fake_test_tree_get(ITestTree *tree, const Key *key) {
  FakeTestTree *impl = to_impl(tree);

  static_assert(false, "Add your implementation here");

  Value res = {0, 0};
  return OK_VALUE_RESULT(res);
}

Result fake_test_tree_contains(ITestTree *tree, const Key *key) {
  FakeTestTree *impl = to_impl(tree);

  static_assert(false, "Add your implementation here");

  bool res = true;
  return OK_BOOL_RESULT(res);
}

Result fake_test_tree_commit(ITestTree *tree) {
  FakeTestTree *impl = to_impl(tree);

  static_assert(false, "Add your implementation here");

  return OK_RESULT();
}

Result fake_test_tree_root_hash(ITestTree *tree) {
  FakeTestTree *impl = to_impl(tree);

  static_assert(false, "Add your implementation here");

  Value res = {0, 0};
  return OK_VALUE_RESULT(res);
}

// Create a function that creates a FakeTestTree and conforms to this type:
// typedef ITestTree *(*TestTreeCreator)();
ITestTree *create_fake_test_tree() {

  FakeTestTree *tree = malloc(sizeof(FakeTestTree));
  if (tree == NULL) {
    printf("Error: malloc failed\n");
    exit(1);
  }

  // populate the Interface
  tree->interface.insert = fake_test_tree_insert;
  tree->interface.remove = fake_test_tree_remove;
  tree->interface.get = fake_test_tree_get;
  tree->interface.contains = fake_test_tree_contains;
  tree->interface.commit = fake_test_tree_commit;
  tree->interface.root_hash = fake_test_tree_root_hash;

  static_assert(false, "Add your implementation here");

  return (ITestTree *)tree;
}

int main(int argc, char *argv[]) {
  return init_runner(argc, argv, create_fake_test_tree);
}