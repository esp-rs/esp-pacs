#[doc = "Register `HIST_BIN15` reader"]
pub type R = crate::R<HIST_BIN15_SPEC>;
#[doc = "Field `HIST_BIN_15` reader - this field represents result of histogram bin 15"]
pub type HIST_BIN_15_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - this field represents result of histogram bin 15"]
    #[inline(always)]
    pub fn hist_bin_15(&self) -> HIST_BIN_15_R {
        HIST_BIN_15_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIST_BIN15")
            .field("hist_bin_15", &self.hist_bin_15())
            .finish()
    }
}
#[doc = "result of histogram bin 15\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin15::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIST_BIN15_SPEC;
impl crate::RegisterSpec for HIST_BIN15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_bin15::R`](R) reader structure"]
impl crate::Readable for HIST_BIN15_SPEC {}
#[doc = "`reset()` method sets HIST_BIN15 to value 0"]
impl crate::Resettable for HIST_BIN15_SPEC {
    const RESET_VALUE: u32 = 0;
}
