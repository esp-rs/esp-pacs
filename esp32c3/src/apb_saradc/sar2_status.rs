#[doc = "Register `SAR2_STATUS` reader"]
pub type R = crate::R<SAR2_STATUS_SPEC>;
#[doc = "Field `SARADC_SAR2_STATUS` reader - saradc2 status about data and channel"]
pub type SARADC_SAR2_STATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - saradc2 status about data and channel"]
    #[inline(always)]
    pub fn saradc_sar2_status(&self) -> SARADC_SAR2_STATUS_R {
        SARADC_SAR2_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR2_STATUS")
            .field(
                "saradc_sar2_status",
                &format_args!("{}", self.saradc_sar2_status().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR2_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar2_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR2_STATUS_SPEC;
impl crate::RegisterSpec for SAR2_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar2_status::R`](R) reader structure"]
impl crate::Readable for SAR2_STATUS_SPEC {}
#[doc = "`reset()` method sets SAR2_STATUS to value 0"]
impl crate::Resettable for SAR2_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
