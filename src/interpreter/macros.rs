#[macro_export]
macro_rules! is_var_defined(
    ($runtime:expr, $name:expr) => ($runtime.borrow().is_var_defined($name))
);

#[macro_export]
macro_rules! set_var(
    ($runtime:expr, $name:expr, $val:expr) => (
        $runtime.borrow_mut().set_var_value($name.clone(), $val)
    )
);

#[macro_export]
macro_rules! get_var(
    ($runtime:expr, $name:expr) => (
        $runtime.borrow().get_var_value($name)
    )
);

#[macro_export]
macro_rules! scope(
    ($runtime:expr) => (Runtime::new_scope($runtime.clone()))
);

#[macro_export]
macro_rules! node_at(
    ($nodes:expr, $position:expr) => ($nodes.get($position).unwrap())
);

#[macro_export]
macro_rules! assert_number_of_arguments(
    ($nodes:expr, $name:expr, $number:expr) => (
        if $nodes.len() != $number {
            runtime_error!(
                "The '{}' construct expects {} arguments - name and value.\n\
                Passed : {:?}", $name, $number, $nodes
            );
        }
    )
);

#[macro_export]
macro_rules! assert_at_least_number_of_arguments(
    ($nodes:expr, $name:expr, $number:expr) => (
        if $nodes.len() < $number {
            runtime_error!(
                "The '{}' construct expects at least {} arguments - name and value.\n\
                Passed : {:?}", $name, $number, $nodes
            );
        }
    );
);

#[macro_export]
macro_rules! try_or_err_to_string(
    ($val:expr) => (
        match $val {
            Ok(v) => v,
            Err(e) => return Err(e.to_string())
        }
    )
);

#[macro_export]
macro_rules! test_assert_run(
    ($src:expr, $res:expr) => (
        assert_eq!(run($src, &Runtime::new()).unwrap(), $res)
    )
);
