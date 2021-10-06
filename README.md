MIT license
Eric R. Olson (c) 2021


A simple crate used for profiling code. 

How to use:
1) Enable the `benchmark` feature.
```
[dependencies]
benchy = { .. , features=["benchmark"]}
```
2) Add timers within hot paths, at the start of the block.
3) At some point in your app, call `Benchy::save(file_path)` to write the contents to disk. 


Example: 

```
use benchy::Benchy;

// Scope A
{
    Benchy::time("test1");

    // do some work

    // Scope A exits, so timer was dropped and logs duration.
}

// Call save to save results to disk.
Benchy::save("someRandomFile.txt");
```