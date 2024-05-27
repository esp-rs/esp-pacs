///Register `HIST_BIN4` reader
pub type R = crate::R<HIST_BIN4_SPEC>;
///Field `HIST_BIN_4` reader - this field represents result of histogram bin 4
pub type HIST_BIN_4_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:16 - this field represents result of histogram bin 4
    #[inline(always)]
    pub fn hist_bin_4(&self) -> HIST_BIN_4_R {
        HIST_BIN_4_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIST_BIN4")
            .field("hist_bin_4", &self.hist_bin_4())
            .finish()
    }
}
/**result of histogram bin 4

You can [`read`](crate::generic::Reg::read) this register and get [`hist_bin4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HIST_BIN4_SPEC;
impl crate::RegisterSpec for HIST_BIN4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hist_bin4::R`](R) reader structure
impl crate::Readable for HIST_BIN4_SPEC {}
///`reset()` method sets HIST_BIN4 to value 0
impl crate::Resettable for HIST_BIN4_SPEC {
    const RESET_VALUE: u32 = 0;
}
