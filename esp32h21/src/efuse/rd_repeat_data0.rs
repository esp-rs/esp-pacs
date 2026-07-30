#[doc = "Register `RD_REPEAT_DATA0` reader"]
pub type R = crate::R<RD_REPEAT_DATA0_SPEC>;
#[doc = "Field `RD_DIS` reader - Represents whether reading of individual eFuse block(block4~block10) is disabled or enabled.\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
pub type RD_DIS_R = crate::FieldReader;
#[doc = "Field `PVT_GLITCH_EN` reader - Represents whether to enable PVT power glitch monitor function.\\\\1: Enable. \\\\0: Disable\\\\"]
pub type PVT_GLITCH_EN_R = crate::BitReader;
#[doc = "Field `DIS_ICACHE` reader - Represents whether icache is disabled or enabled.\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
pub type DIS_ICACHE_R = crate::BitReader;
#[doc = "Field `DIS_USB_JTAG` reader - Represents whether the USB-to-JTAG function in USB Serial/JTAG is disabled. \\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
pub type DIS_USB_JTAG_R = crate::BitReader;
#[doc = "Field `POWERGLITCH_EN` reader - Represents whether to enable power glitch function.\\\\"]
pub type POWERGLITCH_EN_R = crate::BitReader;
#[doc = "Field `DIS_USB_SERIAL_JTAG` reader - Represents whether to disable USB-Serial-JTAG.\\\\"]
pub type DIS_USB_SERIAL_JTAG_R = crate::BitReader;
#[doc = "Field `DIS_FORCE_DOWNLOAD` reader - Represents whether the function that forces chip into Download mode is disabled. \\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
pub type DIS_FORCE_DOWNLOAD_R = crate::BitReader;
#[doc = "Field `SPI_DOWNLOAD_MSPI_DIS` reader - Represents accessing MSPI flash/MSPI RAM by SYS AXI matrix is disabled during boot_mode_download.\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
pub type SPI_DOWNLOAD_MSPI_DIS_R = crate::BitReader;
#[doc = "Field `DIS_TWAI` reader - Represents whether TWAI function is disabled. \\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
pub type DIS_TWAI_R = crate::BitReader;
#[doc = "Field `JTAG_SEL_ENABLE` reader - Represents whether the selection of a JTAG signal source through the strapping pin value is enabled when both EFUSE_DIS_PAD_JTAG and EFUSE_DIS_USB_JTAG are configured to 0. For more information, please refer to Chapter Placeholder.\\\\ 1: Enabled\\\\ 0: Disabled\\\\"]
pub type JTAG_SEL_ENABLE_R = crate::BitReader;
#[doc = "Field `SOFT_DIS_JTAG` reader - Represents whether PAD JTAG is disabled in the soft way. It can be restarted via HMAC. \\\\ Odd count of bits with a value of 1: Disabled\\\\ Even count of bits with a value of 1: Enabled\\\\"]
pub type SOFT_DIS_JTAG_R = crate::FieldReader;
#[doc = "Field `DIS_PAD_JTAG` reader - Represents whether PAD JTAG is disabled in the hard way (permanently).\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
pub type DIS_PAD_JTAG_R = crate::BitReader;
#[doc = "Field `DIS_DOWNLOAD_MANUAL_ENCRYPT` reader - Represents whether flash encryption is disabled (except in SPI boot mode).\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
pub type DIS_DOWNLOAD_MANUAL_ENCRYPT_R = crate::BitReader;
#[doc = "Field `USB_DREFH` reader - Represents single-end input threshold vrefh, 1.76 V to 2 V with step of 80 mV, stored in eFuse.\\\\"]
pub type USB_DREFH_R = crate::FieldReader;
#[doc = "Field `USB_DREFL` reader - Represents single-end input threshold vrefl, 0.8 V to 1.04 V with step of 80 mV, stored in eFuse.\\\\"]
pub type USB_DREFL_R = crate::FieldReader;
#[doc = "Field `USB_EXCHG_PINS` reader - Represents whether the D+ and D- pins is exchanged.\\\\ 1: Exchanged\\\\ 0: Not exchanged\\\\"]
pub type USB_EXCHG_PINS_R = crate::BitReader;
#[doc = "Field `VDD_SPI_AS_GPIO` reader - Represents whether vdd spi pin is functioned as gpio.\\\\ 1: Functioned\\\\ 0: Not functioned\\\\"]
pub type VDD_SPI_AS_GPIO_R = crate::BitReader;
#[doc = "Field `ECDSA_CURVE_MODE` reader - Represents the configuration of the curve of ECDSA calculation.\\\\ 0: Only enable P256\\\\ 1: Only enable P192\\\\ 2: Both enable P256 and P192\\\\ 3: Only enable P256\\\\"]
pub type ECDSA_CURVE_MODE_R = crate::FieldReader;
#[doc = "Field `ECC_FORCE_CONST_TIME` reader - Represents whether to permanently turn on ECC const-time mode.\\\\ 0: Disabled\\\\ 1: Enabled\\\\"]
pub type ECC_FORCE_CONST_TIME_R = crate::BitReader;
#[doc = "Field `XTS_DPA_PSEUDO_LEVEL` reader - Represents control method of xts pseudo-round anti-dpa attack function.\\\\ 0: Controlled by register\\\\ 1-3: The higher the value is, the more pseudo-rounds are inserted to the xts-aes calculation.\\\\"]
pub type XTS_DPA_PSEUDO_LEVEL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Represents whether reading of individual eFuse block(block4~block10) is disabled or enabled.\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
    #[inline(always)]
    pub fn rd_dis(&self) -> RD_DIS_R {
        RD_DIS_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Represents whether to enable PVT power glitch monitor function.\\\\1: Enable. \\\\0: Disable\\\\"]
    #[inline(always)]
    pub fn pvt_glitch_en(&self) -> PVT_GLITCH_EN_R {
        PVT_GLITCH_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents whether icache is disabled or enabled.\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
    #[inline(always)]
    pub fn dis_icache(&self) -> DIS_ICACHE_R {
        DIS_ICACHE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents whether the USB-to-JTAG function in USB Serial/JTAG is disabled. \\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
    #[inline(always)]
    pub fn dis_usb_jtag(&self) -> DIS_USB_JTAG_R {
        DIS_USB_JTAG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents whether to enable power glitch function.\\\\"]
    #[inline(always)]
    pub fn powerglitch_en(&self) -> POWERGLITCH_EN_R {
        POWERGLITCH_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents whether to disable USB-Serial-JTAG.\\\\"]
    #[inline(always)]
    pub fn dis_usb_serial_jtag(&self) -> DIS_USB_SERIAL_JTAG_R {
        DIS_USB_SERIAL_JTAG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents whether the function that forces chip into Download mode is disabled. \\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
    #[inline(always)]
    pub fn dis_force_download(&self) -> DIS_FORCE_DOWNLOAD_R {
        DIS_FORCE_DOWNLOAD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents accessing MSPI flash/MSPI RAM by SYS AXI matrix is disabled during boot_mode_download.\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
    #[inline(always)]
    pub fn spi_download_mspi_dis(&self) -> SPI_DOWNLOAD_MSPI_DIS_R {
        SPI_DOWNLOAD_MSPI_DIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents whether TWAI function is disabled. \\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
    #[inline(always)]
    pub fn dis_twai(&self) -> DIS_TWAI_R {
        DIS_TWAI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Represents whether the selection of a JTAG signal source through the strapping pin value is enabled when both EFUSE_DIS_PAD_JTAG and EFUSE_DIS_USB_JTAG are configured to 0. For more information, please refer to Chapter Placeholder.\\\\ 1: Enabled\\\\ 0: Disabled\\\\"]
    #[inline(always)]
    pub fn jtag_sel_enable(&self) -> JTAG_SEL_ENABLE_R {
        JTAG_SEL_ENABLE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Represents whether PAD JTAG is disabled in the soft way. It can be restarted via HMAC. \\\\ Odd count of bits with a value of 1: Disabled\\\\ Even count of bits with a value of 1: Enabled\\\\"]
    #[inline(always)]
    pub fn soft_dis_jtag(&self) -> SOFT_DIS_JTAG_R {
        SOFT_DIS_JTAG_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Represents whether PAD JTAG is disabled in the hard way (permanently).\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
    #[inline(always)]
    pub fn dis_pad_jtag(&self) -> DIS_PAD_JTAG_R {
        DIS_PAD_JTAG_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents whether flash encryption is disabled (except in SPI boot mode).\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
    #[inline(always)]
    pub fn dis_download_manual_encrypt(&self) -> DIS_DOWNLOAD_MANUAL_ENCRYPT_R {
        DIS_DOWNLOAD_MANUAL_ENCRYPT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Represents single-end input threshold vrefh, 1.76 V to 2 V with step of 80 mV, stored in eFuse.\\\\"]
    #[inline(always)]
    pub fn usb_drefh(&self) -> USB_DREFH_R {
        USB_DREFH_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Represents single-end input threshold vrefl, 0.8 V to 1.04 V with step of 80 mV, stored in eFuse.\\\\"]
    #[inline(always)]
    pub fn usb_drefl(&self) -> USB_DREFL_R {
        USB_DREFL_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - Represents whether the D+ and D- pins is exchanged.\\\\ 1: Exchanged\\\\ 0: Not exchanged\\\\"]
    #[inline(always)]
    pub fn usb_exchg_pins(&self) -> USB_EXCHG_PINS_R {
        USB_EXCHG_PINS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Represents whether vdd spi pin is functioned as gpio.\\\\ 1: Functioned\\\\ 0: Not functioned\\\\"]
    #[inline(always)]
    pub fn vdd_spi_as_gpio(&self) -> VDD_SPI_AS_GPIO_R {
        VDD_SPI_AS_GPIO_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Represents the configuration of the curve of ECDSA calculation.\\\\ 0: Only enable P256\\\\ 1: Only enable P192\\\\ 2: Both enable P256 and P192\\\\ 3: Only enable P256\\\\"]
    #[inline(always)]
    pub fn ecdsa_curve_mode(&self) -> ECDSA_CURVE_MODE_R {
        ECDSA_CURVE_MODE_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - Represents whether to permanently turn on ECC const-time mode.\\\\ 0: Disabled\\\\ 1: Enabled\\\\"]
    #[inline(always)]
    pub fn ecc_force_const_time(&self) -> ECC_FORCE_CONST_TIME_R {
        ECC_FORCE_CONST_TIME_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Represents control method of xts pseudo-round anti-dpa attack function.\\\\ 0: Controlled by register\\\\ 1-3: The higher the value is, the more pseudo-rounds are inserted to the xts-aes calculation.\\\\"]
    #[inline(always)]
    pub fn xts_dpa_pseudo_level(&self) -> XTS_DPA_PSEUDO_LEVEL_R {
        XTS_DPA_PSEUDO_LEVEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA0")
            .field("rd_dis", &self.rd_dis())
            .field("pvt_glitch_en", &self.pvt_glitch_en())
            .field("dis_icache", &self.dis_icache())
            .field("dis_usb_jtag", &self.dis_usb_jtag())
            .field("powerglitch_en", &self.powerglitch_en())
            .field("dis_usb_serial_jtag", &self.dis_usb_serial_jtag())
            .field("dis_force_download", &self.dis_force_download())
            .field("spi_download_mspi_dis", &self.spi_download_mspi_dis())
            .field("dis_twai", &self.dis_twai())
            .field("jtag_sel_enable", &self.jtag_sel_enable())
            .field("soft_dis_jtag", &self.soft_dis_jtag())
            .field("dis_pad_jtag", &self.dis_pad_jtag())
            .field(
                "dis_download_manual_encrypt",
                &self.dis_download_manual_encrypt(),
            )
            .field("usb_drefh", &self.usb_drefh())
            .field("usb_drefl", &self.usb_drefl())
            .field("usb_exchg_pins", &self.usb_exchg_pins())
            .field("vdd_spi_as_gpio", &self.vdd_spi_as_gpio())
            .field("ecdsa_curve_mode", &self.ecdsa_curve_mode())
            .field("ecc_force_const_time", &self.ecc_force_const_time())
            .field("xts_dpa_pseudo_level", &self.xts_dpa_pseudo_level())
            .finish()
    }
}
#[doc = "Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA0_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data0::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA0_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA0 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA0_SPEC {}
