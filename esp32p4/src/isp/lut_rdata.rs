#[doc = "Register `LUT_RDATA` reader"]
pub type R = crate::R<LUT_RDATA_SPEC>;
#[doc = "Field `LUT_RDATA` reader - this field represents the read data of lut. read ISP_LUT_RDATA after write ISP_LUT_CMD register"]
pub type LUT_RDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - this field represents the read data of lut. read ISP_LUT_RDATA after write ISP_LUT_CMD register"]
    #[inline(always)]
    pub fn lut_rdata(&self) -> LUT_RDATA_R {
        LUT_RDATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LUT_RDATA")
            .field("lut_rdata", &self.lut_rdata())
            .finish()
    }
}
#[doc = "LUT read data register\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_rdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LUT_RDATA_SPEC;
impl crate::RegisterSpec for LUT_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lut_rdata::R`](R) reader structure"]
impl crate::Readable for LUT_RDATA_SPEC {}
#[doc = "`reset()` method sets LUT_RDATA to value 0"]
impl crate::Resettable for LUT_RDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
