#[doc = "Register `RD_REPEAT_DATA_ERR4` reader"]
pub type R = crate::R<RD_REPEAT_DATA_ERR4_SPEC>;
#[doc = "Field `_0PXA_TIEH_SEL_0_ERR` reader - Represents the programming error of EFUSE_0PXA_TIEH_SEL_0"]
pub type _0PXA_TIEH_SEL_0_ERR_R = crate::FieldReader;
#[doc = "Field `PVT_GLITCH_EN_ERR` reader - Represents the programming error of EFUSE_PVT_GLITCH_EN"]
pub type PVT_GLITCH_EN_ERR_R = crate::BitReader;
#[doc = "Field `KEY_PURPOSE_5_H_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_5_H"]
pub type KEY_PURPOSE_5_H_ERR_R = crate::BitReader;
#[doc = "Field `KM_DISABLE_DEPLOY_MODE_H_ERR` reader - Represents the programming error of EFUSE_KM_DISABLE_DEPLOY_MODE_H"]
pub type KM_DISABLE_DEPLOY_MODE_H_ERR_R = crate::BitReader;
#[doc = "Field `KM_DISABLE_DEPLOY_MODE_ERR` reader - Represents the programming error of EFUSE_KM_DISABLE_DEPLOY_MODE"]
pub type KM_DISABLE_DEPLOY_MODE_ERR_R = crate::FieldReader;
#[doc = "Field `XTS_DPA_PSEUDO_LEVEL_ERR` reader - Represents the programming error of EFUSE_XTS_DPA_PSEUDO_LEVEL"]
pub type XTS_DPA_PSEUDO_LEVEL_ERR_R = crate::FieldReader;
#[doc = "Field `HP_PWR_SRC_SEL_ERR` reader - Represents the programming error of EFUSE_HP_PWR_SRC_SEL"]
pub type HP_PWR_SRC_SEL_ERR_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_SHA384_EN_ERR` reader - Represents the programming error of EFUSE_SECURE_BOOT_SHA384_EN"]
pub type SECURE_BOOT_SHA384_EN_ERR_R = crate::BitReader;
#[doc = "Field `DIS_WDT_ERR` reader - Represents the programming error of EFUSE_DIS_WDT"]
pub type DIS_WDT_ERR_R = crate::BitReader;
#[doc = "Field `DIS_SWD_ERR` reader - Represents the programming error of EFUSE_DIS_SWD"]
pub type DIS_SWD_ERR_R = crate::BitReader;
#[doc = "Field `PVT_GLITCH_MODE_ERR` reader - Represents the programming error of EFUSE_PVT_GLITCH_MODE"]
pub type PVT_GLITCH_MODE_ERR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Represents the programming error of EFUSE_0PXA_TIEH_SEL_0"]
    #[inline(always)]
    pub fn _0pxa_tieh_sel_0_err(&self) -> _0PXA_TIEH_SEL_0_ERR_R {
        _0PXA_TIEH_SEL_0_ERR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Represents the programming error of EFUSE_PVT_GLITCH_EN"]
    #[inline(always)]
    pub fn pvt_glitch_en_err(&self) -> PVT_GLITCH_EN_ERR_R {
        PVT_GLITCH_EN_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents the programming error of EFUSE_KEY_PURPOSE_5_H"]
    #[inline(always)]
    pub fn key_purpose_5_h_err(&self) -> KEY_PURPOSE_5_H_ERR_R {
        KEY_PURPOSE_5_H_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents the programming error of EFUSE_KM_DISABLE_DEPLOY_MODE_H"]
    #[inline(always)]
    pub fn km_disable_deploy_mode_h_err(&self) -> KM_DISABLE_DEPLOY_MODE_H_ERR_R {
        KM_DISABLE_DEPLOY_MODE_H_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Represents the programming error of EFUSE_KM_DISABLE_DEPLOY_MODE"]
    #[inline(always)]
    pub fn km_disable_deploy_mode_err(&self) -> KM_DISABLE_DEPLOY_MODE_ERR_R {
        KM_DISABLE_DEPLOY_MODE_ERR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Represents the programming error of EFUSE_XTS_DPA_PSEUDO_LEVEL"]
    #[inline(always)]
    pub fn xts_dpa_pseudo_level_err(&self) -> XTS_DPA_PSEUDO_LEVEL_ERR_R {
        XTS_DPA_PSEUDO_LEVEL_ERR_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Represents the programming error of EFUSE_HP_PWR_SRC_SEL"]
    #[inline(always)]
    pub fn hp_pwr_src_sel_err(&self) -> HP_PWR_SRC_SEL_ERR_R {
        HP_PWR_SRC_SEL_ERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Represents the programming error of EFUSE_SECURE_BOOT_SHA384_EN"]
    #[inline(always)]
    pub fn secure_boot_sha384_en_err(&self) -> SECURE_BOOT_SHA384_EN_ERR_R {
        SECURE_BOOT_SHA384_EN_ERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents the programming error of EFUSE_DIS_WDT"]
    #[inline(always)]
    pub fn dis_wdt_err(&self) -> DIS_WDT_ERR_R {
        DIS_WDT_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents the programming error of EFUSE_DIS_SWD"]
    #[inline(always)]
    pub fn dis_swd_err(&self) -> DIS_SWD_ERR_R {
        DIS_SWD_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Represents the programming error of EFUSE_PVT_GLITCH_MODE"]
    #[inline(always)]
    pub fn pvt_glitch_mode_err(&self) -> PVT_GLITCH_MODE_ERR_R {
        PVT_GLITCH_MODE_ERR_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA_ERR4")
            .field("_0pxa_tieh_sel_0_err", &self._0pxa_tieh_sel_0_err())
            .field("pvt_glitch_en_err", &self.pvt_glitch_en_err())
            .field("key_purpose_5_h_err", &self.key_purpose_5_h_err())
            .field(
                "km_disable_deploy_mode_h_err",
                &self.km_disable_deploy_mode_h_err(),
            )
            .field(
                "km_disable_deploy_mode_err",
                &self.km_disable_deploy_mode_err(),
            )
            .field("xts_dpa_pseudo_level_err", &self.xts_dpa_pseudo_level_err())
            .field("hp_pwr_src_sel_err", &self.hp_pwr_src_sel_err())
            .field(
                "secure_boot_sha384_en_err",
                &self.secure_boot_sha384_en_err(),
            )
            .field("dis_wdt_err", &self.dis_wdt_err())
            .field("dis_swd_err", &self.dis_swd_err())
            .field("pvt_glitch_mode_err", &self.pvt_glitch_mode_err())
            .finish()
    }
}
#[doc = "Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA_ERR4_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA_ERR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data_err4::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA_ERR4_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA_ERR4 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA_ERR4_SPEC {}
