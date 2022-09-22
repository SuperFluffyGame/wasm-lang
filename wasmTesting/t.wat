(module
    (memory 1)
    (func $add (param i32 i32)
        i32.const 0
        local.get 0
        local.get 1
        i32.add
        i32.store
    )

    (export "add" (func $add))
)