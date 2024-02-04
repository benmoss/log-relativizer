# log-relativizer

Allows you to turn ISO8601 timestamps in a file into durations relative to the first timestamp. It's easier to demonstrate than explain, so:

```shell
$ cat examples/test.log | log-relativizer
[0s] starting the tests
[1s928ms] test 1
[2s196ms] some logs and crap
[2s461ms] test 2
[3s53ms] whoa so many logs
[3s458ms] tests complete!
```
