#[derive(Debug, Clone)]
pub enum GpuInfoArchitectureFlag {
    VertexImmediateMode,
    VertexTiled,
    VertexSoftware,
    FragmentImmediateMode,
    FragmentDefered,
    FragmentSoftware,
}

impl Default for GpuInfoArchitectureFlag {
    fn default() -> Self {
        Self::VertexImmediateMode
    }
}

#[derive(Debug, Clone)]
pub enum GpuInfoArchitecture {
    Unknown,
    SandyBridge,
    Sgx,
    Mali,
    LllvmPipe,
    SoftPipe,
    SwRast,
}

impl Default for GpuInfoArchitecture {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Clone)]
pub enum GpuInfoVendor {
    Unknown,
    Intel,
    ImaginationTechnologies,
    Arm,
    Qualcomm,
    Nvidia,
    Ati,
    Mesa,
}

impl Default for GpuInfoVendor {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Clone)]
pub enum GpuInfoDriverPackage {
    Unknown,
    Mesa,
}

impl Default for GpuInfoDriverPackage {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Clone)]
pub enum GpuInfoDriverBug {
    None,
    // If this bug is present then it is faster to read pixels into a
    // PBO and then memcpy out of the PBO into system memory rather than
    // directly read into system memory.
    // https://bugs.freedesktop.org/show_bug.cgi?id=46631
    Mesa46631SlowReadPixels = 1 << 0,
}

impl Default for GpuInfoDriverBug {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Default, Debug, Clone)]
pub struct GpuInfoStrings {
    renderer_string: String,
    version_string: String,
    vendor_string: String,
}

#[derive(Default, Debug, Clone)]
pub struct GpuInfoArchitectureDescription {
    architecture: GpuInfoArchitecture,
    name: String,
    flags: GpuInfoArchitectureFlag,
    //   CoglBool (* check_function) (const CoglGpuInfoStrings *strings);
}

#[derive(Default, Debug, Clone)]
pub struct GpuInfoVendorDescription {
    vendor: GpuInfoVendor,
    name: String,
    //   CoglBool (* check_function) (const CoglGpuInfoStrings *strings);
    architectures: GpuInfoArchitectureDescription,
}

#[derive(Default, Debug, Clone)]
pub struct GpuInfoDriverPackageDescription {
    driver_package: GpuInfoDriverPackage,
    name: String,
    //   CoglBool (* check_function) (const CoglGpuInfoStrings *strings, int *version_out);
}

#[derive(Default, Debug, Clone)]
pub struct GpuInfo {
    vendor: GpuInfoVendor,
    vendor_name: Option<String>,

    driver_package: GpuInfoDriverPackage,
    driver_package_name: Option<String>,
    driver_package_version: i32,

    architecture: GpuInfoArchitecture,
    architecture_name: Option<String>,
    architecture_flags: GpuInfoArchitectureFlag,

    driver_bugs: GpuInfoDriverBug,
}
