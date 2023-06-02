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
#[doc = "Field `RD_DIS_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type RD_DIS_ERR_R = crate::FieldReader;
#[doc = "Field `DIS_RTC_RAM_BOOT_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type DIS_RTC_RAM_BOOT_ERR_R = crate::BitReader;
#[doc = "Field `DIS_ICACHE_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type DIS_ICACHE_ERR_R = crate::BitReader;
#[doc = "Field `DIS_DCACHE_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type DIS_DCACHE_ERR_R = crate::BitReader;
#[doc = "Field `DIS_DOWNLOAD_ICACHE_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type DIS_DOWNLOAD_ICACHE_ERR_R = crate::BitReader;
#[doc = "Field `DIS_DOWNLOAD_DCACHE_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type DIS_DOWNLOAD_DCACHE_ERR_R = crate::BitReader;
#[doc = "Field `DIS_FORCE_DOWNLOAD_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type DIS_FORCE_DOWNLOAD_ERR_R = crate::BitReader;
#[doc = "Field `DIS_USB_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type DIS_USB_ERR_R = crate::BitReader;
#[doc = "Field `DIS_CAN_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type DIS_CAN_ERR_R = crate::BitReader;
#[doc = "Field `DIS_APP_CPU_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type DIS_APP_CPU_ERR_R = crate::BitReader;
#[doc = "Field `SOFT_DIS_JTAG_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type SOFT_DIS_JTAG_ERR_R = crate::FieldReader;
#[doc = "Field `DIS_PAD_JTAG_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type DIS_PAD_JTAG_ERR_R = crate::BitReader;
#[doc = "Field `DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R = crate::BitReader;
#[doc = "Field `USB_DREFH_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type USB_DREFH_ERR_R = crate::FieldReader;
#[doc = "Field `USB_DREFL_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type USB_DREFL_ERR_R = crate::FieldReader;
#[doc = "Field `USB_EXCHG_PINS_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type USB_EXCHG_PINS_ERR_R = crate::BitReader;
#[doc = "Field `EXT_PHY_ENABLE_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type EXT_PHY_ENABLE_ERR_R = crate::BitReader;
#[doc = "Field `BTLC_GPIO_ENABLE_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type BTLC_GPIO_ENABLE_ERR_R = crate::FieldReader;
#[doc = "Field `VDD_SPI_MODECURLIM_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type VDD_SPI_MODECURLIM_ERR_R = crate::BitReader;
#[doc = "Field `VDD_SPI_DREFH_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type VDD_SPI_DREFH_ERR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn rd_dis_err(&self) -> RD_DIS_ERR_R {
        RD_DIS_ERR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_rtc_ram_boot_err(&self) -> DIS_RTC_RAM_BOOT_ERR_R {
        DIS_RTC_RAM_BOOT_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_icache_err(&self) -> DIS_ICACHE_ERR_R {
        DIS_ICACHE_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_dcache_err(&self) -> DIS_DCACHE_ERR_R {
        DIS_DCACHE_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_download_icache_err(&self) -> DIS_DOWNLOAD_ICACHE_ERR_R {
        DIS_DOWNLOAD_ICACHE_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_download_dcache_err(&self) -> DIS_DOWNLOAD_DCACHE_ERR_R {
        DIS_DOWNLOAD_DCACHE_ERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_force_download_err(&self) -> DIS_FORCE_DOWNLOAD_ERR_R {
        DIS_FORCE_DOWNLOAD_ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_usb_err(&self) -> DIS_USB_ERR_R {
        DIS_USB_ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_can_err(&self) -> DIS_CAN_ERR_R {
        DIS_CAN_ERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_app_cpu_err(&self) -> DIS_APP_CPU_ERR_R {
        DIS_APP_CPU_ERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn soft_dis_jtag_err(&self) -> SOFT_DIS_JTAG_ERR_R {
        SOFT_DIS_JTAG_ERR_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_pad_jtag_err(&self) -> DIS_PAD_JTAG_ERR_R {
        DIS_PAD_JTAG_ERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_download_manual_encrypt_err(&self) -> DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R {
        DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn usb_drefh_err(&self) -> USB_DREFH_ERR_R {
        USB_DREFH_ERR_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn usb_drefl_err(&self) -> USB_DREFL_ERR_R {
        USB_DREFL_ERR_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn usb_exchg_pins_err(&self) -> USB_EXCHG_PINS_ERR_R {
        USB_EXCHG_PINS_ERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn ext_phy_enable_err(&self) -> EXT_PHY_ENABLE_ERR_R {
        EXT_PHY_ENABLE_ERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn btlc_gpio_enable_err(&self) -> BTLC_GPIO_ENABLE_ERR_R {
        BTLC_GPIO_ENABLE_ERR_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn vdd_spi_modecurlim_err(&self) -> VDD_SPI_MODECURLIM_ERR_R {
        VDD_SPI_MODECURLIM_ERR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - If any bits in this filed are 1, then it indicates a programming error."]
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
                "dis_app_cpu_err",
                &format_args!("{}", self.dis_app_cpu_err().bit()),
            )
            .field(
                "soft_dis_jtag_err",
                &format_args!("{}", self.soft_dis_jtag_err().bits()),
            )
            .field(
                "dis_pad_jtag_err",
                &format_args!("{}", self.dis_pad_jtag_err().bit()),
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
                "btlc_gpio_enable_err",
                &format_args!("{}", self.btlc_gpio_enable_err().bits()),
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
