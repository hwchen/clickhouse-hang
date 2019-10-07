## First pass - weird behavior even without disconnect/reconnect

Done without logging, so not quite sure about clickhouse behavior

When `pool_max=1`, after the first request, can get handle and start query on next round, but is unable to complete query. Will eventually give a db driver error timeout error, and then the connection is freed up.

server logs look something like:
1st query:
```
2019.10.07 13:25:41.701123 [ 25 ] {} <Trace> TCPHandlerFactory: TCP Request. Address: 127.0.0.1:39944
2019.10.07 13:25:41.701367 [ 25 ] {} <Debug> TCPHandler: Connected Rust SQLDriver version 1.1.0, revision: 54213, database: default, user: default.
2019.10.07 13:25:41.701931 [ 25 ] {ed0c5563-a6bd-41c9-b878-2f43338938be} <Debug> executeQuery: (from 127.0.0.1:39944) select 1;
2019.10.07 13:25:41.702107 [ 25 ] {ed0c5563-a6bd-41c9-b878-2f43338938be} <Trace> InterpreterSelectQuery: FetchColumns -> Complete
2019.10.07 13:25:41.702141 [ 25 ] {ed0c5563-a6bd-41c9-b878-2f43338938be} <Debug> executeQuery: Query pipeline:
Expression
 Expression
  One

2019.10.07 13:25:41.702364 [ 25 ] {ed0c5563-a6bd-41c9-b878-2f43338938be} <Information> executeQuery: Read 1 rows, 1.00 B in 0.000 sec., 2605 rows/sec., 2.54 KiB/sec.
2019.10.07 13:25:41.702385 [ 25 ] {ed0c5563-a6bd-41c9-b878-2f43338938be} <Debug> MemoryTracker: Peak memory usage (for query): 12.09 KiB.
2019.10.07 13:25:41.702420 [ 25 ] {ed0c5563-a6bd-41c9-b878-2f43338938be} <Debug> MemoryTracker: Peak memory usage (total): 12.09 KiB.
2019.10.07 13:25:41.702437 [ 25 ] {ed0c5563-a6bd-41c9-b878-2f43338938be} <Information> TCPHandler: Processed in 0.001 sec.
```
Second query:
```
2019.10.07 13:25:49.528482 [ 25 ] {} <Information> TCPHandler: Done processing connection.
```

No further server logs on the second query, even after timeout.

Some thoughts:

Does this work in an async pool with several connections because:
- there's several connections to provide slack
- timeout errors are hidden amongst many queries?
- the period between queries is long enough, that the timeouts will clear for the next connection in time to keep the pool relatively clear.

This doesn't answer the issue of ultimate hanging though, because in this case the hanging is only temporary, and soon cleared by a db error timeout.

Oh, but this is on v0.1.16. What happens on 0.1.14?
