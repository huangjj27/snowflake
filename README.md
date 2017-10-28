# SnowFlake
The `snowflake` crate is an implement of [twitter's snowflake algorithm](https://github.com/twitter/snowflake)
written in rust. Currently it generate uuid as follows:  

- 1 bit for unused sign bit
- 41 bits for milliseconds timestamp since `std::time::UNIX_EPOCH`
- 10 bits for generator id:
- 5 bits for datacenter id
- 5 bits for worker id in specific datacenter
- rest 12 bits for sequence id generated in the same timestamp at the generator

In fact, the bits of the each three information can be flow, as long as they
can form a 64 bit that can be store in an `i64`

TODO:  

- make the bits of information configurable
- make the codes more neat

author by [h_ang!(J27);](mailto:hunagjj.27@qq.com)
