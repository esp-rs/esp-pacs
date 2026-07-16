#[doc = "Register `RD_REPEAT_DATA_ERR4` reader"]
pub type R = crate::R<RD_REPEAT_DATA_ERR4_SPEC>;
#[doc = "Field `SECURE_VERSION_ERR` reader - Represents the programming error of EFUSE_SECURE_VERSION"]
pub type SECURE_VERSION_ERR_R = crate::FieldReader<u16>;
#[doc = "Field `SECURE_BOOT_DISABLE_FAST_WAKE_ERR` reader - Represents the programming error of EFUSE_SECURE_BOOT_DISABLE_FAST_WAKE"]
pub type SECURE_BOOT_DISABLE_FAST_WAKE_ERR_R = crate::BitReader;
#[doc = "Field `HYS_EN_PAD_ERR` reader - Represents the programming error of EFUSE_HYS_EN_PAD"]
pub type HYS_EN_PAD_ERR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Represents the programming error of EFUSE_SECURE_VERSION"]
    #[inline(always)]
    pub fn secure_version_err(&self) -> SECURE_VERSION_ERR_R {
        SECURE_VERSION_ERR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Represents the programming error of EFUSE_SECURE_BOOT_DISABLE_FAST_WAKE"]
    #[inline(always)]
    pub fn secure_boot_disable_fast_wake_err(&self) -> SECURE_BOOT_DISABLE_FAST_WAKE_ERR_R {
        SECURE_BOOT_DISABLE_FAST_WAKE_ERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents the programming error of EFUSE_HYS_EN_PAD"]
    #[inline(always)]
    pub fn hys_en_pad_err(&self) -> HYS_EN_PAD_ERR_R {
        HYS_EN_PAD_ERR_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA_ERR4")
            .field("secure_version_err", &self.secure_version_err())
            .field(
                "secure_boot_disable_fast_wake_err",
                &self.secure_boot_disable_fast_wake_err(),
            )
            .field("hys_en_pad_err", &self.hys_en_pad_err())
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
