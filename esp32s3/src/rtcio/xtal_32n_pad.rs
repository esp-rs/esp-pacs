#[doc = "Register `XTAL_32N_PAD` reader"]
pub struct R(crate::R<XTAL_32N_PAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL_32N_PAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL_32N_PAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL_32N_PAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL_32N_PAD` writer"]
pub struct W(crate::W<XTAL_32N_PAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL_32N_PAD_SPEC>;
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
impl From<crate::W<XTAL_32N_PAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL_32N_PAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `X32N_FUN_IE` reader - input enable in work mode"]
pub type X32N_FUN_IE_R = crate::BitReader<bool>;
#[doc = "Field `X32N_FUN_IE` writer - input enable in work mode"]
pub type X32N_FUN_IE_W<'a> = crate::BitWriter<'a, u32, XTAL_32N_PAD_SPEC, bool, 13>;
#[doc = "Field `X32N_SLP_OE` reader - output enable in sleep mode"]
pub type X32N_SLP_OE_R = crate::BitReader<bool>;
#[doc = "Field `X32N_SLP_OE` writer - output enable in sleep mode"]
pub type X32N_SLP_OE_W<'a> = crate::BitWriter<'a, u32, XTAL_32N_PAD_SPEC, bool, 14>;
#[doc = "Field `X32N_SLP_IE` reader - input enable in sleep mode"]
pub type X32N_SLP_IE_R = crate::BitReader<bool>;
#[doc = "Field `X32N_SLP_IE` writer - input enable in sleep mode"]
pub type X32N_SLP_IE_W<'a> = crate::BitWriter<'a, u32, XTAL_32N_PAD_SPEC, bool, 15>;
#[doc = "Field `X32N_SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type X32N_SLP_SEL_R = crate::BitReader<bool>;
#[doc = "Field `X32N_SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type X32N_SLP_SEL_W<'a> = crate::BitWriter<'a, u32, XTAL_32N_PAD_SPEC, bool, 16>;
#[doc = "Field `X32N_FUN_SEL` reader - function sel"]
pub type X32N_FUN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `X32N_FUN_SEL` writer - function sel"]
pub type X32N_FUN_SEL_W<'a> = crate::FieldWriter<'a, u32, XTAL_32N_PAD_SPEC, u8, u8, 2, 17>;
#[doc = "Field `X32N_MUX_SEL` reader - 1: use RTC GPIO,0: use digital GPIO"]
pub type X32N_MUX_SEL_R = crate::BitReader<bool>;
#[doc = "Field `X32N_MUX_SEL` writer - 1: use RTC GPIO,0: use digital GPIO"]
pub type X32N_MUX_SEL_W<'a> = crate::BitWriter<'a, u32, XTAL_32N_PAD_SPEC, bool, 19>;
#[doc = "Field `X32N_RUE` reader - RUE"]
pub type X32N_RUE_R = crate::BitReader<bool>;
#[doc = "Field `X32N_RUE` writer - RUE"]
pub type X32N_RUE_W<'a> = crate::BitWriter<'a, u32, XTAL_32N_PAD_SPEC, bool, 27>;
#[doc = "Field `X32N_RDE` reader - RDE"]
pub type X32N_RDE_R = crate::BitReader<bool>;
#[doc = "Field `X32N_RDE` writer - RDE"]
pub type X32N_RDE_W<'a> = crate::BitWriter<'a, u32, XTAL_32N_PAD_SPEC, bool, 28>;
#[doc = "Field `X32N_DRV` reader - DRV"]
pub type X32N_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `X32N_DRV` writer - DRV"]
pub type X32N_DRV_W<'a> = crate::FieldWriter<'a, u32, XTAL_32N_PAD_SPEC, u8, u8, 2, 29>;
impl R {
    #[doc = "Bit 13 - input enable in work mode"]
    #[inline(always)]
    pub fn x32n_fun_ie(&self) -> X32N_FUN_IE_R {
        X32N_FUN_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - output enable in sleep mode"]
    #[inline(always)]
    pub fn x32n_slp_oe(&self) -> X32N_SLP_OE_R {
        X32N_SLP_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - input enable in sleep mode"]
    #[inline(always)]
    pub fn x32n_slp_ie(&self) -> X32N_SLP_IE_R {
        X32N_SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn x32n_slp_sel(&self) -> X32N_SLP_SEL_R {
        X32N_SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - function sel"]
    #[inline(always)]
    pub fn x32n_fun_sel(&self) -> X32N_FUN_SEL_R {
        X32N_FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn x32n_mux_sel(&self) -> X32N_MUX_SEL_R {
        X32N_MUX_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 27 - RUE"]
    #[inline(always)]
    pub fn x32n_rue(&self) -> X32N_RUE_R {
        X32N_RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RDE"]
    #[inline(always)]
    pub fn x32n_rde(&self) -> X32N_RDE_R {
        X32N_RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - DRV"]
    #[inline(always)]
    pub fn x32n_drv(&self) -> X32N_DRV_R {
        X32N_DRV_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 13 - input enable in work mode"]
    #[inline(always)]
    pub fn x32n_fun_ie(&mut self) -> X32N_FUN_IE_W {
        X32N_FUN_IE_W::new(self)
    }
    #[doc = "Bit 14 - output enable in sleep mode"]
    #[inline(always)]
    pub fn x32n_slp_oe(&mut self) -> X32N_SLP_OE_W {
        X32N_SLP_OE_W::new(self)
    }
    #[doc = "Bit 15 - input enable in sleep mode"]
    #[inline(always)]
    pub fn x32n_slp_ie(&mut self) -> X32N_SLP_IE_W {
        X32N_SLP_IE_W::new(self)
    }
    #[doc = "Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn x32n_slp_sel(&mut self) -> X32N_SLP_SEL_W {
        X32N_SLP_SEL_W::new(self)
    }
    #[doc = "Bits 17:18 - function sel"]
    #[inline(always)]
    pub fn x32n_fun_sel(&mut self) -> X32N_FUN_SEL_W {
        X32N_FUN_SEL_W::new(self)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn x32n_mux_sel(&mut self) -> X32N_MUX_SEL_W {
        X32N_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 27 - RUE"]
    #[inline(always)]
    pub fn x32n_rue(&mut self) -> X32N_RUE_W {
        X32N_RUE_W::new(self)
    }
    #[doc = "Bit 28 - RDE"]
    #[inline(always)]
    pub fn x32n_rde(&mut self) -> X32N_RDE_W {
        X32N_RDE_W::new(self)
    }
    #[doc = "Bits 29:30 - DRV"]
    #[inline(always)]
    pub fn x32n_drv(&mut self) -> X32N_DRV_W {
        X32N_DRV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure RTC PAD16\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal_32n_pad](index.html) module"]
pub struct XTAL_32N_PAD_SPEC;
impl crate::RegisterSpec for XTAL_32N_PAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal_32n_pad::R](R) reader structure"]
impl crate::Readable for XTAL_32N_PAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal_32n_pad::W](W) writer structure"]
impl crate::Writable for XTAL_32N_PAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTAL_32N_PAD to value 0x4000_0000"]
impl crate::Resettable for XTAL_32N_PAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0000
    }
}
