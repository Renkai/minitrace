# MINITRACE

```sh
$ cargo run --example synchronous
...
...
...
====================================================================== 93 ms
==                                                                      3 ms
==                                                                      3 ms
  ===                                                                   5 ms
   ===                                                                  4 ms
      ===                                                               5 ms
      ===                                                               4 ms
         ====                                                           6 ms
           ===                                                          4 ms
              ======                                                    8 ms
                ===                                                     5 ms
                    =============                                      18 ms
                          =======                                      10 ms
                                 ========                              11 ms
                                       ===                              4 ms
                                          ========                     11 ms
                                               ===                      4 ms
                                                  =========            12 ms
                                                        ===             4 ms
                                                            =========  13 ms
                                                                  ===   4 ms
```

```sh
$ cargo run --example asynchronous
...
...
...
=======================================================                51 ms
                                                                        0 ms
===============================================                        43 ms
                      ========================                         22 ms
===============================================                        43 ms
                      ========================                         22 ms
===============================================                        43 ms
                      ========================                         22 ms
====================================================================== 64 ms
                                               ======================  21 ms
=======================================================                51 ms
```
