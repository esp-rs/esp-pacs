#[doc = "Register `HIST_BIN7` reader"]
pub type R = crate::R<HIST_BIN7_SPEC>;
#[doc = "Field `HIST_BIN_7` reader - this field represents result of histogram bin 7"]
pub type HIST_BIN_7_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - this field represents result of histogram bin 7"]
    #[inline(always)]
    pub fn hist_bin_7(&self) -> HIST_BIN_7_R {
        HIST_BIN_7_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIST_BIN7")
            .field("hist_bin_7", &format_args!("{}", self.hist_bin_7().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HIST_BIN7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "result of histogram bin 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_bin7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIST_BIN7_SPEC;
impl crate::RegisterSpec for HIST_BIN7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_bin7::R`](R) reader structure"]
impl crate::Readable for HIST_BIN7_SPEC {}
#[doc = "`reset()` method sets HIST_BIN7 to value 0"]
impl crate::Resettable for HIST_BIN7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
