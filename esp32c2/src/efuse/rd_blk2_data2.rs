#[doc = "Register `RD_BLK2_DATA2` reader"]
pub struct R(crate::R<RD_BLK2_DATA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_BLK2_DATA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_BLK2_DATA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_BLK2_DATA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LDO_VOL_BIAS_CONFIG_HIGH` reader - Store the bit \\[3:29\\] of ido configuration parameters."]
pub type LDO_VOL_BIAS_CONFIG_HIGH_R = crate::FieldReader<u32, u32>;
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
            .field(
                "ldo_vol_bias_config_high",
                &format_args!("{}", self.ldo_vol_bias_config_high().bits()),
            )
            .field("pvt_low", &format_args!("{}", self.pvt_low().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_BLK2_DATA2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Register 2 of BLOCK2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_blk2_data2](index.html) module"]
pub struct RD_BLK2_DATA2_SPEC;
impl crate::RegisterSpec for RD_BLK2_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_blk2_data2::R](R) reader structure"]
impl crate::Readable for RD_BLK2_DATA2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_BLK2_DATA2 to value 0"]
impl crate::Resettable for RD_BLK2_DATA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
