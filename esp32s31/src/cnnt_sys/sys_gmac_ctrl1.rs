#[doc = "Register `SYS_GMAC_CTRL1` reader"]
pub type R = crate::R<SYS_GMAC_CTRL1_SPEC>;
#[doc = "Field `SYS_PTP_TIMESTAMP_L` reader - "]
pub type SYS_PTP_TIMESTAMP_L_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sys_ptp_timestamp_l(&self) -> SYS_PTP_TIMESTAMP_L_R {
        SYS_PTP_TIMESTAMP_L_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_GMAC_CTRL1")
            .field("sys_ptp_timestamp_l", &self.sys_ptp_timestamp_l())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_gmac_ctrl1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_GMAC_CTRL1_SPEC;
impl crate::RegisterSpec for SYS_GMAC_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_gmac_ctrl1::R`](R) reader structure"]
impl crate::Readable for SYS_GMAC_CTRL1_SPEC {}
#[doc = "`reset()` method sets SYS_GMAC_CTRL1 to value 0"]
impl crate::Resettable for SYS_GMAC_CTRL1_SPEC {}
