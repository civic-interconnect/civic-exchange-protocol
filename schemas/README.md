# Schemas

## GitHub Raw URLs 

All CEP schemas are intended to be resolved by their $id from GitHub raw URLs; examples of $ref assume that hosting pattern.

## Timestamp Requirements

- Timestamp regex is strict. Required: YYYY-MM-DDTHH:MM:SS.microsecondsZ exactly.
- Many systems will only give milliseconds or whole seconds.
- Round / pad when you only have millisecond or second precision.
- Implementers MUST canonicalize to exactly 6 digits (pad with zeros or round) before attestation.

## Resolution Confidence

If score < 1.0, you SHOULD include methodUri and sourceRecordCount.