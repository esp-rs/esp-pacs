#[doc = "Register `AE_BLOCK_MEAN_0` reader"]
pub type R = crate::R<AE_BLOCK_MEAN_0_SPEC>;
#[doc = "Field `AE_B03_MEAN` reader - this field configures block03 Y mean data"]
pub type AE_B03_MEAN_R = crate::FieldReader;
#[doc = "Field `AE_B02_MEAN` reader - this field configures block02 Y mean data"]
pub type AE_B02_MEAN_R = crate::FieldReader;
#[doc = "Field `AE_B01_MEAN` reader - this field configures block01 Y mean data"]
pub type AE_B01_MEAN_R = crate::FieldReader;
#[doc = "Field `AE_B00_MEAN` reader - this field configures block00 Y mean data"]
pub type AE_B00_MEAN_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - this field configures block03 Y mean data"]
    #[inline(always)]
    pub fn ae_b03_mean(&self) -> AE_B03_MEAN_R {
        AE_B03_MEAN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures block02 Y mean data"]
    #[inline(always)]
    pub fn ae_b02_mean(&self) -> AE_B02_MEAN_R {
        AE_B02_MEAN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures block01 Y mean data"]
    #[inline(always)]
    pub fn ae_b01_mean(&self) -> AE_B01_MEAN_R {
        AE_B01_MEAN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures block00 Y mean data"]
    #[inline(always)]
    pub fn ae_b00_mean(&self) -> AE_B00_MEAN_R {
        AE_B00_MEAN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AE_BLOCK_MEAN_0")
            .field("ae_b03_mean", &self.ae_b03_mean())
            .field("ae_b02_mean", &self.ae_b02_mean())
            .field("ae_b01_mean", &self.ae_b01_mean())
            .field("ae_b00_mean", &self.ae_b00_mean())
            .finish()
    }
}
#[doc = "ae statistic result register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_block_mean_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AE_BLOCK_MEAN_0_SPEC;
impl crate::RegisterSpec for AE_BLOCK_MEAN_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ae_block_mean_0::R`](R) reader structure"]
impl crate::Readable for AE_BLOCK_MEAN_0_SPEC {}
#[doc = "`reset()` method sets AE_BLOCK_MEAN_0 to value 0"]
impl crate::Resettable for AE_BLOCK_MEAN_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
