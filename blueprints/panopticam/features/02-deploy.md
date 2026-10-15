# Feature Blueprint: Deploy

> Subsystem `Deploy` covering 1 code slices, 0 API routes, and 0 data models.

## Clean-Room Behavioral Summary (Gemma 4 E2B)

The Deploy subsystem provides a mechanism for targeted resource modification or deployment. The core operation is executed via the `sub` function, which accepts three parameters: a target file system `path`, a string `pattern` used to identify the resource, and a replacement string `repl` containing the new content. This function performs a proprietary, atomic operation to update or inject the content defined by `repl` into the resource specified by `path` matching the `pattern`. The subsystem operates as a side-effect routine, modifying the state of the target resource directly.

To rebuild this subsystem, a developer must implement the logic within a function matching the signature `sub(path: Path, pattern: str, repl: str) -> None`. The implementation must replicate the proprietary deployment routine, ensuring that the input parameters are correctly mapped to the file system operations required to perform the replacement. The interface is strictly defined by these three inputs and a void return, requiring the developer to encapsulate the specific deployment logic within this defined boundary.

## Proprietary Logic Slices

### `sub` (`deploy/bump.py:23-29`)
- **Category:** `business_logic` · **Proprietary Score:** `0.65`
- **Signature:** `def sub(path: Path, pattern: str, repl: str) -> None`
- **Behavior:** Proprietary routine `sub` (6 lines)

