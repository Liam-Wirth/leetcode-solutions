# This file is autogenerated. Do not edit it by hand. Regenerate it with:
#   srb rbi gems

# typed: strict
#
# If you would like to make changes to this file, great! Please create the gem's shim here:
#
#   https://github.com/sorbet/sorbet-typed/new/master?filename=lib/thor/all/thor.rbi
#
# thor-1.2.1

class Thor::Group
  def _invoke_for_class_method(klass, command = nil, *args, &block); end
  def self.banner; end
  def self.baseclass; end
  def self.class_options_help(shell, groups = nil); end
  def self.create_command(meth); end
  def self.create_task(meth); end
  def self.desc(description = nil); end
  def self.dispatch(command, given_args, given_opts, config); end
  def self.get_options_from_invocations(group_options, base_options); end
  def self.handle_argument_error(command, error, _args, arity); end
  def self.help(shell); end
  def self.invocation_blocks; end
  def self.invocations; end
  def self.invoke(*names, &block); end
  def self.invoke_from_option(*names, &block); end
  def self.printable_commands(*arg0); end
  def self.printable_tasks(*arg0); end
  def self.remove_invocation(*names); end
  def self.self_command; end
  def self.self_task; end
  extend Thor::Base::ClassMethods
  extend Thor::Invocation::ClassMethods
  include Thor::Base
  include Thor::Invocation
  include Thor::Shell
end
