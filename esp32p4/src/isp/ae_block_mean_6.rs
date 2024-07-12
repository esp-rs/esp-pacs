#[doc = "Register `AE_BLOCK_MEAN_6` reader"]
pub type R = crate::R<AE_BLOCK_MEAN_6_SPEC>;
#[doc = "Field `AE_B44_MEAN` reader - this field configures block44 Y mean data"]
pub type AE_B44_MEAN_R = crate::FieldReader;
impl R {
    #[doc = "Bits 24:31 - this field configures block44 Y mean data"]
    #[inline(always)]
    pub fn ae_b44_mean(&self) -> AE_B44_MEAN_R {
        AE_B44_MEAN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AE_BLOCK_MEAN_6")
            .field("ae_b44_mean", &self.ae_b44_mean())
            .finish()
    }
}
#[doc = "ae statistic result register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_block_mean_6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AE_BLOCK_MEAN_6_SPEC;
impl crate::RegisterSpec for AE_BLOCK_MEAN_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ae_block_mean_6::R`](R) reader structure"]
impl crate::Readable for AE_BLOCK_MEAN_6_SPEC {}
#[doc = "`reset()` method sets AE_BLOCK_MEAN_6 to value 0"]
impl crate::Resettable for AE_BLOCK_MEAN_6_SPEC {
    const RESET_VALUE: u32 = 0;
}
