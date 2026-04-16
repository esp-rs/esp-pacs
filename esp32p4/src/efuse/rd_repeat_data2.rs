#[doc = "Register `RD_REPEAT_DATA2` reader"]
pub type R = crate::R<RD_REPEAT_DATA2_SPEC>;
#[doc = "Field `KEY_PURPOSE_2` reader - Purpose of Key2."]
pub type KEY_PURPOSE_2_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_3` reader - Purpose of Key3."]
pub type KEY_PURPOSE_3_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_4` reader - Purpose of Key4."]
pub type KEY_PURPOSE_4_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_5` reader - Purpose of Key5."]
pub type KEY_PURPOSE_5_R = crate::FieldReader;
#[doc = "Field `SEC_DPA_LEVEL` reader - Configures the clock random divide mode to determine the dpa secure level"]
pub type SEC_DPA_LEVEL_R = crate::FieldReader;
#[doc = "Field `XTS_DPA_CLK_ENABLE` reader - Sets this bit to enable xts clock anti-dpa attack function."]
pub type XTS_DPA_CLK_ENABLE_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_EN` reader - Set this bit to enable secure boot."]
pub type SECURE_BOOT_EN_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_AGGRESSIVE_REVOKE` reader - Set this bit to enable revoking aggressive secure boot."]
pub type SECURE_BOOT_AGGRESSIVE_REVOKE_R = crate::BitReader;
#[doc = "Field `KM_DEPLOY_ONLY_ONCE_H` reader - EFUSE_KM_DEPLOY_ONLY_ONCE and EFUSE_KM_DEPLOY_ONLY_ONCE_H together form one field: {EFUSE_KM_DEPLOY_ONLY_ONCE_H, EFUSE_KM_DEPLOY_ONLY_ONCE\\[3:0\\]}. Set each bit to control whether corresponding key can only be deployed once. 1 is true, 0 is false. bit 0: ecsda, bit 1: xts, bit2: hmac, bit3: ds, bit4:psram"]
pub type KM_DEPLOY_ONLY_ONCE_H_R = crate::BitReader;
#[doc = "Field `FORCE_USE_KEY_MANAGER_KEY_H` reader - EFUSE_FORCE_USE_KEY_MANAGER_KEY and EFUSE_FORCE_USE_KEY_MANAGER_KEY_H together form one field: {EFUSE_FORCE_USE_KEY_MANAGER_KEY_H, EFUSE_FORCE_USE_KEY_MANAGER_KEY\\[3:0\\]}. Set each bit to control whether corresponding key must come from key manager. 1 is true, 0 is false. bit 0: ecsda, bit 1: xts, bit2: hmac, bit3: ds, bit4:psram"]
pub type FORCE_USE_KEY_MANAGER_KEY_H_R = crate::BitReader;
#[doc = "Field `FLASH_ECC_EN` reader - Set this bit to enable ECC for flash boot."]
pub type FLASH_ECC_EN_R = crate::BitReader;
#[doc = "Field `DIS_USB_OTG_DOWNLOAD_MODE` reader - Set this bit to disable download via USB-OTG."]
pub type DIS_USB_OTG_DOWNLOAD_MODE_R = crate::BitReader;
#[doc = "Field `FLASH_TPUW` reader - Configures flash waiting time after power-up, in unit of ms. When the value less than 15, the waiting time is the configurable value. Otherwise, the waiting time is 30."]
pub type FLASH_TPUW_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Purpose of Key2."]
    #[inline(always)]
    pub fn key_purpose_2(&self) -> KEY_PURPOSE_2_R {
        KEY_PURPOSE_2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Purpose of Key3."]
    #[inline(always)]
    pub fn key_purpose_3(&self) -> KEY_PURPOSE_3_R {
        KEY_PURPOSE_3_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Purpose of Key4."]
    #[inline(always)]
    pub fn key_purpose_4(&self) -> KEY_PURPOSE_4_R {
        KEY_PURPOSE_4_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Purpose of Key5."]
    #[inline(always)]
    pub fn key_purpose_5(&self) -> KEY_PURPOSE_5_R {
        KEY_PURPOSE_5_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Configures the clock random divide mode to determine the dpa secure level"]
    #[inline(always)]
    pub fn sec_dpa_level(&self) -> SEC_DPA_LEVEL_R {
        SEC_DPA_LEVEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - Sets this bit to enable xts clock anti-dpa attack function."]
    #[inline(always)]
    pub fn xts_dpa_clk_enable(&self) -> XTS_DPA_CLK_ENABLE_R {
        XTS_DPA_CLK_ENABLE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to enable secure boot."]
    #[inline(always)]
    pub fn secure_boot_en(&self) -> SECURE_BOOT_EN_R {
        SECURE_BOOT_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to enable revoking aggressive secure boot."]
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke(&self) -> SECURE_BOOT_AGGRESSIVE_REVOKE_R {
        SECURE_BOOT_AGGRESSIVE_REVOKE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - EFUSE_KM_DEPLOY_ONLY_ONCE and EFUSE_KM_DEPLOY_ONLY_ONCE_H together form one field: {EFUSE_KM_DEPLOY_ONLY_ONCE_H, EFUSE_KM_DEPLOY_ONLY_ONCE\\[3:0\\]}. Set each bit to control whether corresponding key can only be deployed once. 1 is true, 0 is false. bit 0: ecsda, bit 1: xts, bit2: hmac, bit3: ds, bit4:psram"]
    #[inline(always)]
    pub fn km_deploy_only_once_h(&self) -> KM_DEPLOY_ONLY_ONCE_H_R {
        KM_DEPLOY_ONLY_ONCE_H_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - EFUSE_FORCE_USE_KEY_MANAGER_KEY and EFUSE_FORCE_USE_KEY_MANAGER_KEY_H together form one field: {EFUSE_FORCE_USE_KEY_MANAGER_KEY_H, EFUSE_FORCE_USE_KEY_MANAGER_KEY\\[3:0\\]}. Set each bit to control whether corresponding key must come from key manager. 1 is true, 0 is false. bit 0: ecsda, bit 1: xts, bit2: hmac, bit3: ds, bit4:psram"]
    #[inline(always)]
    pub fn force_use_key_manager_key_h(&self) -> FORCE_USE_KEY_MANAGER_KEY_H_R {
        FORCE_USE_KEY_MANAGER_KEY_H_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to enable ECC for flash boot."]
    #[inline(always)]
    pub fn flash_ecc_en(&self) -> FLASH_ECC_EN_R {
        FLASH_ECC_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Set this bit to disable download via USB-OTG."]
    #[inline(always)]
    pub fn dis_usb_otg_download_mode(&self) -> DIS_USB_OTG_DOWNLOAD_MODE_R {
        DIS_USB_OTG_DOWNLOAD_MODE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Configures flash waiting time after power-up, in unit of ms. When the value less than 15, the waiting time is the configurable value. Otherwise, the waiting time is 30."]
    #[inline(always)]
    pub fn flash_tpuw(&self) -> FLASH_TPUW_R {
        FLASH_TPUW_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA2")
            .field("key_purpose_2", &self.key_purpose_2())
            .field("key_purpose_3", &self.key_purpose_3())
            .field("key_purpose_4", &self.key_purpose_4())
            .field("key_purpose_5", &self.key_purpose_5())
            .field("sec_dpa_level", &self.sec_dpa_level())
            .field("xts_dpa_clk_enable", &self.xts_dpa_clk_enable())
            .field("secure_boot_en", &self.secure_boot_en())
            .field(
                "secure_boot_aggressive_revoke",
                &self.secure_boot_aggressive_revoke(),
            )
            .field("km_deploy_only_once_h", &self.km_deploy_only_once_h())
            .field(
                "force_use_key_manager_key_h",
                &self.force_use_key_manager_key_h(),
            )
            .field("flash_ecc_en", &self.flash_ecc_en())
            .field(
                "dis_usb_otg_download_mode",
                &self.dis_usb_otg_download_mode(),
            )
            .field("flash_tpuw", &self.flash_tpuw())
            .finish()
    }
}
#[doc = "Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA2_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data2::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA2_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA2 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA2_SPEC {}
