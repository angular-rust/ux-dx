use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug)] // PartialEq, Eq, Hash
pub struct DepthState {
    pub private_member_magic: u32,
    pub private_member_test_enabled: bool,  // Bool
    pub private_member_test_function: i32,  // DepthTestFunction, // TODO: possible should be enum
    pub private_member_write_enabled: bool, // Bool
    pub private_member_range_near: f32,
    pub private_member_range_far: f32,
    pub private_member_padding0: u32,
    pub private_member_padding1: u32,
    pub private_member_padding2: u32,
    pub private_member_padding3: u32,
    pub private_member_padding4: u32,
    pub private_member_padding5: u32,
    pub private_member_padding6: u32,
    pub private_member_padding7: u32,
    pub private_member_padding8: u32,
    pub private_member_padding9: u32,
}

//pub const COGL_DEPTH_TEST_FUNCTION_NEVER: DepthTestFunction = 512;
//pub const COGL_DEPTH_TEST_FUNCTION_LESS: DepthTestFunction = 513;
//pub const COGL_DEPTH_TEST_FUNCTION_EQUAL: DepthTestFunction = 514;
//pub const COGL_DEPTH_TEST_FUNCTION_LEQUAL: DepthTestFunction = 515;
//pub const COGL_DEPTH_TEST_FUNCTION_GREATER: DepthTestFunction = 516;
//pub const COGL_DEPTH_TEST_FUNCTION_NOTEQUAL: DepthTestFunction = 517;
//pub const COGL_DEPTH_TEST_FUNCTION_GEQUAL: DepthTestFunction = 518;
//pub const COGL_DEPTH_TEST_FUNCTION_ALWAYS: DepthTestFunction = 519;

impl DepthState {
    // * depth_state_init:
    // * @state: A #DepthState struct
    // *
    // * Initializes the members of @state to their default values.
    // *
    // * You should never pass an un initialized #DepthState structure
    // * to pipeline_set_depth_state().
    // *
    // * Since: 2.0
    // * Stability: Unstable
    pub fn init(&self) {
        // state->magic = COGL_DEPTH_STATE_MAGIC;

        // /* The same as the GL defaults */
        // state->test_enabled = false;
        // state->write_enabled = true;
        // state->test_function = COGL_DEPTH_TEST_FUNCTION_LESS;
        // state->range_near = 0;
        // state->range_far = 1;
    }

    // * depth_state_set_test_enabled:
    // * @state: A #DepthState struct
    // * @enable: The enable state you want
    // *
    // * Enables or disables depth testing according to the value of
    // * @enable.
    // *
    // * If depth testing is enable then the #DepthTestFunction set
    // * using depth_state_set_test_function() us used to evaluate
    // * the depth value of incoming fragments against the corresponding
    // * value stored in the current depth buffer, and if the test passes
    // * then the fragments depth value is used to update the depth buffer.
    // * (unless you have disabled depth writing via
    // * depth_state_set_write_enabled())
    // *
    // * By default depth testing is disabled.
    // *
    // * NB: this won't directly affect the state of the GPU. You have
    // * to then set the state on a #Pipeline using
    // * pipeline_set_depth_state()
    // *
    // * Since: 2.0
    // * Stability: Unstable
    pub fn set_test_enabled(&self, enable: bool) {
        // _COGL_RETURN_IF_FAIL (state->magic == COGL_DEPTH_STATE_MAGIC);
        // state->test_enabled = enabled;
    }

    // * depth_state_get_test_enabled:
    // * @state: A #DepthState struct
    // *
    // * Gets the current depth test enabled state as previously set by
    // * depth_state_set_test_enabled().
    // *
    // * Returns: The pipeline's current depth test enabled state.
    // * Since: 2.0
    // * Stability: Unstable
    pub fn get_test_enabled(&self) -> bool {
        // _COGL_RETURN_VAL_IF_FAIL (state->magic == COGL_DEPTH_STATE_MAGIC, false);
        // return state->test_enabled;
        unimplemented!()
    }

    // * depth_state_set_write_enabled:
    // * @state: A #DepthState struct
    // * @enable: The enable state you want
    // *
    // * Enables or disables depth buffer writing according to the value of
    // * @enable. Normally when depth testing is enabled and the comparison
    // * between a fragment's depth value and the corresponding depth buffer
    // * value passes then the fragment's depth is written to the depth
    // * buffer unless writing is disabled here.
    // *
    // * By default depth writing is enabled
    // *
    // * NB: this won't directly affect the state of the GPU. You have
    // * to then set the state on a #Pipeline using
    // * pipeline_set_depth_state()
    // *
    // * Since: 2.0
    // * Stability: Unstable
    pub fn set_write_enabled(&self, enable: bool) {
        // _COGL_RETURN_IF_FAIL (state->magic == COGL_DEPTH_STATE_MAGIC);
        // state->write_enabled = enabled;
        unimplemented!()
    }

