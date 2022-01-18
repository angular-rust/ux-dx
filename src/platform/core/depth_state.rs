use super::DepthTestFunction;
use std::{cell::RefCell, mem};

const DEPTH_STATE_MAGIC: u32 = 0xDEADBEEF;

#[derive(Debug, Clone, Copy)]
struct DepthStateProps {
    pub magic: u32,
    pub test_enabled: bool,
    pub test_function: DepthTestFunction,
    pub write_enabled: bool,
    pub range_near: f32,
    pub range_far: f32,
}

#[repr(C)]
#[derive(Debug, Clone)] // PartialEq, Eq, Hash
pub struct DepthState {
    props: RefCell<DepthStateProps>,
}

impl DepthState {
    // depth_state_init:
    // @state: A #DepthState struct
    //
    // Initializes the members of @state to their default values.
    //
    // You should never pass an un initialized #DepthState structure
    // to pipeline_set_depth_state().
    //
    // Since: 2.0
    // Stability: Unstable
    pub fn init(&self) {
        let mut props = self.props.borrow_mut();

        props.magic = DEPTH_STATE_MAGIC;

        // The same as the GL defaults
        props.test_enabled = false;
        props.write_enabled = true;
        props.test_function = DepthTestFunction::Less;
        props.range_near = 0.0;
        props.range_far = 1.0;
    }

    // depth_state_set_test_enabled:
    // @state: A #DepthState struct
    // @enable: The enable state you want
    //
    // Enables or disables depth testing according to the value of
    // @enable.
    //
    // If depth testing is enable then the #DepthTestFunction set
    // using depth_state_set_test_fn() us used to evaluate
    // the depth value of incoming fragments against the corresponding
    // value stored in the current depth buffer, and if the test passes
    // then the fragments depth value is used to update the depth buffer.
    // (unless you have disabled depth writing via
    // depth_state_set_write_enabled())
    //
    // By default depth testing is disabled.
    //
    // NB: this won't directly affect the state of the GPU. You have
    // to then set the state on a #Pipeline using
    // pipeline_set_depth_state()
    //
    // Since: 2.0
    // Stability: Unstable
    pub fn set_test_enabled(&self, enable: bool) {
        let mut props = self.props.borrow_mut();
        props.test_enabled = enable;
    }

    // depth_state_get_test_enabled:
    // @state: A #DepthState struct
    //
    // Gets the current depth test enabled state as previously set by
    // depth_state_set_test_enabled().
    //
    // Returns: The pipeline's current depth test enabled state.
    // Since: 2.0
    // Stability: Unstable
    pub fn test_enabled(&self) -> bool {
        let props = self.props.borrow();
        props.test_enabled
    }

    // depth_state_set_write_enabled:
    // @state: A #DepthState struct
    // @enable: The enable state you want
    //
    // Enables or disables depth buffer writing according to the value of
    // @enable. Normally when depth testing is enabled and the comparison
    // between a fragment's depth value and the corresponding depth buffer
    // value passes then the fragment's depth is written to the depth
    // buffer unless writing is disabled here.
    //
    // By default depth writing is enabled
    //
    // NB: this won't directly affect the state of the GPU. You have
    // to then set the state on a #Pipeline using
    // pipeline_set_depth_state()
    //
    // Since: 2.0
    // Stability: Unstable
    pub fn set_write_enabled(&self, enable: bool) {
        let mut props = self.props.borrow_mut();
        props.write_enabled = enable;
    }

    // depth_state_get_write_enabled:
    // @state: A #DepthState struct
    //
    // Gets the depth writing enable state as set by the corresponding
    // depth_state_set_write_enabled().
    //
    // Returns: The current depth writing enable state
    // Since: 2.0
    // Stability: Unstable
    pub fn write_enabled(&self) -> bool {
        let props = self.props.borrow();
        props.write_enabled
    }

    // // depth_state_set_test_function:
    // // @state: A #DepthState struct
    // // @function: The #DepthTestFunction to set
    // //
    // // Sets the #DepthTestFunction used to compare the depth value of
    // // an incoming fragment against the corresponding value in the current
    // // depth buffer.
    // //
    // // By default the depth test fn is %DEPTH_TEST_FUNCTION_LESS
    // //
    // // NB: this won't directly affect the state of the GPU. You have
    // // to then set the state on a #Pipeline using
    // // pipeline_set_depth_state()
    // //
    // // Since: 2.0
    // // Stability: Unstable
    // pub fn set_test_fn (&self, DepthTestFunction function);

    // // depth_state_get_test_function:
    // // @state: A #DepthState struct
    // //
    // // Gets the current depth test enable state as previously set via
    // // depth_state_set_test_enabled().
    // //
    // // Returns: The current depth test enable state.
    // // Since: 2.0
    // // Stability: Unstable
    // pub fn test_fn(&self) -> DepthTestFunction {

    // }

    // depth_state_set_range:
    // @state: A #DepthState object
    // @near_val: The near component of the desired depth range which will be
    // clamped to the range [0, 1]
    // @far_val: The far component of the desired depth range which will be
    // clamped to the range [0, 1]
    //
    // Sets the range to map depth values in normalized device coordinates
    // to before writing out to a depth buffer.
    //
    // After your geometry has be transformed, clipped and had perspective
    // division applied placing it in normalized device
    // coordinates all depth values between the near and far z clipping
    // planes are in the range -1 to 1. Before writing any depth value to
    // the depth buffer though the value is mapped into the range [0, 1].
    //
    // With this fn you can change the range which depth values are
    // mapped too although the range must still lye within the range [0,
    // 1].
    //
    // If your driver does not support this feature (for example you are
    // using GLES 1 drivers) then if you don't use the default range
    // values you will get an error reported when calling
    // pipeline_set_depth_state (). You can check ahead of time for
    // the %FEATURE_ID_DEPTH_RANGE feature with
    // has_feature() to know if this fn will succeed.
    //
    // By default normalized device coordinate depth values are mapped to
    // the full range of depth buffer values, [0, 1].
    //
    // NB: this won't directly affect the state of the GPU. You have
    // to then set the state on a #Pipeline using
    // pipeline_set_depth_state().
    //
    // Since: 2.0
    // Stability: Unstable
    pub fn set_range(&self, near_val: f32, far_val: f32) {
        let mut props = self.props.borrow_mut();
        props.range_near = near_val;
        props.range_far = far_val;
    }

    // depth_state_get_range:
    // @state: A #DepthState object
    // @near_val: A pointer to store the near component of the depth range
    // @far_val: A pointer to store the far component of the depth range
    //
    // Gets the current range to which normalized depth values are mapped
    // before writing to the depth buffer. This corresponds to the range
    // set with depth_state_set_range().
    //
    // Since: 2.0
    // Stability: Unstable
    pub fn range(&self) -> (f32, f32) {
        let props = self.props.borrow();
        (props.range_near, props.range_far)
    }
}
