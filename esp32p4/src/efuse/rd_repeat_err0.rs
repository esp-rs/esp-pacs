#[doc = "Register `RD_REPEAT_ERR0` reader"]
pub type R = crate::R<RD_REPEAT_ERR0_SPEC>;
#[doc = "Field `RD_DIS_ERR` reader - Indicates a programming error of RD_DIS."]
pub type RD_DIS_ERR_R = crate::FieldReader;
#[doc = "Field `DIS_USB_DEVICE_EXCHG_PINS_ERR` reader - Indicates a programming error of DIS_USB_DEVICE_EXCHG_PINS."]
pub type DIS_USB_DEVICE_EXCHG_PINS_ERR_R = crate::BitReader;
#[doc = "Field `DIS_USB_OTG11_EXCHG_PINS_ERR` reader - Indicates a programming error of DIS_USB_OTG11_EXCHG_PINS."]
pub type DIS_USB_OTG11_EXCHG_PINS_ERR_R = crate::BitReader;
#[doc = "Field `DIS_USB_JTAG_ERR` reader - Indicates a programming error of DIS_USB_JTAG."]
pub type DIS_USB_JTAG_ERR_R = crate::BitReader;
#[doc = "Field `POWERGLITCH_EN_ERR` reader - Indicates a programming error of POWERGLITCH_EN."]
pub type POWERGLITCH_EN_ERR_R = crate::BitReader;
#[doc = "Field `DIS_USB_SERIAL_JTAG_ERR` reader - Indicates a programming error of DIS_USB_SERIAL_JTAG."]
pub type DIS_USB_SERIAL_JTAG_ERR_R = crate::BitReader;
#[doc = "Field `DIS_FORCE_DOWNLOAD_ERR` reader - Indicates a programming error of DIS_FORCE_DOWNLOAD."]
pub type DIS_FORCE_DOWNLOAD_ERR_R = crate::BitReader;
#[doc = "Field `SPI_DOWNLOAD_MSPI_DIS_ERR` reader - Indicates a programming error of SPI_DOWNLOAD_MSPI_DIS."]
pub type SPI_DOWNLOAD_MSPI_DIS_ERR_R = crate::BitReader;
#[doc = "Field `DIS_TWAI_ERR` reader - Indicates a programming error of DIS_TWAI."]
pub type DIS_TWAI_ERR_R = crate::BitReader;
#[doc = "Field `JTAG_SEL_ENABLE_ERR` reader - Indicates a programming error of JTAG_SEL_ENABLE."]
pub type JTAG_SEL_ENABLE_ERR_R = crate::BitReader;
#[doc = "Field `SOFT_DIS_JTAG_ERR` reader - Indicates a programming error of SOFT_DIS_JTAG."]
pub type SOFT_DIS_JTAG_ERR_R = crate::FieldReader;
#[doc = "Field `DIS_PAD_JTAG_ERR` reader - Indicates a programming error of DIS_PAD_JTAG."]
pub type DIS_PAD_JTAG_ERR_R = crate::BitReader;
#[doc = "Field `DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR` reader - Indicates a programming error of DIS_DOWNLOAD_MANUAL_ENCRYPT."]
pub type DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R = crate::BitReader;
#[doc = "Field `USB_DEVICE_DREFH_ERR` reader - Indicates a programming error of USB_DEVICE_DREFH."]
pub type USB_DEVICE_DREFH_ERR_R = crate::FieldReader;
#[doc = "Field `USB_OTG11_DREFH_ERR` reader - Indicates a programming error of USB_OTG11_DREFH."]
pub type USB_OTG11_DREFH_ERR_R = crate::FieldReader;
#[doc = "Field `USB_PHY_SEL_ERR` reader - Indicates a programming error of USB_PHY_SEL."]
pub type USB_PHY_SEL_ERR_R = crate::BitReader;
#[doc = "Field `HUK_GEN_STATE_LOW_ERR` reader - Indicates a programming error of HUK_GEN_STATE_LOW."]
pub type HUK_GEN_STATE_LOW_ERR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Indicates a programming error of RD_DIS."]
    #[inline(always)]
    pub fn rd_dis_err(&self) -> RD_DIS_ERR_R {
        RD_DIS_ERR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Indicates a programming error of DIS_USB_DEVICE_EXCHG_PINS."]
    #[inline(always)]
    pub fn dis_usb_device_exchg_pins_err(&self) -> DIS_USB_DEVICE_EXCHG_PINS_ERR_R {
        DIS_USB_DEVICE_EXCHG_PINS_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates a programming error of DIS_USB_OTG11_EXCHG_PINS."]
    #[inline(always)]
    pub fn dis_usb_otg11_exchg_pins_err(&self) -> DIS_USB_OTG11_EXCHG_PINS_ERR_R {
        DIS_USB_OTG11_EXCHG_PINS_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates a programming error of DIS_USB_JTAG."]
    #[inline(always)]
    pub fn dis_usb_jtag_err(&self) -> DIS_USB_JTAG_ERR_R {
        DIS_USB_JTAG_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates a programming error of POWERGLITCH_EN."]
    #[inline(always)]
    pub fn powerglitch_en_err(&self) -> POWERGLITCH_EN_ERR_R {
        POWERGLITCH_EN_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Indicates a programming error of DIS_USB_SERIAL_JTAG."]
    #[inline(always)]
    pub fn dis_usb_serial_jtag_err(&self) -> DIS_USB_SERIAL_JTAG_ERR_R {
        DIS_USB_SERIAL_JTAG_ERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates a programming error of DIS_FORCE_DOWNLOAD."]
    #[inline(always)]
    pub fn dis_force_download_err(&self) -> DIS_FORCE_DOWNLOAD_ERR_R {
        DIS_FORCE_DOWNLOAD_ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Indicates a programming error of SPI_DOWNLOAD_MSPI_DIS."]
    #[inline(always)]
    pub fn spi_download_mspi_dis_err(&self) -> SPI_DOWNLOAD_MSPI_DIS_ERR_R {
        SPI_DOWNLOAD_MSPI_DIS_ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Indicates a programming error of DIS_TWAI."]
    #[inline(always)]
    pub fn dis_twai_err(&self) -> DIS_TWAI_ERR_R {
        DIS_TWAI_ERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Indicates a programming error of JTAG_SEL_ENABLE."]
    #[inline(always)]
    pub fn jtag_sel_enable_err(&self) -> JTAG_SEL_ENABLE_ERR_R {
        JTAG_SEL_ENABLE_ERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Indicates a programming error of SOFT_DIS_JTAG."]
    #[inline(always)]
    pub fn soft_dis_jtag_err(&self) -> SOFT_DIS_JTAG_ERR_R {
        SOFT_DIS_JTAG_ERR_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Indicates a programming error of DIS_PAD_JTAG."]
    #[inline(always)]
    pub fn dis_pad_jtag_err(&self) -> DIS_PAD_JTAG_ERR_R {
        DIS_PAD_JTAG_ERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Indicates a programming error of DIS_DOWNLOAD_MANUAL_ENCRYPT."]
    #[inline(always)]
    pub fn dis_download_manual_encrypt_err(&self) -> DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R {
        DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Indicates a programming error of USB_DEVICE_DREFH."]
    #[inline(always)]
    pub fn usb_device_drefh_err(&self) -> USB_DEVICE_DREFH_ERR_R {
        USB_DEVICE_DREFH_ERR_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Indicates a programming error of USB_OTG11_DREFH."]
    #[inline(always)]
    pub fn usb_otg11_drefh_err(&self) -> USB_OTG11_DREFH_ERR_R {
        USB_OTG11_DREFH_ERR_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - Indicates a programming error of USB_PHY_SEL."]
    #[inline(always)]
    pub fn usb_phy_sel_err(&self) -> USB_PHY_SEL_ERR_R {
        USB_PHY_SEL_ERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - Indicates a programming error of HUK_GEN_STATE_LOW."]
    #[inline(always)]
    pub fn huk_gen_state_low_err(&self) -> HUK_GEN_STATE_LOW_ERR_R {
        HUK_GEN_STATE_LOW_ERR_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_ERR0")
            .field("rd_dis_err", &self.rd_dis_err())
            .field(
                "dis_usb_device_exchg_pins_err",
                &self.dis_usb_device_exchg_pins_err(),
            )
            .field(
                "dis_usb_otg11_exchg_pins_err",
                &self.dis_usb_otg11_exchg_pins_err(),
            )
            .field("dis_usb_jtag_err", &self.dis_usb_jtag_err())
            .field("powerglitch_en_err", &self.powerglitch_en_err())
            .field("dis_usb_serial_jtag_err", &self.dis_usb_serial_jtag_err())
            .field("dis_force_download_err", &self.dis_force_download_err())
            .field(
                "spi_download_mspi_dis_err",
                &self.spi_download_mspi_dis_err(),
            )
            .field("dis_twai_err", &self.dis_twai_err())
            .field("jtag_sel_enable_err", &self.jtag_sel_enable_err())
            .field("soft_dis_jtag_err", &self.soft_dis_jtag_err())
            .field("dis_pad_jtag_err", &self.dis_pad_jtag_err())
            .field(
                "dis_download_manual_encrypt_err",
                &self.dis_download_manual_encrypt_err(),
            )
            .field("usb_device_drefh_err", &self.usb_device_drefh_err())
            .field("usb_otg11_drefh_err", &self.usb_otg11_drefh_err())
            .field("usb_phy_sel_err", &self.usb_phy_sel_err())
            .field("huk_gen_state_low_err", &self.huk_gen_state_low_err())
            .finish()
    }
}
#[doc = "Programming error record register 0 of BLOCK0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_err0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_ERR0_SPEC;
impl crate::RegisterSpec for RD_REPEAT_ERR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_err0::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR0_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_ERR0 to value 0"]
impl crate::Resettable for RD_REPEAT_ERR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
