#[doc = "Register `RD_BLK2_DATA2` reader"]
pub type R = crate::R<RD_BLK2_DATA2_SPEC>;
#[doc = "Field `LDO_VOL_BIAS_CONFIG_HIGH` reader - Store the bit \\[3:29\\] of ido configuration parameters."]
pub type LDO_VOL_BIAS_CONFIG_HIGH_R = crate::FieldReader<u32>;
#[doc = "Field `PVT_LOW` reader - Store the bit \\[0:4\\] of pvt."]
pub type PVT_LOW_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:26 - Store the bit \\[3:29\\] of ido configuration parameters."]
    #[inline(always)]
    pub fn ldo_vol_bias_config_high(&self) -> LDO_VOL_BIAS_CONFIG_HIGH_R {
        LDO_VOL_BIAS_CONFIG_HIGH_R::new(self.bits & 0x07ff_ffff)
    }
    #[doc = "Bits 27:31 - Store the bit \\[0:4\\] of pvt."]
    #[inline(always)]
    pub fn pvt_low(&self) -> PVT_LOW_R {
        PVT_LOW_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_BLK2_DATA2")
            .field("ldo_vol_bias_config_high", &self.ldo_vol_bias_config_high())
            .field("pvt_low", &self.pvt_low())
            .finish()
    }
}
#[doc = "Register 2 of BLOCK2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_blk2_data2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_BLK2_DATA2_SPEC;
impl crate::RegisterSpec for RD_BLK2_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_blk2_data2::R`](R) reader structure"]
impl crate::Readable for RD_BLK2_DATA2_SPEC {}
#[doc = "`reset()` method sets RD_BLK2_DATA2 to value 0"]
impl crate::Resettable for RD_BLK2_DATA2_SPEC {
    const RESET_VALUE: u32 = 0;
}
