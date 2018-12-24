
## rust-protobuf (rp)

+ Low runtime performance
+ Generates complicated proto rust types
	+ Where some APIs provided are never used
+ Supports service
+ Supports zero-copy

## quick-protobuf (qp)

+ Does not depend on protoc
+ Does not support services
+ Generates simple proto rust types
	+ No trait objects
	+ No virtual functions
+ Fastest among all (https://github.com/tafia/quick-protobuf/tree/master/perftest)
+ I Failed to generate anything in kvproto
	+ Silly `--include` option, wrong relative path resolving
	+ Even if we use absolute path, #133 still blocks the generation
+ The `BytesReader`'s design is compatible with `grpcio`'s zero copy
+ Summary of missing features: #12

## prost (pr)

+ Faster than rp, according to qp's bench
+ Comments are retained
+ Cannot generate .proto imported by other .protos
	+ Easy fix: create a .proto which imports all other protos and generate this proto
+ Generation process is a part of `cargo build`
+ Does not generate `lib.rs`, needs some manual editing
+ Not compatible with `grpcio`'s zero copy (#134, #31)
+ Supports services via custom code generator
