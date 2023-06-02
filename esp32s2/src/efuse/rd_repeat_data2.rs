#[doc = "Register `RD_REPEAT_DATA2` reader"]
pub struct R(crate::R<RD_REPEAT_DATA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_DATA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_DATA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_DATA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEY_PURPOSE_2` reader - Purpose of KEY2. Refer to Table Key Purpose Values."]
pub type KEY_PURPOSE_2_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_3` reader - Purpose of KEY3. Refer to Table Key Purpose Values."]
pub type KEY_PURPOSE_3_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_4` reader - Purpose of KEY4. Refer to Table Key Purpose Values."]
pub type KEY_PURPOSE_4_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_5` reader - Purpose of KEY5. Refer to Table Key Purpose Values."]
pub type KEY_PURPOSE_5_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_6` reader - Purpose of KEY6. Refer to Table Key Purpose Values."]
pub type KEY_PURPOSE_6_R = crate::FieldReader;
#[doc = "Field `SECURE_BOOT_EN` reader - Set this bit to enable secure boot."]
pub type SECURE_BOOT_EN_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_AGGRESSIVE_REVOKE` reader - Set this bit to enable aggressive secure boot key revocation mode."]
pub type SECURE_BOOT_AGGRESSIVE_REVOKE_R = crate::BitReader;
#[doc = "Field `RPT4_RESERVED1` reader - Reserved (used for four backups method)."]
pub type RPT4_RESERVED1_R = crate::FieldReader;
#[doc = "Field `FLASH_TPUW` reader - Configures flash startup delay after SoC power-up, in unit of (ms/2). When the value is 15, delay is 7.5 ms."]
pub type FLASH_TPUW_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Purpose of KEY2. Refer to Table Key Purpose Values."]
    #[inline(always)]
    pub fn key_purpose_2(&self) -> KEY_PURPOSE_2_R {
        KEY_PURPOSE_2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Purpose of KEY3. Refer to Table Key Purpose Values."]
    #[inline(always)]
    pub fn key_purpose_3(&self) -> KEY_PURPOSE_3_R {
        KEY_PURPOSE_3_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Purpose of KEY4. Refer to Table Key Purpose Values."]
    #[inline(always)]
    pub fn key_purpose_4(&self) -> KEY_PURPOSE_4_R {
        KEY_PURPOSE_4_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Purpose of KEY5. Refer to Table Key Purpose Values."]
    #[inline(always)]
    pub fn key_purpose_5(&self) -> KEY_PURPOSE_5_R {
        KEY_PURPOSE_5_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Purpose of KEY6. Refer to Table Key Purpose Values."]
    #[inline(always)]
    pub fn key_purpose_6(&self) -> KEY_PURPOSE_6_R {
        KEY_PURPOSE_6_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Set this bit to enable secure boot."]
    #[inline(always)]
    pub fn secure_boot_en(&self) -> SECURE_BOOT_EN_R {
        SECURE_BOOT_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to enable aggressive secure boot key revocation mode."]
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke(&self) -> SECURE_BOOT_AGGRESSIVE_REVOKE_R {
        SECURE_BOOT_AGGRESSIVE_REVOKE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:27 - Reserved (used for four backups method)."]
    #[inline(always)]
    pub fn rpt4_reserved1(&self) -> RPT4_RESERVED1_R {
        RPT4_RESERVED1_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bits 28:31 - Configures flash startup delay after SoC power-up, in unit of (ms/2). When the value is 15, delay is 7.5 ms."]
    #[inline(always)]
    pub fn flash_tpuw(&self) -> FLASH_TPUW_R {
        FLASH_TPUW_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA2")
            .field(
                "key_purpose_2",
                &format_args!("{}", self.key_purpose_2().bits()),
            )
            .field(
                "key_purpose_3",
                &format_args!("{}", self.key_purpose_3().bits()),
            )
            .field(
                "key_purpose_4",
                &format_args!("{}", self.key_purpose_4().bits()),
            )
            .field(
                "key_purpose_5",
                &format_args!("{}", self.key_purpose_5().bits()),
            )
            .field(
                "key_purpose_6",
                &format_args!("{}", self.key_purpose_6().bits()),
            )
            .field(
                "secure_boot_en",
                &format_args!("{}", self.secure_boot_en().bit()),
            )
            .field(
                "secure_boot_aggressive_revoke",
                &format_args!("{}", self.secure_boot_aggressive_revoke().bit()),
            )
            .field(
                "rpt4_reserved1",
                &format_args!("{}", self.rpt4_reserved1().bits()),
            )
            .field("flash_tpuw", &format_args!("{}", self.flash_tpuw().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_REPEAT_DATA2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Register 3 of BLOCK0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_data2](index.html) module"]
pub struct RD_REPEAT_DATA2_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_data2::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_DATA2 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
