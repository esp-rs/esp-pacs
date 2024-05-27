///Register `AE_BLOCK_MEAN_5` reader
pub type R = crate::R<AE_BLOCK_MEAN_5_SPEC>;
///Field `AE_B43_MEAN` reader - this field configures block43 Y mean data
pub type AE_B43_MEAN_R = crate::FieldReader;
///Field `AE_B42_MEAN` reader - this field configures block42 Y mean data
pub type AE_B42_MEAN_R = crate::FieldReader;
///Field `AE_B41_MEAN` reader - this field configures block41 Y mean data
pub type AE_B41_MEAN_R = crate::FieldReader;
///Field `AE_B40_MEAN` reader - this field configures block40 Y mean data
pub type AE_B40_MEAN_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - this field configures block43 Y mean data
    #[inline(always)]
    pub fn ae_b43_mean(&self) -> AE_B43_MEAN_R {
        AE_B43_MEAN_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - this field configures block42 Y mean data
    #[inline(always)]
    pub fn ae_b42_mean(&self) -> AE_B42_MEAN_R {
        AE_B42_MEAN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - this field configures block41 Y mean data
    #[inline(always)]
    pub fn ae_b41_mean(&self) -> AE_B41_MEAN_R {
        AE_B41_MEAN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - this field configures block40 Y mean data
    #[inline(always)]
    pub fn ae_b40_mean(&self) -> AE_B40_MEAN_R {
        AE_B40_MEAN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AE_BLOCK_MEAN_5")
            .field("ae_b43_mean", &self.ae_b43_mean())
            .field("ae_b42_mean", &self.ae_b42_mean())
            .field("ae_b41_mean", &self.ae_b41_mean())
            .field("ae_b40_mean", &self.ae_b40_mean())
            .finish()
    }
}
/**ae statistic result register 5

You can [`read`](crate::generic::Reg::read) this register and get [`ae_block_mean_5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AE_BLOCK_MEAN_5_SPEC;
impl crate::RegisterSpec for AE_BLOCK_MEAN_5_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ae_block_mean_5::R`](R) reader structure
impl crate::Readable for AE_BLOCK_MEAN_5_SPEC {}
///`reset()` method sets AE_BLOCK_MEAN_5 to value 0
impl crate::Resettable for AE_BLOCK_MEAN_5_SPEC {
    const RESET_VALUE: u32 = 0;
}
