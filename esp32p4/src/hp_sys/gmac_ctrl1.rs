#[doc = "Register `GMAC_CTRL1` reader"]
pub type R = crate::R<GMAC_CTRL1_SPEC>;
#[doc = "Field `PTP_TIMESTAMP_L` reader - N/A"]
pub type PTP_TIMESTAMP_L_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    pub fn ptp_timestamp_l(&self) -> PTP_TIMESTAMP_L_R {
        PTP_TIMESTAMP_L_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GMAC_CTRL1")
            .field("ptp_timestamp_l", &self.ptp_timestamp_l())
            .finish()
    }
}
#[doc = "N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac_ctrl1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GMAC_CTRL1_SPEC;
impl crate::RegisterSpec for GMAC_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_ctrl1::R`](R) reader structure"]
impl crate::Readable for GMAC_CTRL1_SPEC {}
#[doc = "`reset()` method sets GMAC_CTRL1 to value 0"]
impl crate::Resettable for GMAC_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
