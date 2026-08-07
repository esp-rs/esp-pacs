#[doc = "Register `RD_REPEAT_DATA_ERR4` reader"]
pub type R = crate::R<RD_REPEAT_DATA_ERR4_SPEC>;
#[doc = "Field `USB_DREFH_ERR` reader - Represents the programming error of EFUSE_USB_DREFH"]
pub type USB_DREFH_ERR_R = crate::FieldReader;
#[doc = "Field `USB_DREFL_ERR` reader - Represents the programming error of EFUSE_USB_DREFL"]
pub type USB_DREFL_ERR_R = crate::FieldReader;
#[doc = "Field `USB_OTG_FS_DREFH_ERR` reader - Represents the programming error of EFUSE_USB_OTG_FS_DREFH"]
pub type USB_OTG_FS_DREFH_ERR_R = crate::FieldReader;
#[doc = "Field `USB_OTG_FS_DREFL_ERR` reader - Represents the programming error of EFUSE_USB_OTG_FS_DREFL"]
pub type USB_OTG_FS_DREFL_ERR_R = crate::FieldReader;
#[doc = "Field `USB_EXCHG_PINS_ERR` reader - Represents the programming error of EFUSE_USB_EXCHG_PINS"]
pub type USB_EXCHG_PINS_ERR_R = crate::BitReader;
#[doc = "Field `USB_OTG_FS_EXCHG_PINS_ERR` reader - Represents the programming error of EFUSE_USB_OTG_FS_EXCHG_PINS"]
pub type USB_OTG_FS_EXCHG_PINS_ERR_R = crate::BitReader;
#[doc = "Field `USB_PHY_SEL_ERR` reader - Represents the programming error of EFUSE_USB_PHY_SEL"]
pub type USB_PHY_SEL_ERR_R = crate::BitReader;
#[doc = "Field `SOFT_DIS_JTAG_ERR` reader - Represents the programming error of EFUSE_SOFT_DIS_JTAG"]
pub type SOFT_DIS_JTAG_ERR_R = crate::FieldReader;
#[doc = "Field `IO_LDO_ADJUST_ERR` reader - Represents the programming error of EFUSE_IO_LDO_ADJUST"]
pub type IO_LDO_ADJUST_ERR_R = crate::FieldReader;
#[doc = "Field `IO_LDO_1P8_ERR` reader - Represents the programming error of EFUSE_IO_LDO_1P8"]
pub type IO_LDO_1P8_ERR_R = crate::BitReader;
#[doc = "Field `DCDC_CCM_EN_ERR` reader - Represents the programming error of EFUSE_DCDC_CCM_EN"]
pub type DCDC_CCM_EN_ERR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Represents the programming error of EFUSE_USB_DREFH"]
    #[inline(always)]
    pub fn usb_drefh_err(&self) -> USB_DREFH_ERR_R {
        USB_DREFH_ERR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Represents the programming error of EFUSE_USB_DREFL"]
    #[inline(always)]
    pub fn usb_drefl_err(&self) -> USB_DREFL_ERR_R {
        USB_DREFL_ERR_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Represents the programming error of EFUSE_USB_OTG_FS_DREFH"]
    #[inline(always)]
    pub fn usb_otg_fs_drefh_err(&self) -> USB_OTG_FS_DREFH_ERR_R {
        USB_OTG_FS_DREFH_ERR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Represents the programming error of EFUSE_USB_OTG_FS_DREFL"]
    #[inline(always)]
    pub fn usb_otg_fs_drefl_err(&self) -> USB_OTG_FS_DREFL_ERR_R {
        USB_OTG_FS_DREFL_ERR_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Represents the programming error of EFUSE_USB_EXCHG_PINS"]
    #[inline(always)]
    pub fn usb_exchg_pins_err(&self) -> USB_EXCHG_PINS_ERR_R {
        USB_EXCHG_PINS_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents the programming error of EFUSE_USB_OTG_FS_EXCHG_PINS"]
    #[inline(always)]
    pub fn usb_otg_fs_exchg_pins_err(&self) -> USB_OTG_FS_EXCHG_PINS_ERR_R {
        USB_OTG_FS_EXCHG_PINS_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents the programming error of EFUSE_USB_PHY_SEL"]
    #[inline(always)]
    pub fn usb_phy_sel_err(&self) -> USB_PHY_SEL_ERR_R {
        USB_PHY_SEL_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Represents the programming error of EFUSE_SOFT_DIS_JTAG"]
    #[inline(always)]
    pub fn soft_dis_jtag_err(&self) -> SOFT_DIS_JTAG_ERR_R {
        SOFT_DIS_JTAG_ERR_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:21 - Represents the programming error of EFUSE_IO_LDO_ADJUST"]
    #[inline(always)]
    pub fn io_ldo_adjust_err(&self) -> IO_LDO_ADJUST_ERR_R {
        IO_LDO_ADJUST_ERR_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bit 22 - Represents the programming error of EFUSE_IO_LDO_1P8"]
    #[inline(always)]
    pub fn io_ldo_1p8_err(&self) -> IO_LDO_1P8_ERR_R {
        IO_LDO_1P8_ERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Represents the programming error of EFUSE_DCDC_CCM_EN"]
    #[inline(always)]
    pub fn dcdc_ccm_en_err(&self) -> DCDC_CCM_EN_ERR_R {
        DCDC_CCM_EN_ERR_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA_ERR4")
            .field("usb_drefh_err", &self.usb_drefh_err())
            .field("usb_drefl_err", &self.usb_drefl_err())
            .field("usb_otg_fs_drefh_err", &self.usb_otg_fs_drefh_err())
            .field("usb_otg_fs_drefl_err", &self.usb_otg_fs_drefl_err())
            .field("usb_exchg_pins_err", &self.usb_exchg_pins_err())
            .field(
                "usb_otg_fs_exchg_pins_err",
                &self.usb_otg_fs_exchg_pins_err(),
            )
            .field("usb_phy_sel_err", &self.usb_phy_sel_err())
            .field("soft_dis_jtag_err", &self.soft_dis_jtag_err())
            .field("io_ldo_adjust_err", &self.io_ldo_adjust_err())
            .field("io_ldo_1p8_err", &self.io_ldo_1p8_err())
            .field("dcdc_ccm_en_err", &self.dcdc_ccm_en_err())
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