    // * depth_state_get_write_enabled:
    // * @state: A #DepthState struct
    // *
    // * Gets the depth writing enable state as set by the corresponding
    // * depth_state_set_write_enabled().
    // *
    // * Returns: The current depth writing enable state
    // * Since: 2.0
    // * Stability: Unstable
    pub fn get_write_enabled(&self) -> bool {
        // _COGL_RETURN_VAL_IF_FAIL (state->magic == COGL_DEPTH_STATE_MAGIC, false);
        // return state->write_enabled;
        unimplemented!()
    }

    // // * depth_state_set_test_function:
    // // * @state: A #DepthState struct
    // // * @function: The #DepthTestFunction to set
    // // *
    // // * Sets the #DepthTestFunction used to compare the depth value of
    // // * an incoming fragment against the corresponding value in the current
    // // * depth buffer.
    // // *
    // // * By default the depth test function is %COGL_DEPTH_TEST_FUNCTION_LESS
    // // *
    // // * NB: this won't directly affect the state of the GPU. You have
    // // * to then set the state on a #Pipeline using
    // // * pipeline_set_depth_state()
    // // *
    // // * Since: 2.0
    // // * Stability: Unstable
    // pub fn set_test_function (&self, DepthTestFunction function);

    // // * depth_state_get_test_function:
    // // * @state: A #DepthState struct
    // // *
    // // * Gets the current depth test enable state as previously set via
    // // * depth_state_set_test_enabled().
    // // *
    // // * Returns: The current depth test enable state.
    // // * Since: 2.0
    // // * Stability: Unstable
    // pub fn get_test_function(&self) -> DepthTestFunction {

    // }

    // * depth_state_set_range:
    // * @state: A #DepthState object
    // * @near_val: The near component of the desired depth range which will be
    // * clamped to the range [0, 1]
    // * @far_val: The far component of the desired depth range which will be
    // * clamped to the range [0, 1]
    // *
    // * Sets the range to map depth values in normalized device coordinates
    // * to before writing out to a depth buffer.
    // *
    // * After your geometry has be transformed, clipped and had perspective
    // * division applied placing it in normalized device
    // * coordinates all depth values between the near and far z clipping
    // * planes are in the range -1 to 1. Before writing any depth value to
    // * the depth buffer though the value is mapped into the range [0, 1].
    // *
    // * With this function you can change the range which depth values are
    // * mapped too although the range must still lye within the range [0,
    // * 1].
    // *
    // * If your driver does not support this feature (for example you are
    // * using GLES 1 drivers) then if you don't use the default range
    // * values you will get an error reported when calling
    // * pipeline_set_depth_state (). You can check ahead of time for
    // * the %COGL_FEATURE_ID_DEPTH_RANGE feature with
    // * has_feature() to know if this function will succeed.
    // *
    // * By default normalized device coordinate depth values are mapped to
    // * the full range of depth buffer values, [0, 1].
    // *
    // * NB: this won't directly affect the state of the GPU. You have
    // * to then set the state on a #Pipeline using
    // * pipeline_set_depth_state().
    // *
    // * Since: 2.0
    // * Stability: Unstable
    pub fn set_range(&self, near_val: f32, far_val: f32) {
        // _COGL_RETURN_IF_FAIL (state->magic == COGL_DEPTH_STATE_MAGIC);
        // state->range_near = near;
        // state->range_far = far;
        unimplemented!()
    }

    // * depth_state_get_range:
    // * @state: A #DepthState object
    // * @near_val: A pointer to store the near component of the depth range
    // * @far_val: A pointer to store the far component of the depth range
    // *
    // * Gets the current range to which normalized depth values are mapped
    // * before writing to the depth buffer. This corresponds to the range
    // * set with depth_state_set_range().
    // *
    // * Since: 2.0
    // * Stability: Unstable
    pub fn get_range(&self) -> (f32, f32) {
        // _COGL_RETURN_IF_FAIL (state->magic == COGL_DEPTH_STATE_MAGIC);
        // *near_out = state->range_near;
        // *far_out = state->range_far;
        unimplemented!()
    }
}
