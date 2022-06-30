#[doc = "Register `TOUCH_PAD5` reader"]
pub struct R(crate::R<TOUCH_PAD5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_PAD5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_PAD5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_PAD5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_PAD5` writer"]
pub struct W(crate::W<TOUCH_PAD5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_PAD5_SPEC>;
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
impl From<crate::W<TOUCH_PAD5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_PAD5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FUN_IE` reader - input enable in work mode"]
pub type FUN_IE_R = crate::BitReader<bool>;
#[doc = "Field `FUN_IE` writer - input enable in work mode"]
pub type FUN_IE_W<'a> = crate::BitWriter<'a, u32, TOUCH_PAD5_SPEC, bool, 13>;
#[doc = "Field `SLP_OE` reader - output enable in sleep mode"]
pub type SLP_OE_R = crate::BitReader<bool>;
#[doc = "Field `SLP_OE` writer - output enable in sleep mode"]
pub type SLP_OE_W<'a> = crate::BitWriter<'a, u32, TOUCH_PAD5_SPEC, bool, 14>;
#[doc = "Field `SLP_IE` reader - input enable in sleep mode"]
pub type SLP_IE_R = crate::BitReader<bool>;
#[doc = "Field `SLP_IE` writer - input enable in sleep mode"]
pub type SLP_IE_W<'a> = crate::BitWriter<'a, u32, TOUCH_PAD5_SPEC, bool, 15>;
#[doc = "Field `SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type SLP_SEL_R = crate::BitReader<bool>;
#[doc = "Field `SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type SLP_SEL_W<'a> = crate::BitWriter<'a, u32, TOUCH_PAD5_SPEC, bool, 16>;
#[doc = "Field `FUN_SEL` reader - function sel"]
pub type FUN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FUN_SEL` writer - function sel"]
pub type FUN_SEL_W<'a> = crate::FieldWriter<'a, u32, TOUCH_PAD5_SPEC, u8, u8, 2, 17>;
#[doc = "Field `MUX_SEL` reader - 1: use RTC GPIO,0: use digital GPIO"]
pub type MUX_SEL_R = crate::BitReader<bool>;
#[doc = "Field `MUX_SEL` writer - 1: use RTC GPIO,0: use digital GPIO"]
pub type MUX_SEL_W<'a> = crate::BitWriter<'a, u32, TOUCH_PAD5_SPEC, bool, 19>;
#[doc = "Field `XPD` reader - TOUCH_XPD"]
pub type XPD_R = crate::BitReader<bool>;
#[doc = "Field `XPD` writer - TOUCH_XPD"]
pub type XPD_W<'a> = crate::BitWriter<'a, u32, TOUCH_PAD5_SPEC, bool, 20>;
#[doc = "Field `TIE_OPT` reader - TOUCH_TIE_OPT"]
pub type TIE_OPT_R = crate::BitReader<bool>;
#[doc = "Field `TIE_OPT` writer - TOUCH_TIE_OPT"]
pub type TIE_OPT_W<'a> = crate::BitWriter<'a, u32, TOUCH_PAD5_SPEC, bool, 21>;
#[doc = "Field `START` reader - TOUCH_START"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - TOUCH_START"]
pub type START_W<'a> = crate::BitWriter<'a, u32, TOUCH_PAD5_SPEC, bool, 22>;
#[doc = "Field `RUE` reader - RUE"]
pub type RUE_R = crate::BitReader<bool>;
#[doc = "Field `RUE` writer - RUE"]
pub type RUE_W<'a> = crate::BitWriter<'a, u32, TOUCH_PAD5_SPEC, bool, 27>;
#[doc = "Field `RDE` reader - RDE"]
pub type RDE_R = crate::BitReader<bool>;
#[doc = "Field `RDE` writer - RDE"]
pub type RDE_W<'a> = crate::BitWriter<'a, u32, TOUCH_PAD5_SPEC, bool, 28>;
#[doc = "Field `DRV` reader - DRV"]
pub type DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRV` writer - DRV"]
pub type DRV_W<'a> = crate::FieldWriter<'a, u32, TOUCH_PAD5_SPEC, u8, u8, 2, 29>;
impl R {
    #[doc = "Bit 13 - input enable in work mode"]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - output enable in sleep mode"]
    #[inline(always)]
    pub fn slp_oe(&self) -> SLP_OE_R {
        SLP_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - input enable in sleep mode"]
    #[inline(always)]
    pub fn slp_ie(&self) -> SLP_IE_R {
        SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - function sel"]
    #[inline(always)]
    pub fn fun_sel(&self) -> FUN_SEL_R {
        FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn mux_sel(&self) -> MUX_SEL_R {
        MUX_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TOUCH_XPD"]
    #[inline(always)]
    pub fn xpd(&self) -> XPD_R {
        XPD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TOUCH_TIE_OPT"]
    #[inline(always)]
    pub fn tie_opt(&self) -> TIE_OPT_R {
        TIE_OPT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TOUCH_START"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 27 - RUE"]
    #[inline(always)]
    pub fn rue(&self) -> RUE_R {
        RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RDE"]
    #[inline(always)]
    pub fn rde(&self) -> RDE_R {
        RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - DRV"]
    #[inline(always)]
    pub fn drv(&self) -> DRV_R {
        DRV_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 13 - input enable in work mode"]
    #[inline(always)]
    pub fn fun_ie(&mut self) -> FUN_IE_W {
        FUN_IE_W::new(self)
    }
    #[doc = "Bit 14 - output enable in sleep mode"]
    #[inline(always)]
    pub fn slp_oe(&mut self) -> SLP_OE_W {
        SLP_OE_W::new(self)
    }
    #[doc = "Bit 15 - input enable in sleep mode"]
    #[inline(always)]
    pub fn slp_ie(&mut self) -> SLP_IE_W {
        SLP_IE_W::new(self)
    }
    #[doc = "Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn slp_sel(&mut self) -> SLP_SEL_W {
        SLP_SEL_W::new(self)
    }
    #[doc = "Bits 17:18 - function sel"]
    #[inline(always)]
    pub fn fun_sel(&mut self) -> FUN_SEL_W {
        FUN_SEL_W::new(self)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn mux_sel(&mut self) -> MUX_SEL_W {
        MUX_SEL_W::new(self)
    }
    #[doc = "Bit 20 - TOUCH_XPD"]
    #[inline(always)]
    pub fn xpd(&mut self) -> XPD_W {
        XPD_W::new(self)
    }
    #[doc = "Bit 21 - TOUCH_TIE_OPT"]
    #[inline(always)]
    pub fn tie_opt(&mut self) -> TIE_OPT_W {
        TIE_OPT_W::new(self)
    }
    #[doc = "Bit 22 - TOUCH_START"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W::new(self)
    }
    #[doc = "Bit 27 - RUE"]
    #[inline(always)]
    pub fn rue(&mut self) -> RUE_W {
        RUE_W::new(self)
    }
    #[doc = "Bit 28 - RDE"]
    #[inline(always)]
    pub fn rde(&mut self) -> RDE_W {
        RDE_W::new(self)
    }
    #[doc = "Bits 29:30 - DRV"]
    #[inline(always)]
    pub fn drv(&mut self) -> DRV_W {
        DRV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure RTC PAD5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_pad5](index.html) module"]
pub struct TOUCH_PAD5_SPEC;
impl crate::RegisterSpec for TOUCH_PAD5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_pad5::R](R) reader structure"]
impl crate::Readable for TOUCH_PAD5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_pad5::W](W) writer structure"]
impl crate::Writable for TOUCH_PAD5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOUCH_PAD5 to value 0x5000_0000"]
impl crate::Resettable for TOUCH_PAD5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5000_0000
    }
}
