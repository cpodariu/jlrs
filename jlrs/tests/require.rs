mod util;
#[cfg(feature = "sync-rt")]
mod tests {
    use super::util::JULIA;
    use jlrs::prelude::*;

    #[test]
    fn load_module() {
        JULIA.with(|j| {
            let mut frame = StackFrame::new();
            let mut jlrs = j.borrow_mut();

            jlrs.instance(&mut frame)
                .scope(|mut frame| unsafe {
                    Module::main(&frame)
                        .require(&mut frame, "LinearAlgebra")
                        .expect("Cannot load LinearAlgebra");
                    Ok(())
                })
                .unwrap();
        });
    }

    #[test]
    fn cannot_load_nonexistent_module() {
        JULIA.with(|j| {
            let mut frame = StackFrame::new();
            let mut jlrs = j.borrow_mut();

            jlrs.instance(&mut frame)
                .scope(|mut frame| unsafe {
                    Module::main(&frame)
                        .require(&mut frame, "LnearAlgebra")
                        .expect_err("Can load LnearAlgebra");
                    Ok(())
                })
                .unwrap();
        });
    }

    #[test]
    fn call_function_from_loaded_module() {
        JULIA.with(|j| {
            let mut frame = StackFrame::new();
            let mut jlrs = j.borrow_mut();

            jlrs.instance(&mut frame)
                .scope(|mut frame| unsafe {
                    let func = Module::base(&frame)
                        .require(&mut frame, "LinearAlgebra")
                        .expect("Cannot load LinearAlgebra")
                        .cast::<Module>()?
                        .function(&frame, "dot")?
                        .wrapper_unchecked();

                    let mut arr1 = vec![1.0f64, 2.0f64];
                    let mut arr2 = vec![2.0f64, 3.0f64];

                    let arr1_v =
                        Array::from_slice_unchecked(frame.as_extended_target(), &mut arr1, 2)?;
                    let arr2_v =
                        Array::from_slice_unchecked(frame.as_extended_target(), &mut arr2, 2)?;

                    let res = func
                        .call2(&mut frame, arr1_v.as_value(), arr2_v.as_value())
                        .expect("Cannot call LinearAlgebra.dot")
                        .unbox::<f64>()?;

                    assert_eq!(res, 8.0);

                    Ok(())
                })
                .unwrap();
        });
    }
}