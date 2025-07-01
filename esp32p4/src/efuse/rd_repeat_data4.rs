#[doc = "Register `RD_REPEAT_DATA4` reader"]
pub type R = crate::R<RD_REPEAT_DATA4_SPEC>;
#[doc = "Field `_0PXA_TIEH_SEL_0` reader - TBD"]
pub type _0PXA_TIEH_SEL_0_R = crate::FieldReader;
#[doc = "Field `_0PXA_TIEH_SEL_1` reader - TBD."]
pub type _0PXA_TIEH_SEL_1_R = crate::FieldReader;
#[doc = "Field `_0PXA_TIEH_SEL_2` reader - TBD."]
pub type _0PXA_TIEH_SEL_2_R = crate::FieldReader;
#[doc = "Field `_0PXA_TIEH_SEL_3` reader - TBD."]
pub type _0PXA_TIEH_SEL_3_R = crate::FieldReader;
#[doc = "Field `KM_DISABLE_DEPLOY_MODE` reader - TBD."]
pub type KM_DISABLE_DEPLOY_MODE_R = crate::FieldReader;
#[doc = "Field `USB_DEVICE_DREFL` reader - Represents the usb device single-end input low threhold, 0.8 V to 1.04 V with step of 80 mV."]
pub type USB_DEVICE_DREFL_R = crate::FieldReader;
#[doc = "Field `USB_OTG11_DREFL` reader - Represents the usb otg11 single-end input low threhold, 0.8 V to 1.04 V with step of 80 mV."]
pub type USB_OTG11_DREFL_R = crate::FieldReader;
#[doc = "Field `HP_PWR_SRC_SEL` reader - HP system power source select. 0:LDO. 1: DCDC."]
pub type HP_PWR_SRC_SEL_R = crate::BitReader;
#[doc = "Field `DCDC_VSET_EN` reader - Select dcdc vset use efuse_dcdc_vset."]
pub type DCDC_VSET_EN_R = crate::BitReader;
#[doc = "Field `DIS_WDT` reader - Set this bit to disable watch dog."]
pub type DIS_WDT_R = crate::BitReader;
#[doc = "Field `DIS_SWD` reader - Set this bit to disable super-watchdog."]
pub type DIS_SWD_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - TBD"]
    #[inline(always)]
    pub fn _0pxa_tieh_sel_0(&self) -> _0PXA_TIEH_SEL_0_R {
        _0PXA_TIEH_SEL_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - TBD."]
    #[inline(always)]
    pub fn _0pxa_tieh_sel_1(&self) -> _0PXA_TIEH_SEL_1_R {
        _0PXA_TIEH_SEL_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - TBD."]
    #[inline(always)]
    pub fn _0pxa_tieh_sel_2(&self) -> _0PXA_TIEH_SEL_2_R {
        _0PXA_TIEH_SEL_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TBD."]
    #[inline(always)]
    pub fn _0pxa_tieh_sel_3(&self) -> _0PXA_TIEH_SEL_3_R {
        _0PXA_TIEH_SEL_3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - TBD."]
    #[inline(always)]
    pub fn km_disable_deploy_mode(&self) -> KM_DISABLE_DEPLOY_MODE_R {
        KM_DISABLE_DEPLOY_MODE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Represents the usb device single-end input low threhold, 0.8 V to 1.04 V with step of 80 mV."]
    #[inline(always)]
    pub fn usb_device_drefl(&self) -> USB_DEVICE_DREFL_R {
        USB_DEVICE_DREFL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Represents the usb otg11 single-end input low threhold, 0.8 V to 1.04 V with step of 80 mV."]
    #[inline(always)]
    pub fn usb_otg11_drefl(&self) -> USB_OTG11_DREFL_R {
        USB_OTG11_DREFL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 18 - HP system power source select. 0:LDO. 1: DCDC."]
    #[inline(always)]
    pub fn hp_pwr_src_sel(&self) -> HP_PWR_SRC_SEL_R {
        HP_PWR_SRC_SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Select dcdc vset use efuse_dcdc_vset."]
    #[inline(always)]
    pub fn dcdc_vset_en(&self) -> DCDC_VSET_EN_R {
        DCDC_VSET_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to disable watch dog."]
    #[inline(always)]
    pub fn dis_wdt(&self) -> DIS_WDT_R {
        DIS_WDT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to disable super-watchdog."]
    #[inline(always)]
    pub fn dis_swd(&self) -> DIS_SWD_R {
        DIS_SWD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA4")
            .field("_0pxa_tieh_sel_0", &self._0pxa_tieh_sel_0())
            .field("_0pxa_tieh_sel_1", &self._0pxa_tieh_sel_1())
            .field("_0pxa_tieh_sel_2", &self._0pxa_tieh_sel_2())
            .field("_0pxa_tieh_sel_3", &self._0pxa_tieh_sel_3())
            .field("km_disable_deploy_mode", &self.km_disable_deploy_mode())
            .field("usb_device_drefl", &self.usb_device_drefl())
            .field("usb_otg11_drefl", &self.usb_otg11_drefl())
            .field("hp_pwr_src_sel", &self.hp_pwr_src_sel())
            .field("dcdc_vset_en", &self.dcdc_vset_en())
            .field("dis_wdt", &self.dis_wdt())
            .field("dis_swd", &self.dis_swd())
            .finish()
    }
}
#[doc = "BLOCK0 data register 5.\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA4_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data4::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA4_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA4 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA4_SPEC {}
