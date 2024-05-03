#[doc = "Register `RD_BLK2_DATA1` reader"]
pub type R = crate::R<RD_BLK2_DATA1_SPEC>;
#[doc = "Field `MAC_ID_HIGH` reader - Store the bit \\[31:47\\] of MAC."]
pub type MAC_ID_HIGH_R = crate::FieldReader<u16>;
#[doc = "Field `WAFER_VERSION` reader - Store wafer version."]
pub type WAFER_VERSION_R = crate::FieldReader;
#[doc = "Field `PKG_VERSION` reader - Store package version."]
pub type PKG_VERSION_R = crate::FieldReader;
#[doc = "Field `BLK2_EFUSE_VERSION` reader - Store efuse version."]
pub type BLK2_EFUSE_VERSION_R = crate::FieldReader;
#[doc = "Field `RF_REF_I_BIAS_CONFIG` reader - Store rf configuration parameters."]
pub type RF_REF_I_BIAS_CONFIG_R = crate::FieldReader;
#[doc = "Field `LDO_VOL_BIAS_CONFIG_LOW` reader - Store the bit \\[0:2\\] of ido configuration parameters."]
pub type LDO_VOL_BIAS_CONFIG_LOW_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Store the bit \\[31:47\\] of MAC."]
    #[inline(always)]
    pub fn mac_id_high(&self) -> MAC_ID_HIGH_R {
        MAC_ID_HIGH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Store wafer version."]
    #[inline(always)]
    pub fn wafer_version(&self) -> WAFER_VERSION_R {
        WAFER_VERSION_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Store package version."]
    #[inline(always)]
    pub fn pkg_version(&self) -> PKG_VERSION_R {
        PKG_VERSION_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - Store efuse version."]
    #[inline(always)]
    pub fn blk2_efuse_version(&self) -> BLK2_EFUSE_VERSION_R {
        BLK2_EFUSE_VERSION_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:28 - Store rf configuration parameters."]
    #[inline(always)]
    pub fn rf_ref_i_bias_config(&self) -> RF_REF_I_BIAS_CONFIG_R {
        RF_REF_I_BIAS_CONFIG_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bits 29:31 - Store the bit \\[0:2\\] of ido configuration parameters."]
    #[inline(always)]
    pub fn ldo_vol_bias_config_low(&self) -> LDO_VOL_BIAS_CONFIG_LOW_R {
        LDO_VOL_BIAS_CONFIG_LOW_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_BLK2_DATA1")
            .field("mac_id_high", &self.mac_id_high().bits())
            .field("wafer_version", &self.wafer_version().bits())
            .field("pkg_version", &self.pkg_version().bits())
            .field("blk2_efuse_version", &self.blk2_efuse_version().bits())
            .field("rf_ref_i_bias_config", &self.rf_ref_i_bias_config().bits())
            .field(
                "ldo_vol_bias_config_low",
                &self.ldo_vol_bias_config_low().bits(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_BLK2_DATA1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Register 1 of BLOCK2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk2_data1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_BLK2_DATA1_SPEC;
impl crate::RegisterSpec for RD_BLK2_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_blk2_data1::R`](R) reader structure"]
impl crate::Readable for RD_BLK2_DATA1_SPEC {}
#[doc = "`reset()` method sets RD_BLK2_DATA1 to value 0"]
impl crate::Resettable for RD_BLK2_DATA1_SPEC {
    const RESET_VALUE: u32 = 0;
}
