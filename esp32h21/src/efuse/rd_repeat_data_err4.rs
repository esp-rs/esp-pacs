#[doc = "Register `RD_REPEAT_DATA_ERR4` reader"]
pub type R = crate::R<RD_REPEAT_DATA_ERR4_SPEC>;
#[doc = "Field `HYS_EN_PAD1_ERR` reader - Represents the programming error of EFUSE_HYS_EN_PAD1"]
pub type HYS_EN_PAD1_ERR_R = crate::FieldReader<u32>;
#[doc = "Field `FLASH_LDO_POWER_SEL_ERR` reader - Represents the programming error of EFUSE_FLASH_LDO_POWER_SEL"]
pub type FLASH_LDO_POWER_SEL_ERR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:21 - Represents the programming error of EFUSE_HYS_EN_PAD1"]
    #[inline(always)]
    pub fn hys_en_pad1_err(&self) -> HYS_EN_PAD1_ERR_R {
        HYS_EN_PAD1_ERR_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bit 22 - Represents the programming error of EFUSE_FLASH_LDO_POWER_SEL"]
    #[inline(always)]
    pub fn flash_ldo_power_sel_err(&self) -> FLASH_LDO_POWER_SEL_ERR_R {
        FLASH_LDO_POWER_SEL_ERR_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA_ERR4")
            .field("hys_en_pad1_err", &self.hys_en_pad1_err())
            .field("flash_ldo_power_sel_err", &self.flash_ldo_power_sel_err())
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
