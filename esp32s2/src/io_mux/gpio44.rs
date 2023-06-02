#[doc = "Register `GPIO44` reader"]
pub struct R(crate::R<GPIO44_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO44_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO44_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO44_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO44` writer"]
pub struct W(crate::W<GPIO44_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO44_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GPIO44_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO44_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCU_OE` reader - Output enable of the pin in sleep mode. 1: Output enabled. 0: Output disabled."]
pub type MCU_OE_R = crate::BitReader;
#[doc = "Field `MCU_OE` writer - Output enable of the pin in sleep mode. 1: Output enabled. 0: Output disabled."]
pub type MCU_OE_W<'a, const O: u8> = crate::BitWriter<'a, GPIO44_SPEC, O>;
#[doc = "Field `SLP_SEL` reader - Sleep mode selection of this pin. Set to 1 to put the pin in sleep mode."]
pub type SLP_SEL_R = crate::BitReader;
#[doc = "Field `SLP_SEL` writer - Sleep mode selection of this pin. Set to 1 to put the pin in sleep mode."]
pub type SLP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, GPIO44_SPEC, O>;
#[doc = "Field `MCU_WPD` reader - Pull-down enable of the pin during sleep mode. 1: Internal pull-down enabled. 0: internal pull-down disabled."]
pub type MCU_WPD_R = crate::BitReader;
#[doc = "Field `MCU_WPD` writer - Pull-down enable of the pin during sleep mode. 1: Internal pull-down enabled. 0: internal pull-down disabled."]
pub type MCU_WPD_W<'a, const O: u8> = crate::BitWriter<'a, GPIO44_SPEC, O>;
#[doc = "Field `MCU_WPU` reader - Pull-up enable of the pin during sleep mode. 1: Internal pull-up enabled. 0: Internal pull-up disabled."]
pub type MCU_WPU_R = crate::BitReader;
#[doc = "Field `MCU_WPU` writer - Pull-up enable of the pin during sleep mode. 1: Internal pull-up enabled. 0: Internal pull-up disabled."]
pub type MCU_WPU_W<'a, const O: u8> = crate::BitWriter<'a, GPIO44_SPEC, O>;
#[doc = "Field `MCU_IE` reader - Input enable of the pin during sleep mode. 1: Input enabled. 0: Input disabled."]
pub type MCU_IE_R = crate::BitReader;
#[doc = "Field `MCU_IE` writer - Input enable of the pin during sleep mode. 1: Input enabled. 0: Input disabled."]
pub type MCU_IE_W<'a, const O: u8> = crate::BitWriter<'a, GPIO44_SPEC, O>;
#[doc = "Field `FUN_WPD` reader - Pull-down enable of the pin. 1: Internal pull-down enabled. 0: internal pull-down disabled."]
pub type FUN_WPD_R = crate::BitReader;
#[doc = "Field `FUN_WPD` writer - Pull-down enable of the pin. 1: Internal pull-down enabled. 0: internal pull-down disabled."]
pub type FUN_WPD_W<'a, const O: u8> = crate::BitWriter<'a, GPIO44_SPEC, O>;
#[doc = "Field `FUN_WPU` reader - Pull-up enable of the pin. 1: Internal pull-up enabled. 0: Internal pull-up disabled."]
pub type FUN_WPU_R = crate::BitReader;
#[doc = "Field `FUN_WPU` writer - Pull-up enable of the pin. 1: Internal pull-up enabled. 0: Internal pull-up disabled."]
pub type FUN_WPU_W<'a, const O: u8> = crate::BitWriter<'a, GPIO44_SPEC, O>;
#[doc = "Field `FUN_IE` reader - Input enable of the pin. 1: Input enabled. 0: Input disabled."]
pub type FUN_IE_R = crate::BitReader;
#[doc = "Field `FUN_IE` writer - Input enable of the pin. 1: Input enabled. 0: Input disabled."]
pub type FUN_IE_W<'a, const O: u8> = crate::BitWriter<'a, GPIO44_SPEC, O>;
#[doc = "Field `FUN_DRV` reader - Select the drive strength of the pin. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
pub type FUN_DRV_R = crate::FieldReader;
#[doc = "Field `FUN_DRV` writer - Select the drive strength of the pin. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
pub type FUN_DRV_W<'a, const O: u8> = crate::FieldWriter<'a, GPIO44_SPEC, 2, O>;
#[doc = "Field `MCU_SEL` reader - Select IO MUX function for this signal. 0: Select Function 1. 1: Select Function 2, etc."]
pub type MCU_SEL_R = crate::FieldReader;
#[doc = "Field `MCU_SEL` writer - Select IO MUX function for this signal. 0: Select Function 1. 1: Select Function 2, etc."]
pub type MCU_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, GPIO44_SPEC, 3, O>;
#[doc = "Field `FILTER_EN` reader - Enable filter for pin input signals. 1: Filter enabled. 2: Filter disabled."]
pub type FILTER_EN_R = crate::BitReader;
#[doc = "Field `FILTER_EN` writer - Enable filter for pin input signals. 1: Filter enabled. 2: Filter disabled."]
pub type FILTER_EN_W<'a, const O: u8> = crate::BitWriter<'a, GPIO44_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Output enable of the pin in sleep mode. 1: Output enabled. 0: Output disabled."]
    #[inline(always)]
    pub fn mcu_oe(&self) -> MCU_OE_R {
        MCU_OE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sleep mode selection of this pin. Set to 1 to put the pin in sleep mode."]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pull-down enable of the pin during sleep mode. 1: Internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn mcu_wpd(&self) -> MCU_WPD_R {
        MCU_WPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pull-up enable of the pin during sleep mode. 1: Internal pull-up enabled. 0: Internal pull-up disabled."]
    #[inline(always)]
    pub fn mcu_wpu(&self) -> MCU_WPU_R {
        MCU_WPU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input enable of the pin during sleep mode. 1: Input enabled. 0: Input disabled."]
    #[inline(always)]
    pub fn mcu_ie(&self) -> MCU_IE_R {
        MCU_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Pull-down enable of the pin. 1: Internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn fun_wpd(&self) -> FUN_WPD_R {
        FUN_WPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pull-up enable of the pin. 1: Internal pull-up enabled. 0: Internal pull-up disabled."]
    #[inline(always)]
    pub fn fun_wpu(&self) -> FUN_WPU_R {
        FUN_WPU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input enable of the pin. 1: Input enabled. 0: Input disabled."]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Select the drive strength of the pin. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
    #[inline(always)]
    pub fn fun_drv(&self) -> FUN_DRV_R {
        FUN_DRV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Select IO MUX function for this signal. 0: Select Function 1. 1: Select Function 2, etc."]
    #[inline(always)]
    pub fn mcu_sel(&self) -> MCU_SEL_R {
        MCU_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Enable filter for pin input signals. 1: Filter enabled. 2: Filter disabled."]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO44")
            .field("mcu_oe", &format_args!("{}", self.mcu_oe().bit()))
            .field("slp_sel", &format_args!("{}", self.slp_sel().bit()))
            .field("mcu_wpd", &format_args!("{}", self.mcu_wpd().bit()))
            .field("mcu_wpu", &format_args!("{}", self.mcu_wpu().bit()))
            .field("mcu_ie", &format_args!("{}", self.mcu_ie().bit()))
            .field("fun_wpd", &format_args!("{}", self.fun_wpd().bit()))
            .field("fun_wpu", &format_args!("{}", self.fun_wpu().bit()))
            .field("fun_ie", &format_args!("{}", self.fun_ie().bit()))
            .field("fun_drv", &format_args!("{}", self.fun_drv().bits()))
            .field("mcu_sel", &format_args!("{}", self.mcu_sel().bits()))
            .field("filter_en", &format_args!("{}", self.filter_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO44_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Output enable of the pin in sleep mode. 1: Output enabled. 0: Output disabled."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_oe(&mut self) -> MCU_OE_W<0> {
        MCU_OE_W::new(self)
    }
    #[doc = "Bit 1 - Sleep mode selection of this pin. Set to 1 to put the pin in sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn slp_sel(&mut self) -> SLP_SEL_W<1> {
        SLP_SEL_W::new(self)
    }
    #[doc = "Bit 2 - Pull-down enable of the pin during sleep mode. 1: Internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_wpd(&mut self) -> MCU_WPD_W<2> {
        MCU_WPD_W::new(self)
    }
    #[doc = "Bit 3 - Pull-up enable of the pin during sleep mode. 1: Internal pull-up enabled. 0: Internal pull-up disabled."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_wpu(&mut self) -> MCU_WPU_W<3> {
        MCU_WPU_W::new(self)
    }
    #[doc = "Bit 4 - Input enable of the pin during sleep mode. 1: Input enabled. 0: Input disabled."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_ie(&mut self) -> MCU_IE_W<4> {
        MCU_IE_W::new(self)
    }
    #[doc = "Bit 7 - Pull-down enable of the pin. 1: Internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fun_wpd(&mut self) -> FUN_WPD_W<7> {
        FUN_WPD_W::new(self)
    }
    #[doc = "Bit 8 - Pull-up enable of the pin. 1: Internal pull-up enabled. 0: Internal pull-up disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fun_wpu(&mut self) -> FUN_WPU_W<8> {
        FUN_WPU_W::new(self)
    }
    #[doc = "Bit 9 - Input enable of the pin. 1: Input enabled. 0: Input disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fun_ie(&mut self) -> FUN_IE_W<9> {
        FUN_IE_W::new(self)
    }
    #[doc = "Bits 10:11 - Select the drive strength of the pin. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
    #[inline(always)]
    #[must_use]
    pub fn fun_drv(&mut self) -> FUN_DRV_W<10> {
        FUN_DRV_W::new(self)
    }
    #[doc = "Bits 12:14 - Select IO MUX function for this signal. 0: Select Function 1. 1: Select Function 2, etc."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_sel(&mut self) -> MCU_SEL_W<12> {
        MCU_SEL_W::new(self)
    }
    #[doc = "Bit 15 - Enable filter for pin input signals. 1: Filter enabled. 2: Filter disabled."]
    #[inline(always)]
    #[must_use]
    pub fn filter_en(&mut self) -> FILTER_EN_W<15> {
        FILTER_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register for pin GPIO44\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio44](index.html) module"]
pub struct GPIO44_SPEC;
impl crate::RegisterSpec for GPIO44_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio44::R](R) reader structure"]
impl crate::Readable for GPIO44_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio44::W](W) writer structure"]
impl crate::Writable for GPIO44_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO44 to value 0x0b00"]
impl crate::Resettable for GPIO44_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b00;
}
