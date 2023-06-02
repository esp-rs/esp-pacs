#[doc = "Register `IDINTEN` reader"]
pub struct R(crate::R<IDINTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDINTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDINTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDINTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDINTEN` writer"]
pub struct W(crate::W<IDINTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDINTEN_SPEC>;
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
impl From<crate::W<IDINTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDINTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI` reader - Transmit Interrupt Enable. When set with Normal Interrupt Summary Enable, Transmit Interrupt is enabled. When reset, Transmit Interrupt is disabled."]
pub type TI_R = crate::BitReader;
#[doc = "Field `TI` writer - Transmit Interrupt Enable. When set with Normal Interrupt Summary Enable, Transmit Interrupt is enabled. When reset, Transmit Interrupt is disabled."]
pub type TI_W<'a, const O: u8> = crate::BitWriter<'a, IDINTEN_SPEC, O>;
#[doc = "Field `RI` reader - Receive Interrupt Enable. When set with Normal Interrupt Summary Enable, Receive Interrupt is enabled. When reset, Receive Interrupt is disabled."]
pub type RI_R = crate::BitReader;
#[doc = "Field `RI` writer - Receive Interrupt Enable. When set with Normal Interrupt Summary Enable, Receive Interrupt is enabled. When reset, Receive Interrupt is disabled."]
pub type RI_W<'a, const O: u8> = crate::BitWriter<'a, IDINTEN_SPEC, O>;
#[doc = "Field `FBE` reader - Fatal Bus Error Enable. When set with Abnormal Interrupt Summary Enable, the Fatal Bus Error Interrupt is enabled. When reset, Fatal Bus Error Enable Interrupt is disabled."]
pub type FBE_R = crate::BitReader;
#[doc = "Field `FBE` writer - Fatal Bus Error Enable. When set with Abnormal Interrupt Summary Enable, the Fatal Bus Error Interrupt is enabled. When reset, Fatal Bus Error Enable Interrupt is disabled."]
pub type FBE_W<'a, const O: u8> = crate::BitWriter<'a, IDINTEN_SPEC, O>;
#[doc = "Field `DU` reader - Descriptor Unavailable Interrupt. When set along with Abnormal Interrupt Summary Enable, the DU interrupt is enabled."]
pub type DU_R = crate::BitReader;
#[doc = "Field `DU` writer - Descriptor Unavailable Interrupt. When set along with Abnormal Interrupt Summary Enable, the DU interrupt is enabled."]
pub type DU_W<'a, const O: u8> = crate::BitWriter<'a, IDINTEN_SPEC, O>;
#[doc = "Field `CES` reader - Card Error summary Interrupt Enable. When set, it enables the Card Interrupt summary."]
pub type CES_R = crate::BitReader;
#[doc = "Field `CES` writer - Card Error summary Interrupt Enable. When set, it enables the Card Interrupt summary."]
pub type CES_W<'a, const O: u8> = crate::BitWriter<'a, IDINTEN_SPEC, O>;
#[doc = "Field `NI` reader - Normal Interrupt Summary Enable. When set, a normal interrupt is enabled. When reset, a normal interrupt is disabled. This bit enables the following bits: IDINTEN\\[0\\]: Transmit Interrupt; IDINTEN\\[1\\]: Receive Interrupt."]
pub type NI_R = crate::BitReader;
#[doc = "Field `NI` writer - Normal Interrupt Summary Enable. When set, a normal interrupt is enabled. When reset, a normal interrupt is disabled. This bit enables the following bits: IDINTEN\\[0\\]: Transmit Interrupt; IDINTEN\\[1\\]: Receive Interrupt."]
pub type NI_W<'a, const O: u8> = crate::BitWriter<'a, IDINTEN_SPEC, O>;
#[doc = "Field `AI` reader - Abnormal Interrupt Summary Enable. When set, an abnormal interrupt is enabled. This bit enables the following bits: IDINTEN\\[2\\]: Fatal Bus Error Interrupt; IDINTEN\\[4\\]: DU Interrupt."]
pub type AI_R = crate::BitReader;
#[doc = "Field `AI` writer - Abnormal Interrupt Summary Enable. When set, an abnormal interrupt is enabled. This bit enables the following bits: IDINTEN\\[2\\]: Fatal Bus Error Interrupt; IDINTEN\\[4\\]: DU Interrupt."]
pub type AI_W<'a, const O: u8> = crate::BitWriter<'a, IDINTEN_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable. When set with Normal Interrupt Summary Enable, Transmit Interrupt is enabled. When reset, Transmit Interrupt is disabled."]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Interrupt Enable. When set with Normal Interrupt Summary Enable, Receive Interrupt is enabled. When reset, Receive Interrupt is disabled."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fatal Bus Error Enable. When set with Abnormal Interrupt Summary Enable, the Fatal Bus Error Interrupt is enabled. When reset, Fatal Bus Error Enable Interrupt is disabled."]
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt. When set along with Abnormal Interrupt Summary Enable, the DU interrupt is enabled."]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Card Error summary Interrupt Enable. When set, it enables the Card Interrupt summary."]
    #[inline(always)]
    pub fn ces(&self) -> CES_R {
        CES_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary Enable. When set, a normal interrupt is enabled. When reset, a normal interrupt is disabled. This bit enables the following bits: IDINTEN\\[0\\]: Transmit Interrupt; IDINTEN\\[1\\]: Receive Interrupt."]
    #[inline(always)]
    pub fn ni(&self) -> NI_R {
        NI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary Enable. When set, an abnormal interrupt is enabled. This bit enables the following bits: IDINTEN\\[2\\]: Fatal Bus Error Interrupt; IDINTEN\\[4\\]: DU Interrupt."]
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDINTEN")
            .field("ti", &format_args!("{}", self.ti().bit()))
            .field("ri", &format_args!("{}", self.ri().bit()))
            .field("fbe", &format_args!("{}", self.fbe().bit()))
            .field("du", &format_args!("{}", self.du().bit()))
            .field("ces", &format_args!("{}", self.ces().bit()))
            .field("ni", &format_args!("{}", self.ni().bit()))
            .field("ai", &format_args!("{}", self.ai().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IDINTEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable. When set with Normal Interrupt Summary Enable, Transmit Interrupt is enabled. When reset, Transmit Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ti(&mut self) -> TI_W<0> {
        TI_W::new(self)
    }
    #[doc = "Bit 1 - Receive Interrupt Enable. When set with Normal Interrupt Summary Enable, Receive Interrupt is enabled. When reset, Receive Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RI_W<1> {
        RI_W::new(self)
    }
    #[doc = "Bit 2 - Fatal Bus Error Enable. When set with Abnormal Interrupt Summary Enable, the Fatal Bus Error Interrupt is enabled. When reset, Fatal Bus Error Enable Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fbe(&mut self) -> FBE_W<2> {
        FBE_W::new(self)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt. When set along with Abnormal Interrupt Summary Enable, the DU interrupt is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn du(&mut self) -> DU_W<4> {
        DU_W::new(self)
    }
    #[doc = "Bit 5 - Card Error summary Interrupt Enable. When set, it enables the Card Interrupt summary."]
    #[inline(always)]
    #[must_use]
    pub fn ces(&mut self) -> CES_W<5> {
        CES_W::new(self)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary Enable. When set, a normal interrupt is enabled. When reset, a normal interrupt is disabled. This bit enables the following bits: IDINTEN\\[0\\]: Transmit Interrupt; IDINTEN\\[1\\]: Receive Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ni(&mut self) -> NI_W<8> {
        NI_W::new(self)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary Enable. When set, an abnormal interrupt is enabled. This bit enables the following bits: IDINTEN\\[2\\]: Fatal Bus Error Interrupt; IDINTEN\\[4\\]: DU Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AI_W<9> {
        AI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IDMAC interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idinten](index.html) module"]
pub struct IDINTEN_SPEC;
impl crate::RegisterSpec for IDINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idinten::R](R) reader structure"]
impl crate::Readable for IDINTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idinten::W](W) writer structure"]
impl crate::Writable for IDINTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDINTEN to value 0"]
impl crate::Resettable for IDINTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
