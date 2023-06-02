#[doc = "Register `RD_REPEAT_DATA0` reader"]
pub struct R(crate::R<RD_REPEAT_DATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_DATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_DATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_DATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_DIS` reader - Disables software reading from individual eFuse blocks (BLOCK4-10)."]
pub type RD_DIS_R = crate::FieldReader;
#[doc = "Field `DIS_RTC_RAM_BOOT` reader - Reserved."]
pub type DIS_RTC_RAM_BOOT_R = crate::BitReader;
#[doc = "Field `DIS_ICACHE` reader - Set this bit to disable Icache."]
pub type DIS_ICACHE_R = crate::BitReader;
#[doc = "Field `DIS_DCACHE` reader - Set this bit to disable Dcache."]
pub type DIS_DCACHE_R = crate::BitReader;
#[doc = "Field `DIS_DOWNLOAD_ICACHE` reader - Disables Icache when SoC is in Download mode."]
pub type DIS_DOWNLOAD_ICACHE_R = crate::BitReader;
#[doc = "Field `DIS_DOWNLOAD_DCACHE` reader - Disables Dcache when SoC is in Download mode."]
pub type DIS_DOWNLOAD_DCACHE_R = crate::BitReader;
#[doc = "Field `DIS_FORCE_DOWNLOAD` reader - Set this bit to disable the function that forces chip into download mode."]
pub type DIS_FORCE_DOWNLOAD_R = crate::BitReader;
#[doc = "Field `DIS_USB` reader - Set this bit to disable USB OTG function."]
pub type DIS_USB_R = crate::BitReader;
#[doc = "Field `DIS_CAN` reader - Set this bit to disable the TWAI Controller function."]
pub type DIS_CAN_R = crate::BitReader;
#[doc = "Field `DIS_BOOT_REMAP` reader - Disables capability to Remap RAM to ROM address space."]
pub type DIS_BOOT_REMAP_R = crate::BitReader;
#[doc = "Field `RPT4_RESERVED5` reader - Reserved (used for four backups method)."]
pub type RPT4_RESERVED5_R = crate::BitReader;
#[doc = "Field `SOFT_DIS_JTAG` reader - Software disables JTAG. When software disabled, JTAG can be activated temporarily by HMAC peripheral."]
pub type SOFT_DIS_JTAG_R = crate::BitReader;
#[doc = "Field `HARD_DIS_JTAG` reader - Hardware disables JTAG permanently."]
pub type HARD_DIS_JTAG_R = crate::BitReader;
#[doc = "Field `DIS_DOWNLOAD_MANUAL_ENCRYPT` reader - Disables flash encryption when in download boot modes."]
pub type DIS_DOWNLOAD_MANUAL_ENCRYPT_R = crate::BitReader;
#[doc = "Field `USB_DREFH` reader - Controls single-end input threshold vrefh, 1.76 V to 2 V with step of 80 mV, stored in eFuse."]
pub type USB_DREFH_R = crate::FieldReader;
#[doc = "Field `USB_DREFL` reader - Controls single-end input threshold vrefl, 0.8 V to 1.04 V with step of 80 mV, stored in eFuse."]
pub type USB_DREFL_R = crate::FieldReader;
#[doc = "Field `USB_EXCHG_PINS` reader - Set this bit to exchange USB D+ and D- pins."]
pub type USB_EXCHG_PINS_R = crate::BitReader;
#[doc = "Field `EXT_PHY_ENABLE` reader - Set this bit to enable external USB PHY."]
pub type EXT_PHY_ENABLE_R = crate::BitReader;
#[doc = "Field `USB_FORCE_NOPERSIST` reader - If set, forces USB BVALID to 1."]
pub type USB_FORCE_NOPERSIST_R = crate::BitReader;
#[doc = "Field `RPT4_RESERVED0` reader - Reserved (used for four backups method)."]
pub type RPT4_RESERVED0_R = crate::FieldReader;
#[doc = "Field `VDD_SPI_MODECURLIM` reader - SPI regulator switches current limit mode."]
pub type VDD_SPI_MODECURLIM_R = crate::BitReader;
#[doc = "Field `VDD_SPI_DREFH` reader - SPI regulator high voltage reference."]
pub type VDD_SPI_DREFH_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Disables software reading from individual eFuse blocks (BLOCK4-10)."]
    #[inline(always)]
    pub fn rd_dis(&self) -> RD_DIS_R {
        RD_DIS_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Reserved."]
    #[inline(always)]
    pub fn dis_rtc_ram_boot(&self) -> DIS_RTC_RAM_BOOT_R {
        DIS_RTC_RAM_BOOT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to disable Icache."]
    #[inline(always)]
    pub fn dis_icache(&self) -> DIS_ICACHE_R {
        DIS_ICACHE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to disable Dcache."]
    #[inline(always)]
    pub fn dis_dcache(&self) -> DIS_DCACHE_R {
        DIS_DCACHE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Disables Icache when SoC is in Download mode."]
    #[inline(always)]
    pub fn dis_download_icache(&self) -> DIS_DOWNLOAD_ICACHE_R {
        DIS_DOWNLOAD_ICACHE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Disables Dcache when SoC is in Download mode."]
    #[inline(always)]
    pub fn dis_download_dcache(&self) -> DIS_DOWNLOAD_DCACHE_R {
        DIS_DOWNLOAD_DCACHE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to disable the function that forces chip into download mode."]
    #[inline(always)]
    pub fn dis_force_download(&self) -> DIS_FORCE_DOWNLOAD_R {
        DIS_FORCE_DOWNLOAD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to disable USB OTG function."]
    #[inline(always)]
    pub fn dis_usb(&self) -> DIS_USB_R {
        DIS_USB_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to disable the TWAI Controller function."]
    #[inline(always)]
    pub fn dis_can(&self) -> DIS_CAN_R {
        DIS_CAN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Disables capability to Remap RAM to ROM address space."]
    #[inline(always)]
    pub fn dis_boot_remap(&self) -> DIS_BOOT_REMAP_R {
        DIS_BOOT_REMAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reserved (used for four backups method)."]
    #[inline(always)]
    pub fn rpt4_reserved5(&self) -> RPT4_RESERVED5_R {
        RPT4_RESERVED5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Software disables JTAG. When software disabled, JTAG can be activated temporarily by HMAC peripheral."]
    #[inline(always)]
    pub fn soft_dis_jtag(&self) -> SOFT_DIS_JTAG_R {
        SOFT_DIS_JTAG_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Hardware disables JTAG permanently."]
    #[inline(always)]
    pub fn hard_dis_jtag(&self) -> HARD_DIS_JTAG_R {
        HARD_DIS_JTAG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Disables flash encryption when in download boot modes."]
    #[inline(always)]
    pub fn dis_download_manual_encrypt(&self) -> DIS_DOWNLOAD_MANUAL_ENCRYPT_R {
        DIS_DOWNLOAD_MANUAL_ENCRYPT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Controls single-end input threshold vrefh, 1.76 V to 2 V with step of 80 mV, stored in eFuse."]
    #[inline(always)]
    pub fn usb_drefh(&self) -> USB_DREFH_R {
        USB_DREFH_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Controls single-end input threshold vrefl, 0.8 V to 1.04 V with step of 80 mV, stored in eFuse."]
    #[inline(always)]
    pub fn usb_drefl(&self) -> USB_DREFL_R {
        USB_DREFL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Set this bit to exchange USB D+ and D- pins."]
    #[inline(always)]
    pub fn usb_exchg_pins(&self) -> USB_EXCHG_PINS_R {
        USB_EXCHG_PINS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Set this bit to enable external USB PHY."]
    #[inline(always)]
    pub fn ext_phy_enable(&self) -> EXT_PHY_ENABLE_R {
        EXT_PHY_ENABLE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - If set, forces USB BVALID to 1."]
    #[inline(always)]
    pub fn usb_force_nopersist(&self) -> USB_FORCE_NOPERSIST_R {
        USB_FORCE_NOPERSIST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Reserved (used for four backups method)."]
    #[inline(always)]
    pub fn rpt4_reserved0(&self) -> RPT4_RESERVED0_R {
        RPT4_RESERVED0_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - SPI regulator switches current limit mode."]
    #[inline(always)]
    pub fn vdd_spi_modecurlim(&self) -> VDD_SPI_MODECURLIM_R {
        VDD_SPI_MODECURLIM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - SPI regulator high voltage reference."]
    #[inline(always)]
    pub fn vdd_spi_drefh(&self) -> VDD_SPI_DREFH_R {
        VDD_SPI_DREFH_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA0")
            .field("rd_dis", &format_args!("{}", self.rd_dis().bits()))
            .field(
                "dis_rtc_ram_boot",
                &format_args!("{}", self.dis_rtc_ram_boot().bit()),
            )
            .field("dis_icache", &format_args!("{}", self.dis_icache().bit()))
            .field("dis_dcache", &format_args!("{}", self.dis_dcache().bit()))
            .field(
                "dis_download_icache",
                &format_args!("{}", self.dis_download_icache().bit()),
            )
            .field(
                "dis_download_dcache",
                &format_args!("{}", self.dis_download_dcache().bit()),
            )
            .field(
                "dis_force_download",
                &format_args!("{}", self.dis_force_download().bit()),
            )
            .field("dis_usb", &format_args!("{}", self.dis_usb().bit()))
            .field("dis_can", &format_args!("{}", self.dis_can().bit()))
            .field(
                "dis_boot_remap",
                &format_args!("{}", self.dis_boot_remap().bit()),
            )
            .field(
                "rpt4_reserved5",
                &format_args!("{}", self.rpt4_reserved5().bit()),
            )
            .field(
                "soft_dis_jtag",
                &format_args!("{}", self.soft_dis_jtag().bit()),
            )
            .field(
                "hard_dis_jtag",
                &format_args!("{}", self.hard_dis_jtag().bit()),
            )
            .field(
                "dis_download_manual_encrypt",
                &format_args!("{}", self.dis_download_manual_encrypt().bit()),
            )
            .field("usb_drefh", &format_args!("{}", self.usb_drefh().bits()))
            .field("usb_drefl", &format_args!("{}", self.usb_drefl().bits()))
            .field(
                "usb_exchg_pins",
                &format_args!("{}", self.usb_exchg_pins().bit()),
            )
            .field(
                "ext_phy_enable",
                &format_args!("{}", self.ext_phy_enable().bit()),
            )
            .field(
                "usb_force_nopersist",
                &format_args!("{}", self.usb_force_nopersist().bit()),
            )
            .field(
                "rpt4_reserved0",
                &format_args!("{}", self.rpt4_reserved0().bits()),
            )
            .field(
                "vdd_spi_modecurlim",
                &format_args!("{}", self.vdd_spi_modecurlim().bit()),
            )
            .field(
                "vdd_spi_drefh",
                &format_args!("{}", self.vdd_spi_drefh().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_REPEAT_DATA0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Register 1 of BLOCK0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_data0](index.html) module"]
pub struct RD_REPEAT_DATA0_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_data0::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_DATA0 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
