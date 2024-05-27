#[doc = "Register `AE_BLOCK_MEAN_3` reader"]
pub type R = crate::R<AE_BLOCK_MEAN_3_SPEC>;
#[doc = "Field `AE_B30_MEAN` reader - this field configures block30 Y mean data"]
pub type AE_B30_MEAN_R = crate::FieldReader;
#[doc = "Field `AE_B24_MEAN` reader - this field configures block24 Y mean data"]
pub type AE_B24_MEAN_R = crate::FieldReader;
#[doc = "Field `AE_B23_MEAN` reader - this field configures block23 Y mean data"]
pub type AE_B23_MEAN_R = crate::FieldReader;
#[doc = "Field `AE_B22_MEAN` reader - this field configures block22 Y mean data"]
pub type AE_B22_MEAN_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - this field configures block30 Y mean data"]
    #[inline(always)]
    pub fn ae_b30_mean(&self) -> AE_B30_MEAN_R {
        AE_B30_MEAN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures block24 Y mean data"]
    #[inline(always)]
    pub fn ae_b24_mean(&self) -> AE_B24_MEAN_R {
        AE_B24_MEAN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures block23 Y mean data"]
    #[inline(always)]
    pub fn ae_b23_mean(&self) -> AE_B23_MEAN_R {
        AE_B23_MEAN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures block22 Y mean data"]
    #[inline(always)]
    pub fn ae_b22_mean(&self) -> AE_B22_MEAN_R {
        AE_B22_MEAN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AE_BLOCK_MEAN_3")
            .field("ae_b30_mean", &self.ae_b30_mean())
            .field("ae_b24_mean", &self.ae_b24_mean())
            .field("ae_b23_mean", &self.ae_b23_mean())
            .field("ae_b22_mean", &self.ae_b22_mean())
            .finish()
    }
}
#[doc = "ae statistic result register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ae_block_mean_3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AE_BLOCK_MEAN_3_SPEC;
impl crate::RegisterSpec for AE_BLOCK_MEAN_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ae_block_mean_3::R`](R) reader structure"]
impl crate::Readable for AE_BLOCK_MEAN_3_SPEC {}
#[doc = "`reset()` method sets AE_BLOCK_MEAN_3 to value 0"]
impl crate::Resettable for AE_BLOCK_MEAN_3_SPEC {
    const RESET_VALUE: u32 = 0;
}
