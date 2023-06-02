#[doc = "Register `WAKEUP_STATE` reader"]
pub struct R(crate::R<WAKEUP_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKEUP_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKEUP_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKEUP_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKEUP_STATE` writer"]
pub struct W(crate::W<WAKEUP_STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKEUP_STATE_SPEC>;
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
impl From<crate::W<WAKEUP_STATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKEUP_STATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKEUP_ENA` reader - wakeup enable bitmap"]
pub type WAKEUP_ENA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WAKEUP_ENA` writer - wakeup enable bitmap"]
pub type WAKEUP_ENA_W<'a, const O: u8> = crate::FieldWriter<'a, WAKEUP_STATE_SPEC, 17, O, u32, u32>;
impl R {
    #[doc = "Bits 15:31 - wakeup enable bitmap"]
    #[inline(always)]
    pub fn wakeup_ena(&self) -> WAKEUP_ENA_R {
        WAKEUP_ENA_R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAKEUP_STATE")
            .field("wakeup_ena", &format_args!("{}", self.wakeup_ena().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WAKEUP_STATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 15:31 - wakeup enable bitmap"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_ena(&mut self) -> WAKEUP_ENA_W<15> {
        WAKEUP_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeup_state](index.html) module"]
pub struct WAKEUP_STATE_SPEC;
impl crate::RegisterSpec for WAKEUP_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakeup_state::R](R) reader structure"]
impl crate::Readable for WAKEUP_STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakeup_state::W](W) writer structure"]
impl crate::Writable for WAKEUP_STATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WAKEUP_STATE to value 0x0006_0000"]
impl crate::Resettable for WAKEUP_STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0006_0000;
}
