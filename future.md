# Future Development

### Trait Refactoring

`AddRole` + `PreassignRole` is not the best desicion. I think that I can find a better solution for it. At least what can be done there is combine the into a single trait and call.

### Macro for role restriction

Macros are a substantial part of Substrate. In my opinion it would be a good idea to write a simple macro if a user has some role from the list. Ideally it should be a procedural macro that you just use on your call.

### Restriction map

In my opinion it would be convenient to have a restriction map that shows which calls are restricted to which roles. It would make the number of questions less. However, it still can be constructed offchain and saved as a document, so it is a low priority.

### Benchmarking
  
Each pallet should be benchmarked to set the correct weights on it. It helps Substrate to build blocks with predictable load for each block and make total throughput better. Also I would update the weigth based on it.

### More tests

Currently there are tests for sunny path, tests for the errors that are produced by pallet and that's it. I would like to add some system tests where we set up a real node and check how it works there.

