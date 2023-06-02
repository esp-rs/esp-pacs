#[doc = "Register `RD_REPEAT_ERR0` reader"]
pub struct R(crate::R<RD_REPEAT_ERR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_ERR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_ERR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_ERR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_DIS_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_RD_DIS."]
pub type RD_DIS_ERR_R = crate::FieldReader;
#[doc = "Field `DIS_RTC_RAM_BOOT_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_RTC_RAM_BOOT."]
pub type DIS_RTC_RAM_BOOT_ERR_R = crate::BitReader;
#[doc = "Field `DIS_ICACHE_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_ICACHE."]
pub type DIS_ICACHE_ERR_R = crate::BitReader;
#[doc = "Field `DIS_DCACHE_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_DCACHE."]
pub type DIS_DCACHE_ERR_R = crate::BitReader;
#[doc = "Field `DIS_DOWNLOAD_ICACHE_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_DOWNLOAD_ICACHE."]
pub type DIS_DOWNLOAD_ICACHE_ERR_R = crate::BitReader;
#[doc = "Field `DIS_DOWNLOAD_DCACHE_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_DOWNLOAD_DCACHE."]
pub type DIS_DOWNLOAD_DCACHE_ERR_R = crate::BitReader;
#[doc = "Field `DIS_FORCE_DOWNLOAD_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_FORCE_DOWNLOAD."]
pub type DIS_FORCE_DOWNLOAD_ERR_R = crate::BitReader;
#[doc = "Field `DIS_USB_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_USB."]
pub type DIS_USB_ERR_R = crate::BitReader;
#[doc = "Field `DIS_CAN_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_CAN."]
pub type DIS_CAN_ERR_R = crate::BitReader;
#[doc = "Field `DIS_BOOT_REMAP_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_BOOT_REMAP."]
pub type DIS_BOOT_REMAP_ERR_R = crate::BitReader;
#[doc = "Field `RPT4_RESERVED5_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_RPT4_RESERVED5."]
pub type RPT4_RESERVED5_ERR_R = crate::BitReader;
#[doc = "Field `SOFT_DIS_JTAG_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_SOFT_DIS_JTAG."]
pub type SOFT_DIS_JTAG_ERR_R = crate::BitReader;
#[doc = "Field `HARD_DIS_JTAG_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_HARD_DIS_JTAG."]
pub type HARD_DIS_JTAG_ERR_R = crate::BitReader;
#[doc = "Field `DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_DOWNLOAD_MANUAL_ENCRYPT."]
pub type DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R = crate::BitReader;
#[doc = "Field `USB_DREFH_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_USB_DREFH."]
pub type USB_DREFH_ERR_R = crate::FieldReader;
#[doc = "Field `USB_DREFL_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_USB_DREFL."]
pub type USB_DREFL_ERR_R = crate::FieldReader;
#[doc = "Field `USB_EXCHG_PINS_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_USB_EXCHG_PINS."]
pub type USB_EXCHG_PINS_ERR_R = crate::BitReader;
#[doc = "Field `EXT_PHY_ENABLE_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_EXT_PHY_ENABLE."]
pub type EXT_PHY_ENABLE_ERR_R = crate::BitReader;
#[doc = "Field `USB_FORCE_NOPERSIST_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_USB_FORCE_NOPERSIST."]
pub type USB_FORCE_NOPERSIST_ERR_R = crate::BitReader;
#[doc = "Field `RPT4_RESERVED0_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_RPT4_RESERVED0."]
pub type RPT4_RESERVED0_ERR_R = crate::FieldReader;
#[doc = "Field `VDD_SPI_MODECURLIM_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_MODECURLIM."]
pub type VDD_SPI_MODECURLIM_ERR_R = crate::BitReader;
#[doc = "Field `VDD_SPI_DREFH_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_DREFH."]
pub type VDD_SPI_DREFH_ERR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Any bit equal to 1 denotes a programming error in EFUSE_RD_DIS."]
    #[inline(always)]
    pub fn rd_dis_err(&self) -> RD_DIS_ERR_R {
        RD_DIS_ERR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_RTC_RAM_BOOT."]
    #[inline(always)]
    pub fn dis_rtc_ram_boot_err(&self) -> DIS_RTC_RAM_BOOT_ERR_R {
        DIS_RTC_RAM_BOOT_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_ICACHE."]
    #[inline(always)]
    pub fn dis_icache_err(&self) -> DIS_ICACHE_ERR_R {
        DIS_ICACHE_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_DCACHE."]
    #[inline(always)]
    pub fn dis_dcache_err(&self) -> DIS_DCACHE_ERR_R {
        DIS_DCACHE_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_DOWNLOAD_ICACHE."]
    #[inline(always)]
    pub fn dis_download_icache_err(&self) -> DIS_DOWNLOAD_ICACHE_ERR_R {
        DIS_DOWNLOAD_ICACHE_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_DOWNLOAD_DCACHE."]
    #[inline(always)]
    pub fn dis_download_dcache_err(&self) -> DIS_DOWNLOAD_DCACHE_ERR_R {
        DIS_DOWNLOAD_DCACHE_ERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_FORCE_DOWNLOAD."]
    #[inline(always)]
    pub fn dis_force_download_err(&self) -> DIS_FORCE_DOWNLOAD_ERR_R {
        DIS_FORCE_DOWNLOAD_ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_USB."]
    #[inline(always)]
    pub fn dis_usb_err(&self) -> DIS_USB_ERR_R {
        DIS_USB_ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_CAN."]
    #[inline(always)]
    pub fn dis_can_err(&self) -> DIS_CAN_ERR_R {
        DIS_CAN_ERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_BOOT_REMAP."]
    #[inline(always)]
    pub fn dis_boot_remap_err(&self) -> DIS_BOOT_REMAP_ERR_R {
        DIS_BOOT_REMAP_ERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Any bit equal to 1 denotes a programming error in EFUSE_RPT4_RESERVED5."]
    #[inline(always)]
    pub fn rpt4_reserved5_err(&self) -> RPT4_RESERVED5_ERR_R {
        RPT4_RESERVED5_ERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Any bit equal to 1 denotes a programming error in EFUSE_SOFT_DIS_JTAG."]
    #[inline(always)]
    pub fn soft_dis_jtag_err(&self) -> SOFT_DIS_JTAG_ERR_R {
        SOFT_DIS_JTAG_ERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Any bit equal to 1 denotes a programming error in EFUSE_HARD_DIS_JTAG."]
    #[inline(always)]
    pub fn hard_dis_jtag_err(&self) -> HARD_DIS_JTAG_ERR_R {
        HARD_DIS_JTAG_ERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_DOWNLOAD_MANUAL_ENCRYPT."]
    #[inline(always)]
    pub fn dis_download_manual_encrypt_err(&self) -> DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R {
        DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Any bit equal to 1 denotes a programming error in EFUSE_USB_DREFH."]
    #[inline(always)]
    pub fn usb_drefh_err(&self) -> USB_DREFH_ERR_R {
        USB_DREFH_ERR_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Any bit equal to 1 denotes a programming error in EFUSE_USB_DREFL."]
    #[inline(always)]
    pub fn usb_drefl_err(&self) -> USB_DREFL_ERR_R {
        USB_DREFL_ERR_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Any bit equal to 1 denotes a programming error in EFUSE_USB_EXCHG_PINS."]
    #[inline(always)]
    pub fn usb_exchg_pins_err(&self) -> USB_EXCHG_PINS_ERR_R {
        USB_EXCHG_PINS_ERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Any bit equal to 1 denotes a programming error in EFUSE_EXT_PHY_ENABLE."]
    #[inline(always)]
    pub fn ext_phy_enable_err(&self) -> EXT_PHY_ENABLE_ERR_R {
        EXT_PHY_ENABLE_ERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Any bit equal to 1 denotes a programming error in EFUSE_USB_FORCE_NOPERSIST."]
    #[inline(always)]
    pub fn usb_force_nopersist_err(&self) -> USB_FORCE_NOPERSIST_ERR_R {
        USB_FORCE_NOPERSIST_ERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Any bit equal to 1 denotes a programming error in EFUSE_RPT4_RESERVED0."]
    #[inline(always)]
    pub fn rpt4_reserved0_err(&self) -> RPT4_RESERVED0_ERR_R {
        RPT4_RESERVED0_ERR_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_MODECURLIM."]
    #[inline(always)]
    pub fn vdd_spi_modecurlim_err(&self) -> VDD_SPI_MODECURLIM_ERR_R {
        VDD_SPI_MODECURLIM_ERR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_DREFH."]
    #[inline(always)]
    pub fn vdd_spi_drefh_err(&self) -> VDD_SPI_DREFH_ERR_R {
        VDD_SPI_DREFH_ERR_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_ERR0")
            .field("rd_dis_err", &format_args!("{}", self.rd_dis_err().bits()))
            .field(
                "dis_rtc_ram_boot_err",
                &format_args!("{}", self.dis_rtc_ram_boot_err().bit()),
            )
            .field(
                "dis_icache_err",
                &format_args!("{}", self.dis_icache_err().bit()),
            )
            .field(
                "dis_dcache_err",
                &format_args!("{}", self.dis_dcache_err().bit()),
            )
            .field(
                "dis_download_icache_err",
                &format_args!("{}", self.dis_download_icache_err().bit()),
            )
            .field(
                "dis_download_dcache_err",
                &format_args!("{}", self.dis_download_dcache_err().bit()),
            )
            .field(
                "dis_force_download_err",
                &format_args!("{}", self.dis_force_download_err().bit()),
            )
            .field("dis_usb_err", &format_args!("{}", self.dis_usb_err().bit()))
            .field("dis_can_err", &format_args!("{}", self.dis_can_err().bit()))
            .field(
                "dis_boot_remap_err",
                &format_args!("{}", self.dis_boot_remap_err().bit()),
            )
            .field(
                "rpt4_reserved5_err",
                &format_args!("{}", self.rpt4_reserved5_err().bit()),
            )
            .field(
                "soft_dis_jtag_err",
                &format_args!("{}", self.soft_dis_jtag_err().bit()),
            )
            .field(
                "hard_dis_jtag_err",
                &format_args!("{}", self.hard_dis_jtag_err().bit()),
            )
            .field(
                "dis_download_manual_encrypt_err",
                &format_args!("{}", self.dis_download_manual_encrypt_err().bit()),
            )
            .field(
                "usb_drefh_err",
                &format_args!("{}", self.usb_drefh_err().bits()),
            )
            .field(
                "usb_drefl_err",
                &format_args!("{}", self.usb_drefl_err().bits()),
            )
            .field(
                "usb_exchg_pins_err",
                &format_args!("{}", self.usb_exchg_pins_err().bit()),
            )
            .field(
                "ext_phy_enable_err",
                &format_args!("{}", self.ext_phy_enable_err().bit()),
            )
            .field(
                "usb_force_nopersist_err",
                &format_args!("{}", self.usb_force_nopersist_err().bit()),
            )
            .field(
                "rpt4_reserved0_err",
                &format_args!("{}", self.rpt4_reserved0_err().bits()),
            )
            .field(
                "vdd_spi_modecurlim_err",
                &format_args!("{}", self.vdd_spi_modecurlim_err().bit()),
            )
            .field(
                "vdd_spi_drefh_err",
                &format_args!("{}", self.vdd_spi_drefh_err().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_REPEAT_ERR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Programming error record register 0 of BLOCK0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_err0](index.html) module"]
pub struct RD_REPEAT_ERR0_SPEC;
impl crate::RegisterSpec for RD_REPEAT_ERR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_err0::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_ERR0 to value 0"]
impl crate::Resettable for RD_REPEAT_ERR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
