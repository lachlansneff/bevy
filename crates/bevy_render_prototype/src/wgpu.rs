pub trait UnwrapWgpu: Sized {
    type WgpuType;

    fn try_unwrap_wgpu(self) -> Result<Self::WgpuType, ()>;
    #[inline(always)]
    fn unwrap_wgpu(self) -> Self::WgpuType {
        self.try_unwrap_wgpu()
            .unwrap_or_else(|_| crate::wrong_backend())
    }
}
