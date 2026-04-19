#[doc = "Register `RD_REPEAT_DATA4` reader"]
pub type R = crate::R<RD_REPEAT_DATA4_SPEC>;
#[doc = "Field `_0PXA_TIEH_SEL_0` reader - Output LDO VO0 tieh source select. 0: 1'b1 1: sdmmc1 2: reg 3:sdmmc0"]
pub type _0PXA_TIEH_SEL_0_R = crate::FieldReader;
#[doc = "Field `PVT_GLITCH_EN` reader - Represents whether to enable PVT power glitch monitor function.\\\\1:Enable. \\\\0:Disable"]
pub type PVT_GLITCH_EN_R = crate::BitReader;
#[doc = "Field `KEY_PURPOSE_5_H` reader - Purpose of Key5. The 5-th bit."]
pub type KEY_PURPOSE_5_H_R = crate::BitReader;
#[doc = "Field `KM_DISABLE_DEPLOY_MODE_H` reader - EFUSE_KM_DISABLE_DEPLOY_MODE and EFUSE_KM_DISABLE_DEPLOY_MODE_H together form one field: {EFUSE_KM_DISABLE_DEPLOY_MODE_H, EFUSE_KM_DISABLE_DEPLOY_MODE\\[3:0\\]}. Set each bit to control whether corresponding key's deploy mode of new value deployment is disabled. 1 is true, 0 is false. bit 0: ecsda, bit 1: xts, bit2: hmac, bit3: ds, bit4:psram"]
pub type KM_DISABLE_DEPLOY_MODE_H_R = crate::BitReader;
#[doc = "Field `KM_DISABLE_DEPLOY_MODE` reader - EFUSE_KM_DISABLE_DEPLOY_MODE and EFUSE_KM_DISABLE_DEPLOY_MODE_H together form one field: {EFUSE_KM_DISABLE_DEPLOY_MODE_H, EFUSE_KM_DISABLE_DEPLOY_MODE\\[3:0\\]}. Set each bit to control whether corresponding key's deploy mode of new value deployment is disabled. 1 is true, 0 is false. bit 0: ecsda, bit 1: xts, bit2: hmac, bit3: ds, bit4:psram"]
pub type KM_DISABLE_DEPLOY_MODE_R = crate::FieldReader;
#[doc = "Field `XTS_DPA_PSEUDO_LEVEL` reader - Sets this bit to control the xts pseudo-round anti-dpa attack function. 0: controlled by register. 1-3: the higer the value is, the more pseudo-rounds are inserted to the xts-aes calculation"]
pub type XTS_DPA_PSEUDO_LEVEL_R = crate::FieldReader;
#[doc = "Field `HP_PWR_SRC_SEL` reader - HP system power source select. 0:LDO 1: DCDC"]
pub type HP_PWR_SRC_SEL_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_SHA384_EN` reader - Represents whether secure boot using SHA-384 is enabled. 0: disable 1: enable"]
pub type SECURE_BOOT_SHA384_EN_R = crate::BitReader;
#[doc = "Field `DIS_WDT` reader - Set this bit to disable watch dog."]
pub type DIS_WDT_R = crate::BitReader;
#[doc = "Field `DIS_SWD` reader - Set bit to disable super-watchdog"]
pub type DIS_SWD_R = crate::BitReader;
#[doc = "Field `PVT_GLITCH_MODE` reader - Use to configure glitch mode"]
pub type PVT_GLITCH_MODE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Output LDO VO0 tieh source select. 0: 1'b1 1: sdmmc1 2: reg 3:sdmmc0"]
    #[inline(always)]
    pub fn _0pxa_tieh_sel_0(&self) -> _0PXA_TIEH_SEL_0_R {
        _0PXA_TIEH_SEL_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Represents whether to enable PVT power glitch monitor function.\\\\1:Enable. \\\\0:Disable"]
    #[inline(always)]
    pub fn pvt_glitch_en(&self) -> PVT_GLITCH_EN_R {
        PVT_GLITCH_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Purpose of Key5. The 5-th bit."]
    #[inline(always)]
    pub fn key_purpose_5_h(&self) -> KEY_PURPOSE_5_H_R {
        KEY_PURPOSE_5_H_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - EFUSE_KM_DISABLE_DEPLOY_MODE and EFUSE_KM_DISABLE_DEPLOY_MODE_H together form one field: {EFUSE_KM_DISABLE_DEPLOY_MODE_H, EFUSE_KM_DISABLE_DEPLOY_MODE\\[3:0\\]}. Set each bit to control whether corresponding key's deploy mode of new value deployment is disabled. 1 is true, 0 is false. bit 0: ecsda, bit 1: xts, bit2: hmac, bit3: ds, bit4:psram"]
    #[inline(always)]
    pub fn km_disable_deploy_mode_h(&self) -> KM_DISABLE_DEPLOY_MODE_H_R {
        KM_DISABLE_DEPLOY_MODE_H_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - EFUSE_KM_DISABLE_DEPLOY_MODE and EFUSE_KM_DISABLE_DEPLOY_MODE_H together form one field: {EFUSE_KM_DISABLE_DEPLOY_MODE_H, EFUSE_KM_DISABLE_DEPLOY_MODE\\[3:0\\]}. Set each bit to control whether corresponding key's deploy mode of new value deployment is disabled. 1 is true, 0 is false. bit 0: ecsda, bit 1: xts, bit2: hmac, bit3: ds, bit4:psram"]
    #[inline(always)]
    pub fn km_disable_deploy_mode(&self) -> KM_DISABLE_DEPLOY_MODE_R {
        KM_DISABLE_DEPLOY_MODE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Sets this bit to control the xts pseudo-round anti-dpa attack function. 0: controlled by register. 1-3: the higer the value is, the more pseudo-rounds are inserted to the xts-aes calculation"]
    #[inline(always)]
    pub fn xts_dpa_pseudo_level(&self) -> XTS_DPA_PSEUDO_LEVEL_R {
        XTS_DPA_PSEUDO_LEVEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - HP system power source select. 0:LDO 1: DCDC"]
    #[inline(always)]
    pub fn hp_pwr_src_sel(&self) -> HP_PWR_SRC_SEL_R {
        HP_PWR_SRC_SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Represents whether secure boot using SHA-384 is enabled. 0: disable 1: enable"]
    #[inline(always)]
    pub fn secure_boot_sha384_en(&self) -> SECURE_BOOT_SHA384_EN_R {
        SECURE_BOOT_SHA384_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to disable watch dog."]
    #[inline(always)]
    pub fn dis_wdt(&self) -> DIS_WDT_R {
        DIS_WDT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set bit to disable super-watchdog"]
    #[inline(always)]
    pub fn dis_swd(&self) -> DIS_SWD_R {
        DIS_SWD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Use to configure glitch mode"]
    #[inline(always)]
    pub fn pvt_glitch_mode(&self) -> PVT_GLITCH_MODE_R {
        PVT_GLITCH_MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA4")
            .field("_0pxa_tieh_sel_0", &self._0pxa_tieh_sel_0())
            .field("pvt_glitch_en", &self.pvt_glitch_en())
            .field("key_purpose_5_h", &self.key_purpose_5_h())
            .field("km_disable_deploy_mode_h", &self.km_disable_deploy_mode_h())
            .field("km_disable_deploy_mode", &self.km_disable_deploy_mode())
            .field("xts_dpa_pseudo_level", &self.xts_dpa_pseudo_level())
            .field("hp_pwr_src_sel", &self.hp_pwr_src_sel())
            .field("secure_boot_sha384_en", &self.secure_boot_sha384_en())
            .field("dis_wdt", &self.dis_wdt())
            .field("dis_swd", &self.dis_swd())
            .field("pvt_glitch_mode", &self.pvt_glitch_mode())
            .finish()
    }
}
#[doc = "Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA4_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data4::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA4_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA4 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA4_SPEC {}
