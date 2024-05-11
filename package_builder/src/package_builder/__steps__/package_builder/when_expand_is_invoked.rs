use super::super::Context;

pub fn when_expand_is_invoked(context: &mut Context) {
  let error;
  let output;
  let result;

  {
    let builder = context.package_builder_mut();
    result = builder.expand();
    error = builder.error().clone();
    output = builder.output().clone();
  }

  context.result_set(result);

  context.error_set(error);
  context.output_set(output);
}
