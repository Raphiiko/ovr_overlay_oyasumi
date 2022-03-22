use autocxx::prelude::*; // use all the main autocxx functions

include_cpp! {
    #include "openvr.h"

    generate!("vr::VR_Init")
    generate!("vr::EVRApplicationType")
    generate!("vr::VR_Shutdown")

    generate!("vr::IVRSystem")
    generate!("vr::IVROverlay")

    generate!("vr::VR_GetVRInitErrorAsSymbol")
    generate!("vr::EVRInitError")

}

//pub use ffi::vr::*;
pub use ffi::vr::*;
pub use ffi::{make_string, ToCppString};
