#[doc = "Register `INTERRUPT_ENA` reader"]
pub struct R(crate::R<INTERRUPT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERRUPT_ENA` writer"]
pub struct W(crate::W<INTERRUPT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPT_ENA_SPEC>;
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
impl From<crate::W<INTERRUPT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERRUPT_ENA` reader - Set this bit to 1 to enable the RSA interrupt. This option is enabled by default."]
pub type INTERRUPT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `INTERRUPT_ENA` writer - Set this bit to 1 to enable the RSA interrupt. This option is enabled by default."]
pub type INTERRUPT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERRUPT_ENA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to 1 to enable the RSA interrupt. This option is enabled by default."]
    #[inline(always)]
    pub fn interrupt_ena(&self) -> INTERRUPT_ENA_R {
        INTERRUPT_ENA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to 1 to enable the RSA interrupt. This option is enabled by default."]
    #[inline(always)]
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
#[doc = "RSA interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_ena](index.html) module"]
pub struct INTERRUPT_ENA_SPEC;
impl crate::RegisterSpec for INTERRUPT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt_ena::R](R) reader structure"]
impl crate::Readable for INTERRUPT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interrupt_ena::W](W) writer structure"]
impl crate::Writable for INTERRUPT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTERRUPT_ENA to value 0"]
impl crate::Resettable for INTERRUPT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
