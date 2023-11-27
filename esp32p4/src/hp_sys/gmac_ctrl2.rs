#[doc = "Register `GMAC_CTRL2` reader"]
pub type R = crate::R<GMAC_CTRL2_SPEC>;
#[doc = "Field `PTP_TIMESTAMP_H` reader - N/A"]
pub type PTP_TIMESTAMP_H_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    pub fn ptp_timestamp_h(&self) -> PTP_TIMESTAMP_H_R {
        PTP_TIMESTAMP_H_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GMAC_CTRL2")
            .field(
                "ptp_timestamp_h",
                &format_args!("{}", self.ptp_timestamp_h().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GMAC_CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "N/A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_ctrl2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GMAC_CTRL2_SPEC;
impl crate::RegisterSpec for GMAC_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_ctrl2::R`](R) reader structure"]
impl crate::Readable for GMAC_CTRL2_SPEC {}
#[doc = "`reset()` method sets GMAC_CTRL2 to value 0"]
impl crate::Resettable for GMAC_CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
