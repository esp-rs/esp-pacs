///Register `AE_BLOCK_MEAN_2` reader
pub type R = crate::R<AE_BLOCK_MEAN_2_SPEC>;
///Field `AE_B21_MEAN` reader - this field configures block21 Y mean data
pub type AE_B21_MEAN_R = crate::FieldReader;
///Field `AE_B20_MEAN` reader - this field configures block20 Y mean data
pub type AE_B20_MEAN_R = crate::FieldReader;
///Field `AE_B14_MEAN` reader - this field configures block14 Y mean data
pub type AE_B14_MEAN_R = crate::FieldReader;
///Field `AE_B13_MEAN` reader - this field configures block13 Y mean data
pub type AE_B13_MEAN_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - this field configures block21 Y mean data
    #[inline(always)]
    pub fn ae_b21_mean(&self) -> AE_B21_MEAN_R {
        AE_B21_MEAN_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - this field configures block20 Y mean data
    #[inline(always)]
    pub fn ae_b20_mean(&self) -> AE_B20_MEAN_R {
        AE_B20_MEAN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - this field configures block14 Y mean data
    #[inline(always)]
    pub fn ae_b14_mean(&self) -> AE_B14_MEAN_R {
        AE_B14_MEAN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - this field configures block13 Y mean data
    #[inline(always)]
    pub fn ae_b13_mean(&self) -> AE_B13_MEAN_R {
        AE_B13_MEAN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AE_BLOCK_MEAN_2")
            .field("ae_b21_mean", &self.ae_b21_mean())
            .field("ae_b20_mean", &self.ae_b20_mean())
            .field("ae_b14_mean", &self.ae_b14_mean())
            .field("ae_b13_mean", &self.ae_b13_mean())
            .finish()
    }
}
/**ae statistic result register 2

You can [`read`](crate::generic::Reg::read) this register and get [`ae_block_mean_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AE_BLOCK_MEAN_2_SPEC;
impl crate::RegisterSpec for AE_BLOCK_MEAN_2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ae_block_mean_2::R`](R) reader structure
impl crate::Readable for AE_BLOCK_MEAN_2_SPEC {}
///`reset()` method sets AE_BLOCK_MEAN_2 to value 0
impl crate::Resettable for AE_BLOCK_MEAN_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
