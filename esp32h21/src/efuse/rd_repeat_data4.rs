#[doc = "Register `RD_REPEAT_DATA4` reader"]
pub type R = crate::R<RD_REPEAT_DATA4_SPEC>;
#[doc = "Field `HYS_EN_PAD1` reader - Represents whether to enable the hysteresis function of pad 6-27.\\\\ 0: Disabled\\\\ 1: Enabled\\\\"]
pub type HYS_EN_PAD1_R = crate::FieldReader<u32>;
#[doc = "Field `FLASH_LDO_POWER_SEL` reader - Represents which flash LDO is selected.\\\\ 0: FLASH LDO 1P8.\\\\ 1: FLASH LDO 1P2.\\\\"]
pub type FLASH_LDO_POWER_SEL_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:21 - Represents whether to enable the hysteresis function of pad 6-27.\\\\ 0: Disabled\\\\ 1: Enabled\\\\"]
    #[inline(always)]
    pub fn hys_en_pad1(&self) -> HYS_EN_PAD1_R {
        HYS_EN_PAD1_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bit 22 - Represents which flash LDO is selected.\\\\ 0: FLASH LDO 1P8.\\\\ 1: FLASH LDO 1P2.\\\\"]
    #[inline(always)]
    pub fn flash_ldo_power_sel(&self) -> FLASH_LDO_POWER_SEL_R {
        FLASH_LDO_POWER_SEL_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA4")
            .field("hys_en_pad1", &self.hys_en_pad1())
            .field("flash_ldo_power_sel", &self.flash_ldo_power_sel())
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
