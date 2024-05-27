///Register `AE_BLOCK_MEAN_1` reader
pub type R = crate::R<AE_BLOCK_MEAN_1_SPEC>;
///Field `AE_B12_MEAN` reader - this field configures block12 Y mean data
pub type AE_B12_MEAN_R = crate::FieldReader;
///Field `AE_B11_MEAN` reader - this field configures block11 Y mean data
pub type AE_B11_MEAN_R = crate::FieldReader;
///Field `AE_B10_MEAN` reader - this field configures block10 Y mean data
pub type AE_B10_MEAN_R = crate::FieldReader;
///Field `AE_B04_MEAN` reader - this field configures block04 Y mean data
pub type AE_B04_MEAN_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - this field configures block12 Y mean data
    #[inline(always)]
    pub fn ae_b12_mean(&self) -> AE_B12_MEAN_R {
        AE_B12_MEAN_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - this field configures block11 Y mean data
    #[inline(always)]
    pub fn ae_b11_mean(&self) -> AE_B11_MEAN_R {
        AE_B11_MEAN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - this field configures block10 Y mean data
    #[inline(always)]
    pub fn ae_b10_mean(&self) -> AE_B10_MEAN_R {
        AE_B10_MEAN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - this field configures block04 Y mean data
    #[inline(always)]
    pub fn ae_b04_mean(&self) -> AE_B04_MEAN_R {
        AE_B04_MEAN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AE_BLOCK_MEAN_1")
            .field("ae_b12_mean", &self.ae_b12_mean())
            .field("ae_b11_mean", &self.ae_b11_mean())
            .field("ae_b10_mean", &self.ae_b10_mean())
            .field("ae_b04_mean", &self.ae_b04_mean())
            .finish()
    }
}
/**ae statistic result register 1

You can [`read`](crate::generic::Reg::read) this register and get [`ae_block_mean_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AE_BLOCK_MEAN_1_SPEC;
impl crate::RegisterSpec for AE_BLOCK_MEAN_1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ae_block_mean_1::R`](R) reader structure
impl crate::Readable for AE_BLOCK_MEAN_1_SPEC {}
///`reset()` method sets AE_BLOCK_MEAN_1 to value 0
impl crate::Resettable for AE_BLOCK_MEAN_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
