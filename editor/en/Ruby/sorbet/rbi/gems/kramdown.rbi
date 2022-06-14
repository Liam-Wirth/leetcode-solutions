# This file is autogenerated. Do not edit it by hand. Regenerate it with:
#   srb rbi gems

# typed: true
#
# If you would like to make changes to this file, great! Please create the gem's shim here:
#
#   https://github.com/sorbet/sorbet-typed/new/master?filename=lib/kramdown/all/kramdown.rbi
#
# kramdown-2.4.0

module Kramdown
end
module Kramdown::Parser
end
class Kramdown::Parser::Kramdown < Kramdown::Parser::Base
  def configure_parser; end
  def initialize(source, options); end
  def new_block_el(*args); end
  def parse; end
  def parse_blocks(el, text = nil); end
  def parse_spans(el, stop_re = nil, parsers = nil, text_type = nil); end
  def reset_env(opts = nil); end
  def restore_env(env); end
  def save_env; end
  def self.define_parser(name, start_re, span_start = nil, meth_name = nil); end
  def self.has_parser?(name); end
  def self.parser(name = nil); end
  def span_parser_regexps(parsers = nil); end
  def span_pattern_cache(stop_re, span_start); end
  def update_attr_with_ial(attr, ial); end
  def update_link_definitions(link_defs); end
  def update_raw_text(item); end
  def update_tree(element); end
  include Kramdown
end
class Kramdown::Parser::Kramdown::Data < Struct
  def method; end
  def method=(_); end
  def name; end
  def name=(_); end
  def self.[](*arg0); end
  def self.inspect; end
  def self.members; end
  def self.new(*arg0); end
  def span_start; end
  def span_start=(_); end
  def start_re; end
  def start_re=(_); end
end
