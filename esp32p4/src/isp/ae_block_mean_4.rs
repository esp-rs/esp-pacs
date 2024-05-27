#[doc = "Register `AE_BLOCK_MEAN_4` reader"]
pub type R = crate::R<AE_BLOCK_MEAN_4_SPEC>;
#[doc = "Field `AE_B34_MEAN` reader - this field configures block34 Y mean data"]
pub type AE_B34_MEAN_R = crate::FieldReader;
#[doc = "Field `AE_B33_MEAN` reader - this field configures block33 Y mean data"]
pub type AE_B33_MEAN_R = crate::FieldReader;
#[doc = "Field `AE_B32_MEAN` reader - this field configures block32 Y mean data"]
pub type AE_B32_MEAN_R = crate::FieldReader;
#[doc = "Field `AE_B31_MEAN` reader - this field configures block31 Y mean data"]
pub type AE_B31_MEAN_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - this field configures block34 Y mean data"]
    #[inline(always)]
    pub fn ae_b34_mean(&self) -> AE_B34_MEAN_R {
        AE_B34_MEAN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures block33 Y mean data"]
    #[inline(always)]
    pub fn ae_b33_mean(&self) -> AE_B33_MEAN_R {
        AE_B33_MEAN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures block32 Y mean data"]
    #[inline(always)]
    pub fn ae_b32_mean(&self) -> AE_B32_MEAN_R {
        AE_B32_MEAN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures block31 Y mean data"]
    #[inline(always)]
    pub fn ae_b31_mean(&self) -> AE_B31_MEAN_R {
        AE_B31_MEAN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AE_BLOCK_MEAN_4")
            .field("ae_b34_mean", &self.ae_b34_mean())
            .field("ae_b33_mean", &self.ae_b33_mean())
            .field("ae_b32_mean", &self.ae_b32_mean())
            .field("ae_b31_mean", &self.ae_b31_mean())
            .finish()
    }
}
#[doc = "ae statistic result register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ae_block_mean_4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AE_BLOCK_MEAN_4_SPEC;
impl crate::RegisterSpec for AE_BLOCK_MEAN_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ae_block_mean_4::R`](R) reader structure"]
impl crate::Readable for AE_BLOCK_MEAN_4_SPEC {}
#[doc = "`reset()` method sets AE_BLOCK_MEAN_4 to value 0"]
impl crate::Resettable for AE_BLOCK_MEAN_4_SPEC {
    const RESET_VALUE: u32 = 0;
}
