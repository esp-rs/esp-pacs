///Register `RD_REPEAT_DATA2` reader
pub type R = crate::R<RD_REPEAT_DATA2_SPEC>;
///Field `KEY_PURPOSE_2` reader - Purpose of KEY2. Refer to Table Key Purpose Values.
pub type KEY_PURPOSE_2_R = crate::FieldReader;
///Field `KEY_PURPOSE_3` reader - Purpose of KEY3. Refer to Table Key Purpose Values.
pub type KEY_PURPOSE_3_R = crate::FieldReader;
///Field `KEY_PURPOSE_4` reader - Purpose of KEY4. Refer to Table Key Purpose Values.
pub type KEY_PURPOSE_4_R = crate::FieldReader;
///Field `KEY_PURPOSE_5` reader - Purpose of KEY5. Refer to Table Key Purpose Values.
pub type KEY_PURPOSE_5_R = crate::FieldReader;
///Field `KEY_PURPOSE_6` reader - Purpose of KEY6. Refer to Table Key Purpose Values.
pub type KEY_PURPOSE_6_R = crate::FieldReader;
///Field `SECURE_BOOT_EN` reader - Set this bit to enable secure boot.
pub type SECURE_BOOT_EN_R = crate::BitReader;
///Field `SECURE_BOOT_AGGRESSIVE_REVOKE` reader - Set this bit to enable aggressive secure boot key revocation mode.
pub type SECURE_BOOT_AGGRESSIVE_REVOKE_R = crate::BitReader;
///Field `RPT4_RESERVED1` reader - Reserved (used for four backups method).
pub type RPT4_RESERVED1_R = crate::FieldReader;
///Field `FLASH_TPUW` reader - Configures flash startup delay after SoC power-up, in unit of (ms/2). When the value is 15, delay is 7.5 ms.
pub type FLASH_TPUW_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - Purpose of KEY2. Refer to Table Key Purpose Values.
    #[inline(always)]
    pub fn key_purpose_2(&self) -> KEY_PURPOSE_2_R {
        KEY_PURPOSE_2_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Purpose of KEY3. Refer to Table Key Purpose Values.
    #[inline(always)]
    pub fn key_purpose_3(&self) -> KEY_PURPOSE_3_R {
        KEY_PURPOSE_3_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Purpose of KEY4. Refer to Table Key Purpose Values.
    #[inline(always)]
    pub fn key_purpose_4(&self) -> KEY_PURPOSE_4_R {
        KEY_PURPOSE_4_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Purpose of KEY5. Refer to Table Key Purpose Values.
    #[inline(always)]
    pub fn key_purpose_5(&self) -> KEY_PURPOSE_5_R {
        KEY_PURPOSE_5_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Purpose of KEY6. Refer to Table Key Purpose Values.
    #[inline(always)]
    pub fn key_purpose_6(&self) -> KEY_PURPOSE_6_R {
        KEY_PURPOSE_6_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 20 - Set this bit to enable secure boot.
    #[inline(always)]
    pub fn secure_boot_en(&self) -> SECURE_BOOT_EN_R {
        SECURE_BOOT_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Set this bit to enable aggressive secure boot key revocation mode.
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke(&self) -> SECURE_BOOT_AGGRESSIVE_REVOKE_R {
        SECURE_BOOT_AGGRESSIVE_REVOKE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:27 - Reserved (used for four backups method).
    #[inline(always)]
    pub fn rpt4_reserved1(&self) -> RPT4_RESERVED1_R {
        RPT4_RESERVED1_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    ///Bits 28:31 - Configures flash startup delay after SoC power-up, in unit of (ms/2). When the value is 15, delay is 7.5 ms.
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
            .field("key_purpose_6", &self.key_purpose_6())
            .field("secure_boot_en", &self.secure_boot_en())
            .field(
                "secure_boot_aggressive_revoke",
                &self.secure_boot_aggressive_revoke(),
            )
            .field("rpt4_reserved1", &self.rpt4_reserved1())
            .field("flash_tpuw", &self.flash_tpuw())
            .finish()
    }
}
/**Register 3 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_REPEAT_DATA2_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_repeat_data2::R`](R) reader structure
impl crate::Readable for RD_REPEAT_DATA2_SPEC {}
///`reset()` method sets RD_REPEAT_DATA2 to value 0
impl crate::Resettable for RD_REPEAT_DATA2_SPEC {
    const RESET_VALUE: u32 = 0;
}
