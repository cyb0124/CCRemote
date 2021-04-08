# CCRemote
CCRemote is an ComputerCraft program by cyb0124 for item-storage and auto-crafting.
It is a direct port of the OpenComputers program [OCRemote](https://github.com/cyb0124/OCRemote).
Checkout [OCRemote](https://github.com/cyb0124/OCRemote) for more details.

This project requires either Plethora or generic peripherals (available in CC:Tweaked 1.15+).

### Notes
- CC supports asynchronous peripheral task execution, so load-balancing between multiple computers is unnecessary.
  However, this feature is still ported from OCRemote to allow redundancy.
- There can be a race-condition caused by inventories being modified between calls to `list` and `getItemDetail`.
  This situation will be detected as an error and will be recovered gracefully.
- Because CC's wired network can transport items over any distance, the "bus" inventory is no longer necessary.
  However, it is still ported from OCRemote to allow wireless transportation.
- CraftyProcess may enter invalid state if Internet connection breaks or server exits during its execution.
  This is due to CraftyProcess requiring synchronization between multiple clients.
  All other processes still guarantees state validity when Internet connection breaks or server exits.
