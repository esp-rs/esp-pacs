#[doc = "Register `RD_MAC_SYS3` reader"]
pub type R = crate::R<RD_MAC_SYS3_SPEC>;
#[doc = "Field `PUMP_DRV` reader - Use to configure charge pump voltage gain"]
pub type PUMP_DRV_R = crate::FieldReader;
#[doc = "Field `WDT_DELAY_SEL` reader - Represents the threshold level of the RTC watchdog STG0 timeout.\\\\ 0: Original threshold configuration value of STG0 *2 \\\\1: Original threshold configuration value of STG0 *4 \\\\2: Original threshold configuration value of STG0 *8 \\\\3: Original threshold configuration value of STG0 *16 \\\\"]
pub type WDT_DELAY_SEL_R = crate::FieldReader;
#[doc = "Field `HYS_EN_PAD` reader - Represents whether the hysteresis function of corresponding PAD is enabled.\\\\ 1: enabled\\\\ 0:disabled\\\\"]
pub type HYS_EN_PAD_R = crate::BitReader;
#[doc = "Field `PVT_GLITCH_CHARGE_RESET` reader - Represents whether to trigger reset or charge pump when PVT power glitch happened.\\\\1:Trigger charge pump. \\\\0:Trigger reset"]
pub type PVT_GLITCH_CHARGE_RESET_R = crate::BitReader;
#[doc = "Field `VDD_SPI_LDO_ADJUST` reader - Represents configuration of FLASH LDO mode and voltage.\\\\"]
pub type VDD_SPI_LDO_ADJUST_R = crate::FieldReader;
#[doc = "Field `FLASH_LDO_POWER_SEL` reader - Represents which flash ldo be select:\\\\ 1: FLASH LDO 1P2\\\\ 0 : FLASH LDO 1P8\\\\"]
pub type FLASH_LDO_POWER_SEL_R = crate::BitReader;
#[doc = "Field `SYS_DATA_PART0_0` reader - Represents the first 14-bit of zeroth part of system data."]
pub type SYS_DATA_PART0_0_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Use to configure charge pump voltage gain"]
    #[inline(always)]
    pub fn pump_drv(&self) -> PUMP_DRV_R {
        PUMP_DRV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Represents the threshold level of the RTC watchdog STG0 timeout.\\\\ 0: Original threshold configuration value of STG0 *2 \\\\1: Original threshold configuration value of STG0 *4 \\\\2: Original threshold configuration value of STG0 *8 \\\\3: Original threshold configuration value of STG0 *16 \\\\"]
    #[inline(always)]
    pub fn wdt_delay_sel(&self) -> WDT_DELAY_SEL_R {
        WDT_DELAY_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Represents whether the hysteresis function of corresponding PAD is enabled.\\\\ 1: enabled\\\\ 0:disabled\\\\"]
    #[inline(always)]
    pub fn hys_en_pad(&self) -> HYS_EN_PAD_R {
        HYS_EN_PAD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents whether to trigger reset or charge pump when PVT power glitch happened.\\\\1:Trigger charge pump. \\\\0:Trigger reset"]
    #[inline(always)]
    pub fn pvt_glitch_charge_reset(&self) -> PVT_GLITCH_CHARGE_RESET_R {
        PVT_GLITCH_CHARGE_RESET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 9:16 - Represents configuration of FLASH LDO mode and voltage.\\\\"]
    #[inline(always)]
    pub fn vdd_spi_ldo_adjust(&self) -> VDD_SPI_LDO_ADJUST_R {
        VDD_SPI_LDO_ADJUST_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bit 17 - Represents which flash ldo be select:\\\\ 1: FLASH LDO 1P2\\\\ 0 : FLASH LDO 1P8\\\\"]
    #[inline(always)]
    pub fn flash_ldo_power_sel(&self) -> FLASH_LDO_POWER_SEL_R {
        FLASH_LDO_POWER_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - Represents the first 14-bit of zeroth part of system data."]
    #[inline(always)]
    pub fn sys_data_part0_0(&self) -> SYS_DATA_PART0_0_R {
        SYS_DATA_PART0_0_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SYS3")
            .field("pump_drv", &self.pump_drv())
            .field("wdt_delay_sel", &self.wdt_delay_sel())
            .field("hys_en_pad", &self.hys_en_pad())
            .field("pvt_glitch_charge_reset", &self.pvt_glitch_charge_reset())
            .field("vdd_spi_ldo_adjust", &self.vdd_spi_ldo_adjust())
            .field("flash_ldo_power_sel", &self.flash_ldo_power_sel())
            .field("sys_data_part0_0", &self.sys_data_part0_0())
            .finish()
    }
}
#[doc = "Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_MAC_SYS3_SPEC;
impl crate::RegisterSpec for RD_MAC_SYS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_mac_sys3::R`](R) reader structure"]
impl crate::Readable for RD_MAC_SYS3_SPEC {}
#[doc = "`reset()` method sets RD_MAC_SYS3 to value 0"]
impl crate::Resettable for RD_MAC_SYS3_SPEC {}
