#[doc = "Register `UART_ID` reader"]
pub struct R(crate::R<UART_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_ID` writer"]
pub struct W(crate::W<UART_ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_ID_SPEC>;
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
impl From<crate::W<UART_ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `uart_id` reader - "]
pub type UART_ID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `uart_id` writer - "]
pub type UART_ID_W<'a, const O: u8> = crate::FieldWriter<'a, UART_ID_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn uart_id(&self) -> UART_ID_R {
        UART_ID_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_ID")
            .field("uart_id", &format_args!("{}", self.uart_id().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UART_ID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn uart_id(&mut self) -> UART_ID_W<0> {
        UART_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART_ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_id](index.html) module"]
pub struct UART_ID_SPEC;
impl crate::RegisterSpec for UART_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_id::R](R) reader structure"]
impl crate::Readable for UART_ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_id::W](W) writer structure"]
impl crate::Writable for UART_ID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_ID to value 0"]
impl crate::Resettable for UART_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
