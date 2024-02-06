#[doc = "Register `RD_REPEAT_ERR4` reader"]
pub type R = crate::R<RD_REPEAT_ERR4_SPEC>;
#[doc = "Field `_0PXA_TIEH_SEL_0_ERR` reader - Indicates a programming error of 0PXA_TIEH_SEL_0."]
pub type _0PXA_TIEH_SEL_0_ERR_R = crate::FieldReader;
#[doc = "Field `_0PXA_TIEH_SEL_1_ERR` reader - Indicates a programming error of 0PXA_TIEH_SEL_1."]
pub type _0PXA_TIEH_SEL_1_ERR_R = crate::FieldReader;
#[doc = "Field `_0PXA_TIEH_SEL_2_ERR` reader - Indicates a programming error of 0PXA_TIEH_SEL_2."]
pub type _0PXA_TIEH_SEL_2_ERR_R = crate::FieldReader;
#[doc = "Field `_0PXA_TIEH_SEL_3_ERR` reader - Indicates a programming error of 0PXA_TIEH_SEL_3."]
pub type _0PXA_TIEH_SEL_3_ERR_R = crate::FieldReader;
#[doc = "Field `KM_DISABLE_DEPLOY_MODE_ERR` reader - TBD."]
pub type KM_DISABLE_DEPLOY_MODE_ERR_R = crate::FieldReader;
#[doc = "Field `USB_DEVICE_DREFL_ERR` reader - Indicates a programming error of USB_DEVICE_DREFL."]
pub type USB_DEVICE_DREFL_ERR_R = crate::FieldReader;
#[doc = "Field `USB_OTG11_DREFL_ERR` reader - Indicates a programming error of USB_OTG11_DREFL."]
pub type USB_OTG11_DREFL_ERR_R = crate::FieldReader;
#[doc = "Field `HP_PWR_SRC_SEL_ERR` reader - Indicates a programming error of HP_PWR_SRC_SEL."]
pub type HP_PWR_SRC_SEL_ERR_R = crate::BitReader;
#[doc = "Field `DCDC_VSET_EN_ERR` reader - Indicates a programming error of DCDC_VSET_EN."]
pub type DCDC_VSET_EN_ERR_R = crate::BitReader;
#[doc = "Field `DIS_WDT_ERR` reader - Indicates a programming error of DIS_WDT."]
pub type DIS_WDT_ERR_R = crate::BitReader;
#[doc = "Field `DIS_SWD_ERR` reader - Indicates a programming error of DIS_SWD."]
pub type DIS_SWD_ERR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Indicates a programming error of 0PXA_TIEH_SEL_0."]
    #[inline(always)]
    pub fn _0pxa_tieh_sel_0_err(&self) -> _0PXA_TIEH_SEL_0_ERR_R {
        _0PXA_TIEH_SEL_0_ERR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Indicates a programming error of 0PXA_TIEH_SEL_1."]
    #[inline(always)]
    pub fn _0pxa_tieh_sel_1_err(&self) -> _0PXA_TIEH_SEL_1_ERR_R {
        _0PXA_TIEH_SEL_1_ERR_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Indicates a programming error of 0PXA_TIEH_SEL_2."]
    #[inline(always)]
    pub fn _0pxa_tieh_sel_2_err(&self) -> _0PXA_TIEH_SEL_2_ERR_R {
        _0PXA_TIEH_SEL_2_ERR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Indicates a programming error of 0PXA_TIEH_SEL_3."]
    #[inline(always)]
    pub fn _0pxa_tieh_sel_3_err(&self) -> _0PXA_TIEH_SEL_3_ERR_R {
        _0PXA_TIEH_SEL_3_ERR_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - TBD."]
    #[inline(always)]
    pub fn km_disable_deploy_mode_err(&self) -> KM_DISABLE_DEPLOY_MODE_ERR_R {
        KM_DISABLE_DEPLOY_MODE_ERR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Indicates a programming error of USB_DEVICE_DREFL."]
    #[inline(always)]
    pub fn usb_device_drefl_err(&self) -> USB_DEVICE_DREFL_ERR_R {
        USB_DEVICE_DREFL_ERR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Indicates a programming error of USB_OTG11_DREFL."]
    #[inline(always)]
    pub fn usb_otg11_drefl_err(&self) -> USB_OTG11_DREFL_ERR_R {
        USB_OTG11_DREFL_ERR_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 18 - Indicates a programming error of HP_PWR_SRC_SEL."]
    #[inline(always)]
    pub fn hp_pwr_src_sel_err(&self) -> HP_PWR_SRC_SEL_ERR_R {
        HP_PWR_SRC_SEL_ERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Indicates a programming error of DCDC_VSET_EN."]
    #[inline(always)]
    pub fn dcdc_vset_en_err(&self) -> DCDC_VSET_EN_ERR_R {
        DCDC_VSET_EN_ERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Indicates a programming error of DIS_WDT."]
    #[inline(always)]
    pub fn dis_wdt_err(&self) -> DIS_WDT_ERR_R {
        DIS_WDT_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Indicates a programming error of DIS_SWD."]
    #[inline(always)]
    pub fn dis_swd_err(&self) -> DIS_SWD_ERR_R {
        DIS_SWD_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_ERR4")
            .field(
                "_0pxa_tieh_sel_0_err",
                &format_args!("{}", self._0pxa_tieh_sel_0_err().bits()),
            )
            .field(
                "_0pxa_tieh_sel_1_err",
                &format_args!("{}", self._0pxa_tieh_sel_1_err().bits()),
            )
            .field(
                "_0pxa_tieh_sel_2_err",
                &format_args!("{}", self._0pxa_tieh_sel_2_err().bits()),
            )
            .field(
                "_0pxa_tieh_sel_3_err",
                &format_args!("{}", self._0pxa_tieh_sel_3_err().bits()),
            )
            .field(
                "km_disable_deploy_mode_err",
                &format_args!("{}", self.km_disable_deploy_mode_err().bits()),
            )
            .field(
                "usb_device_drefl_err",
                &format_args!("{}", self.usb_device_drefl_err().bits()),
            )
            .field(
                "usb_otg11_drefl_err",
                &format_args!("{}", self.usb_otg11_drefl_err().bits()),
            )
            .field(
                "hp_pwr_src_sel_err",
                &format_args!("{}", self.hp_pwr_src_sel_err().bit()),
            )
            .field(
                "dcdc_vset_en_err",
                &format_args!("{}", self.dcdc_vset_en_err().bit()),
            )
            .field("dis_wdt_err", &format_args!("{}", self.dis_wdt_err().bit()))
            .field("dis_swd_err", &format_args!("{}", self.dis_swd_err().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_REPEAT_ERR4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Programming error record register 4 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_ERR4_SPEC;
impl crate::RegisterSpec for RD_REPEAT_ERR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_err4::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR4_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_ERR4 to value 0"]
impl crate::Resettable for RD_REPEAT_ERR4_SPEC {
    const RESET_VALUE: u32 = 0;
}
