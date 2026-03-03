# frozen_string_literal: true

# Extension configuration for the iscc_lib native Ruby extension.
# Uses rb_sys to bridge Cargo's cdylib build into Ruby's mkmf system.

require "mkmf"
require "rb_sys/mkmf"

create_rust_makefile("iscc_lib/iscc_rb")
