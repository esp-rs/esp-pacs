#[doc = "Register `TIMER_SYNCI_CFG` reader"]
pub struct R(crate::R<TIMER_SYNCI_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_SYNCI_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_SYNCI_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_SYNCI_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER_SYNCI_CFG` writer"]
pub struct W(crate::W<TIMER_SYNCI_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_SYNCI_CFG_SPEC>;
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
impl From<crate::W<TIMER_SYNCI_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_SYNCI_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER0_SYNCISEL` reader - "]
pub type TIMER0_SYNCISEL_R = crate::FieldReader;
#[doc = "Field `TIMER0_SYNCISEL` writer - "]
pub type TIMER0_SYNCISEL_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER_SYNCI_CFG_SPEC, 3, O>;
#[doc = "Field `TIMER1_SYNCISEL` reader - "]
pub type TIMER1_SYNCISEL_R = crate::FieldReader;
#[doc = "Field `TIMER1_SYNCISEL` writer - "]
pub type TIMER1_SYNCISEL_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER_SYNCI_CFG_SPEC, 3, O>;
#[doc = "Field `TIMER2_SYNCISEL` reader - "]
pub type TIMER2_SYNCISEL_R = crate::FieldReader;
#[doc = "Field `TIMER2_SYNCISEL` writer - "]
pub type TIMER2_SYNCISEL_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER_SYNCI_CFG_SPEC, 3, O>;
#[doc = "Field `EXTERNAL_SYNCI0_INVERT` reader - "]
pub type EXTERNAL_SYNCI0_INVERT_R = crate::BitReader;
#[doc = "Field `EXTERNAL_SYNCI0_INVERT` writer - "]
pub type EXTERNAL_SYNCI0_INVERT_W<'a, const O: u8> = crate::BitWriter<'a, TIMER_SYNCI_CFG_SPEC, O>;
#[doc = "Field `EXTERNAL_SYNCI1_INVERT` reader - "]
pub type EXTERNAL_SYNCI1_INVERT_R = crate::BitReader;
#[doc = "Field `EXTERNAL_SYNCI1_INVERT` writer - "]
pub type EXTERNAL_SYNCI1_INVERT_W<'a, const O: u8> = crate::BitWriter<'a, TIMER_SYNCI_CFG_SPEC, O>;
#[doc = "Field `EXTERNAL_SYNCI2_INVERT` reader - "]
pub type EXTERNAL_SYNCI2_INVERT_R = crate::BitReader;
#[doc = "Field `EXTERNAL_SYNCI2_INVERT` writer - "]
pub type EXTERNAL_SYNCI2_INVERT_W<'a, const O: u8> = crate::BitWriter<'a, TIMER_SYNCI_CFG_SPEC, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn timer0_syncisel(&self) -> TIMER0_SYNCISEL_R {
        TIMER0_SYNCISEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn timer1_syncisel(&self) -> TIMER1_SYNCISEL_R {
        TIMER1_SYNCISEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn timer2_syncisel(&self) -> TIMER2_SYNCISEL_R {
        TIMER2_SYNCISEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn external_synci0_invert(&self) -> EXTERNAL_SYNCI0_INVERT_R {
        EXTERNAL_SYNCI0_INVERT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn external_synci1_invert(&self) -> EXTERNAL_SYNCI1_INVERT_R {
        EXTERNAL_SYNCI1_INVERT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn external_synci2_invert(&self) -> EXTERNAL_SYNCI2_INVERT_R {
        EXTERNAL_SYNCI2_INVERT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_SYNCI_CFG")
            .field(
                "timer0_syncisel",
                &format_args!("{}", self.timer0_syncisel().bits()),
            )
            .field(
                "timer1_syncisel",
                &format_args!("{}", self.timer1_syncisel().bits()),
            )
            .field(
                "timer2_syncisel",
                &format_args!("{}", self.timer2_syncisel().bits()),
            )
            .field(
                "external_synci0_invert",
                &format_args!("{}", self.external_synci0_invert().bit()),
            )
            .field(
                "external_synci1_invert",
                &format_args!("{}", self.external_synci1_invert().bit()),
            )
            .field(
                "external_synci2_invert",
                &format_args!("{}", self.external_synci2_invert().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER_SYNCI_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_syncisel(&mut self) -> TIMER0_SYNCISEL_W<0> {
        TIMER0_SYNCISEL_W::new(self)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_syncisel(&mut self) -> TIMER1_SYNCISEL_W<3> {
        TIMER1_SYNCISEL_W::new(self)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_syncisel(&mut self) -> TIMER2_SYNCISEL_W<6> {
        TIMER2_SYNCISEL_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn external_synci0_invert(&mut self) -> EXTERNAL_SYNCI0_INVERT_W<9> {
        EXTERNAL_SYNCI0_INVERT_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn external_synci1_invert(&mut self) -> EXTERNAL_SYNCI1_INVERT_W<10> {
        EXTERNAL_SYNCI1_INVERT_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn external_synci2_invert(&mut self) -> EXTERNAL_SYNCI2_INVERT_W<11> {
        EXTERNAL_SYNCI2_INVERT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_synci_cfg](index.html) module"]
pub struct TIMER_SYNCI_CFG_SPEC;
impl crate::RegisterSpec for TIMER_SYNCI_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_synci_cfg::R](R) reader structure"]
impl crate::Readable for TIMER_SYNCI_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_synci_cfg::W](W) writer structure"]
impl crate::Writable for TIMER_SYNCI_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER_SYNCI_CFG to value 0"]
impl crate::Resettable for TIMER_SYNCI_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
