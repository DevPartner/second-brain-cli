---
type: "zknotes"
date: 2026-03-05 1772697372.383
tags: [dotnet-gc, memory-management]
---

# Generations, compaction, and finalization

## Source

From: GPT

## Summary

 In .NET, the GC organizes managed objects by age into generations (Gen 0, 1, 2) to collect short-lived objects quickly and long-lived objects less often. Compaction removes fragmentation by moving surviving objects together and updating references, improving memory locality. Finalization is a cleanup step for objects with finalizers, but it is slower and non-deterministic, so it should be used sparingly.

## Key Ideas

- **Generations**: Most objects die young, so Gen 0 is collected frequently; surviving objects are promoted to older generations.
- **Compaction**: After collection, live objects are relocated to close memory gaps, which reduces fragmentation and can improve performance.
- **Finalization**: Objects with finalizers require extra GC work (at least two passes), so prefer `IDisposable`/`using` for deterministic cleanup.

## Related

- [[2026-03-05-factory-pattern]]
