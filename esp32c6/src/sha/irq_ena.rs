#[doc = "Register `IRQ_ENA` reader"]
pub struct R(crate::R<IRQ_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQ_ENA` writer"]
pub struct W(crate::W<IRQ_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_ENA_SPEC>;
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
impl From<crate::W<IRQ_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERRUPT_ENA` reader - Sha interrupt enable register. 1'b0: disable(default). 1'b1: enable."]
pub type INTERRUPT_ENA_R = crate::BitReader;
#[doc = "Field `INTERRUPT_ENA` writer - Sha interrupt enable register. 1'b0: disable(default). 1'b1: enable."]
pub type INTERRUPT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, IRQ_ENA_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Sha interrupt enable register. 1'b0: disable(default). 1'b1: enable."]
    #[inline(always)]
    pub fn interrupt_ena(&self) -> INTERRUPT_ENA_R {
        INTERRUPT_ENA_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ_ENA")
            .field(
                "interrupt_ena",
                &format_args!("{}", self.interrupt_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IRQ_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Sha interrupt enable register. 1'b0: disable(default). 1'b1: enable."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_ena(&mut self) -> INTERRUPT_ENA_W<0> {
        INTERRUPT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_ena](index.html) module"]
pub struct IRQ_ENA_SPEC;
impl crate::RegisterSpec for IRQ_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_ena::R](R) reader structure"]
impl crate::Readable for IRQ_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_ena::W](W) writer structure"]
impl crate::Writable for IRQ_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQ_ENA to value 0"]
impl crate::Resettable for IRQ_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
