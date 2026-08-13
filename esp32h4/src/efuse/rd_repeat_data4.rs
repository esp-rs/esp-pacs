#[doc = "Register `RD_REPEAT_DATA4` reader"]
pub type R = crate::R<RD_REPEAT_DATA4_SPEC>;
#[doc = "Field `USB_DREFH` reader - Represents the single-end input threhold vrefh of USB_SERIAL_JTAG PHY, 1.76 V to 2 V with step of 80 mV."]
pub type USB_DREFH_R = crate::FieldReader;
#[doc = "Field `USB_DREFL` reader - Represents the single-end input threhold vrefl of USB_SERIAL_JTAG PHY, 1.76 V to 2 V with step of 80 mV."]
pub type USB_DREFL_R = crate::FieldReader;
#[doc = "Field `USB_OTG_FS_DREFH` reader - Represents the single-end input threhold vrefh of USB_OTG_FS PHY, 1.76 V to 2 V with step of 80 mV."]
pub type USB_OTG_FS_DREFH_R = crate::FieldReader;
#[doc = "Field `USB_OTG_FS_DREFL` reader - Represents the single-end input threhold vrefl of USB_OTG_FS PHY, 1.76 V to 2 V with step of 80 mV."]
pub type USB_OTG_FS_DREFL_R = crate::FieldReader;
#[doc = "Field `USB_EXCHG_PINS` reader - Represents whether the D+ and D- pins of USB_SERIAL_JTAG PHY is exchanged.\\\\ 1: exchanged\\\\ 0: not exchanged\\\\"]
pub type USB_EXCHG_PINS_R = crate::BitReader;
#[doc = "Field `USB_OTG_FS_EXCHG_PINS` reader - Represents whether the D+ and D- pins of USB_OTG_FS PHY is exchanged.\\\\ 1: exchanged\\\\ 0: not exchanged\\\\"]
pub type USB_OTG_FS_EXCHG_PINS_R = crate::BitReader;
#[doc = "Field `USB_PHY_SEL` reader - Represents whether to exchange the USB_SERIAL_JTAG PHY with USB_OTG_FS PHY. \\\\ 1: exchanged. \\\\ 0: not exchanged."]
pub type USB_PHY_SEL_R = crate::BitReader;
#[doc = "Field `SOFT_DIS_JTAG` reader - Represents whether JTAG is disabled in soft way.\\\\ Odd number: disabled\\\\ Even number: enabled\\\\"]
pub type SOFT_DIS_JTAG_R = crate::FieldReader;
#[doc = "Field `IO_LDO_ADJUST` reader - Represents configuration of IO LDO mode and voltage.\\\\"]
pub type IO_LDO_ADJUST_R = crate::FieldReader;
#[doc = "Field `IO_LDO_1P8` reader - Represents select IO LDO voltage to 1.8V or 3.3V.\\\\ 1: 1.8V\\\\ 0: 3.3V\\\\"]
pub type IO_LDO_1P8_R = crate::BitReader;
#[doc = "Field `DCDC_CCM_EN` reader - Represents whether change DCDC to CCM mode"]
pub type DCDC_CCM_EN_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Represents the single-end input threhold vrefh of USB_SERIAL_JTAG PHY, 1.76 V to 2 V with step of 80 mV."]
    #[inline(always)]
    pub fn usb_drefh(&self) -> USB_DREFH_R {
        USB_DREFH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Represents the single-end input threhold vrefl of USB_SERIAL_JTAG PHY, 1.76 V to 2 V with step of 80 mV."]
    #[inline(always)]
    pub fn usb_drefl(&self) -> USB_DREFL_R {
        USB_DREFL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Represents the single-end input threhold vrefh of USB_OTG_FS PHY, 1.76 V to 2 V with step of 80 mV."]
    #[inline(always)]
    pub fn usb_otg_fs_drefh(&self) -> USB_OTG_FS_DREFH_R {
        USB_OTG_FS_DREFH_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Represents the single-end input threhold vrefl of USB_OTG_FS PHY, 1.76 V to 2 V with step of 80 mV."]
    #[inline(always)]
    pub fn usb_otg_fs_drefl(&self) -> USB_OTG_FS_DREFL_R {
        USB_OTG_FS_DREFL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Represents whether the D+ and D- pins of USB_SERIAL_JTAG PHY is exchanged.\\\\ 1: exchanged\\\\ 0: not exchanged\\\\"]
    #[inline(always)]
    pub fn usb_exchg_pins(&self) -> USB_EXCHG_PINS_R {
        USB_EXCHG_PINS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents whether the D+ and D- pins of USB_OTG_FS PHY is exchanged.\\\\ 1: exchanged\\\\ 0: not exchanged\\\\"]
    #[inline(always)]
    pub fn usb_otg_fs_exchg_pins(&self) -> USB_OTG_FS_EXCHG_PINS_R {
        USB_OTG_FS_EXCHG_PINS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents whether to exchange the USB_SERIAL_JTAG PHY with USB_OTG_FS PHY. \\\\ 1: exchanged. \\\\ 0: not exchanged."]
    #[inline(always)]
    pub fn usb_phy_sel(&self) -> USB_PHY_SEL_R {
        USB_PHY_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Represents whether JTAG is disabled in soft way.\\\\ Odd number: disabled\\\\ Even number: enabled\\\\"]
    #[inline(always)]
    pub fn soft_dis_jtag(&self) -> SOFT_DIS_JTAG_R {
        SOFT_DIS_JTAG_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:21 - Represents configuration of IO LDO mode and voltage.\\\\"]
    #[inline(always)]
    pub fn io_ldo_adjust(&self) -> IO_LDO_ADJUST_R {
        IO_LDO_ADJUST_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bit 22 - Represents select IO LDO voltage to 1.8V or 3.3V.\\\\ 1: 1.8V\\\\ 0: 3.3V\\\\"]
    #[inline(always)]
    pub fn io_ldo_1p8(&self) -> IO_LDO_1P8_R {
        IO_LDO_1P8_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Represents whether change DCDC to CCM mode"]
    #[inline(always)]
    pub fn dcdc_ccm_en(&self) -> DCDC_CCM_EN_R {
        DCDC_CCM_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA4")
            .field("usb_drefh", &self.usb_drefh())
            .field("usb_drefl", &self.usb_drefl())
            .field("usb_otg_fs_drefh", &self.usb_otg_fs_drefh())
            .field("usb_otg_fs_drefl", &self.usb_otg_fs_drefl())
            .field("usb_exchg_pins", &self.usb_exchg_pins())
            .field("usb_otg_fs_exchg_pins", &self.usb_otg_fs_exchg_pins())
            .field("usb_phy_sel", &self.usb_phy_sel())
            .field("soft_dis_jtag", &self.soft_dis_jtag())
            .field("io_ldo_adjust", &self.io_ldo_adjust())
            .field("io_ldo_1p8", &self.io_ldo_1p8())
            .field("dcdc_ccm_en", &self.dcdc_ccm_en())
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
