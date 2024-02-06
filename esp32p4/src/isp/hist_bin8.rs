#[doc = "Register `HIST_BIN8` reader"]
pub type R = crate::R<HIST_BIN8_SPEC>;
#[doc = "Field `HIST_BIN_8` reader - this field represents result of histogram bin 8"]
pub type HIST_BIN_8_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - this field represents result of histogram bin 8"]
    #[inline(always)]
    pub fn hist_bin_8(&self) -> HIST_BIN_8_R {
        HIST_BIN_8_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIST_BIN8")
            .field("hist_bin_8", &format_args!("{}", self.hist_bin_8().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HIST_BIN8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "result of histogram bin 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_bin8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIST_BIN8_SPEC;
impl crate::RegisterSpec for HIST_BIN8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_bin8::R`](R) reader structure"]
impl crate::Readable for HIST_BIN8_SPEC {}
#[doc = "`reset()` method sets HIST_BIN8 to value 0"]
impl crate::Resettable for HIST_BIN8_SPEC {
    const RESET_VALUE: u32 = 0;
}
