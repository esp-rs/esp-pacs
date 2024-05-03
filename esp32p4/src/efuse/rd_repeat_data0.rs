#[doc = "Register `RD_REPEAT_DATA0` reader"]
pub type R = crate::R<RD_REPEAT_DATA0_SPEC>;
#[doc = "Field `RD_DIS` reader - Represents whether reading of individual eFuse block(block4~block10) is disabled or enabled. 1: disabled. 0: enabled."]
pub type RD_DIS_R = crate::FieldReader;
#[doc = "Field `USB_DEVICE_EXCHG_PINS` reader - Enable usb device exchange pins of D+ and D-."]
pub type USB_DEVICE_EXCHG_PINS_R = crate::BitReader;
#[doc = "Field `USB_OTG11_EXCHG_PINS` reader - Enable usb otg11 exchange pins of D+ and D-."]
pub type USB_OTG11_EXCHG_PINS_R = crate::BitReader;
#[doc = "Field `DIS_USB_JTAG` reader - Represents whether the function of usb switch to jtag is disabled or enabled. 1: disabled. 0: enabled."]
pub type DIS_USB_JTAG_R = crate::BitReader;
#[doc = "Field `POWERGLITCH_EN` reader - Represents whether power glitch function is enabled. 1: enabled. 0: disabled."]
pub type POWERGLITCH_EN_R = crate::BitReader;
#[doc = "Field `DIS_USB_SERIAL_JTAG` reader - Represents whether USB-Serial-JTAG is disabled or enabled. 1: disabled. 0: enabled."]
pub type DIS_USB_SERIAL_JTAG_R = crate::BitReader;
#[doc = "Field `DIS_FORCE_DOWNLOAD` reader - Represents whether the function that forces chip into download mode is disabled or enabled. 1: disabled. 0: enabled."]
pub type DIS_FORCE_DOWNLOAD_R = crate::BitReader;
#[doc = "Field `SPI_DOWNLOAD_MSPI_DIS` reader - Set this bit to disable accessing MSPI flash/MSPI ram by SYS AXI matrix during boot_mode_download."]
pub type SPI_DOWNLOAD_MSPI_DIS_R = crate::BitReader;
#[doc = "Field `DIS_TWAI` reader - Represents whether TWAI function is disabled or enabled. 1: disabled. 0: enabled."]
pub type DIS_TWAI_R = crate::BitReader;
#[doc = "Field `JTAG_SEL_ENABLE` reader - Represents whether the selection between usb_to_jtag and pad_to_jtag through strapping gpio15 when both EFUSE_DIS_PAD_JTAG and EFUSE_DIS_USB_JTAG are equal to 0 is enabled or disabled. 1: enabled. 0: disabled."]
pub type JTAG_SEL_ENABLE_R = crate::BitReader;
#[doc = "Field `SOFT_DIS_JTAG` reader - Represents whether JTAG is disabled in soft way. Odd number: disabled. Even number: enabled."]
pub type SOFT_DIS_JTAG_R = crate::FieldReader;
#[doc = "Field `DIS_PAD_JTAG` reader - Represents whether JTAG is disabled in the hard way(permanently). 1: disabled. 0: enabled."]
pub type DIS_PAD_JTAG_R = crate::BitReader;
#[doc = "Field `DIS_DOWNLOAD_MANUAL_ENCRYPT` reader - Represents whether flash encrypt function is disabled or enabled(except in SPI boot mode). 1: disabled. 0: enabled."]
pub type DIS_DOWNLOAD_MANUAL_ENCRYPT_R = crate::BitReader;
#[doc = "Field `USB_DEVICE_DREFH` reader - USB intphy of usb device signle-end input high threshold, 1.76V to 2V. Step by 80mV"]
pub type USB_DEVICE_DREFH_R = crate::FieldReader;
#[doc = "Field `USB_OTG11_DREFH` reader - USB intphy of usb otg11 signle-end input high threshold, 1.76V to 2V. Step by 80mV"]
pub type USB_OTG11_DREFH_R = crate::FieldReader;
#[doc = "Field `USB_PHY_SEL` reader - TBD"]
pub type USB_PHY_SEL_R = crate::BitReader;
#[doc = "Field `KM_HUK_GEN_STATE_LOW` reader - Set this bit to control validation of HUK generate mode. Odd of 1 is invalid, even of 1 is valid."]
pub type KM_HUK_GEN_STATE_LOW_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Represents whether reading of individual eFuse block(block4~block10) is disabled or enabled. 1: disabled. 0: enabled."]
    #[inline(always)]
    pub fn rd_dis(&self) -> RD_DIS_R {
        RD_DIS_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Enable usb device exchange pins of D+ and D-."]
    #[inline(always)]
    pub fn usb_device_exchg_pins(&self) -> USB_DEVICE_EXCHG_PINS_R {
        USB_DEVICE_EXCHG_PINS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable usb otg11 exchange pins of D+ and D-."]
    #[inline(always)]
    pub fn usb_otg11_exchg_pins(&self) -> USB_OTG11_EXCHG_PINS_R {
        USB_OTG11_EXCHG_PINS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents whether the function of usb switch to jtag is disabled or enabled. 1: disabled. 0: enabled."]
    #[inline(always)]
    pub fn dis_usb_jtag(&self) -> DIS_USB_JTAG_R {
        DIS_USB_JTAG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents whether power glitch function is enabled. 1: enabled. 0: disabled."]
    #[inline(always)]
    pub fn powerglitch_en(&self) -> POWERGLITCH_EN_R {
        POWERGLITCH_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents whether USB-Serial-JTAG is disabled or enabled. 1: disabled. 0: enabled."]
    #[inline(always)]
    pub fn dis_usb_serial_jtag(&self) -> DIS_USB_SERIAL_JTAG_R {
        DIS_USB_SERIAL_JTAG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents whether the function that forces chip into download mode is disabled or enabled. 1: disabled. 0: enabled."]
    #[inline(always)]
    pub fn dis_force_download(&self) -> DIS_FORCE_DOWNLOAD_R {
        DIS_FORCE_DOWNLOAD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to disable accessing MSPI flash/MSPI ram by SYS AXI matrix during boot_mode_download."]
    #[inline(always)]
    pub fn spi_download_mspi_dis(&self) -> SPI_DOWNLOAD_MSPI_DIS_R {
        SPI_DOWNLOAD_MSPI_DIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents whether TWAI function is disabled or enabled. 1: disabled. 0: enabled."]
    #[inline(always)]
    pub fn dis_twai(&self) -> DIS_TWAI_R {
        DIS_TWAI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Represents whether the selection between usb_to_jtag and pad_to_jtag through strapping gpio15 when both EFUSE_DIS_PAD_JTAG and EFUSE_DIS_USB_JTAG are equal to 0 is enabled or disabled. 1: enabled. 0: disabled."]
    #[inline(always)]
    pub fn jtag_sel_enable(&self) -> JTAG_SEL_ENABLE_R {
        JTAG_SEL_ENABLE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Represents whether JTAG is disabled in soft way. Odd number: disabled. Even number: enabled."]
    #[inline(always)]
    pub fn soft_dis_jtag(&self) -> SOFT_DIS_JTAG_R {
        SOFT_DIS_JTAG_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Represents whether JTAG is disabled in the hard way(permanently). 1: disabled. 0: enabled."]
    #[inline(always)]
    pub fn dis_pad_jtag(&self) -> DIS_PAD_JTAG_R {
        DIS_PAD_JTAG_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents whether flash encrypt function is disabled or enabled(except in SPI boot mode). 1: disabled. 0: enabled."]
    #[inline(always)]
    pub fn dis_download_manual_encrypt(&self) -> DIS_DOWNLOAD_MANUAL_ENCRYPT_R {
        DIS_DOWNLOAD_MANUAL_ENCRYPT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - USB intphy of usb device signle-end input high threshold, 1.76V to 2V. Step by 80mV"]
    #[inline(always)]
    pub fn usb_device_drefh(&self) -> USB_DEVICE_DREFH_R {
        USB_DEVICE_DREFH_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - USB intphy of usb otg11 signle-end input high threshold, 1.76V to 2V. Step by 80mV"]
    #[inline(always)]
    pub fn usb_otg11_drefh(&self) -> USB_OTG11_DREFH_R {
        USB_OTG11_DREFH_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - TBD"]
    #[inline(always)]
    pub fn usb_phy_sel(&self) -> USB_PHY_SEL_R {
        USB_PHY_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - Set this bit to control validation of HUK generate mode. Odd of 1 is invalid, even of 1 is valid."]
    #[inline(always)]
    pub fn km_huk_gen_state_low(&self) -> KM_HUK_GEN_STATE_LOW_R {
        KM_HUK_GEN_STATE_LOW_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA0")
            .field("rd_dis", &self.rd_dis().bits())
            .field("usb_device_exchg_pins", &self.usb_device_exchg_pins().bit())
            .field("usb_otg11_exchg_pins", &self.usb_otg11_exchg_pins().bit())
            .field("dis_usb_jtag", &self.dis_usb_jtag().bit())
            .field("powerglitch_en", &self.powerglitch_en().bit())
            .field("dis_usb_serial_jtag", &self.dis_usb_serial_jtag().bit())
            .field("dis_force_download", &self.dis_force_download().bit())
            .field("spi_download_mspi_dis", &self.spi_download_mspi_dis().bit())
            .field("dis_twai", &self.dis_twai().bit())
            .field("jtag_sel_enable", &self.jtag_sel_enable().bit())
            .field("soft_dis_jtag", &self.soft_dis_jtag().bits())
            .field("dis_pad_jtag", &self.dis_pad_jtag().bit())
            .field(
                "dis_download_manual_encrypt",
                &self.dis_download_manual_encrypt().bit(),
            )
            .field("usb_device_drefh", &self.usb_device_drefh().bits())
            .field("usb_otg11_drefh", &self.usb_otg11_drefh().bits())
            .field("usb_phy_sel", &self.usb_phy_sel().bit())
            .field("km_huk_gen_state_low", &self.km_huk_gen_state_low().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_REPEAT_DATA0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "BLOCK0 data register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA0_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data0::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA0_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA0 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA0_SPEC {
    const RESET_VALUE: u32 = 0;
}
