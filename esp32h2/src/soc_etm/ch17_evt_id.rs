#[doc = "Register `CH17_EVT_ID` reader"]
pub struct R(crate::R<CH17_EVT_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH17_EVT_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH17_EVT_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH17_EVT_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH17_EVT_ID` writer"]
pub struct W(crate::W<CH17_EVT_ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH17_EVT_ID_SPEC>;
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
impl From<crate::W<CH17_EVT_ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH17_EVT_ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH17_EVT_ID` reader - ch17_evt_id"]
pub type CH17_EVT_ID_R = crate::FieldReader;
#[doc = "Field `CH17_EVT_ID` writer - ch17_evt_id"]
pub type CH17_EVT_ID_W<'a, const O: u8> = crate::FieldWriter<'a, CH17_EVT_ID_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ch17_evt_id"]
    #[inline(always)]
    pub fn ch17_evt_id(&self) -> CH17_EVT_ID_R {
        CH17_EVT_ID_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH17_EVT_ID")
            .field(
                "ch17_evt_id",
                &format_args!("{}", self.ch17_evt_id().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH17_EVT_ID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - ch17_evt_id"]
    #[inline(always)]
    #[must_use]
    pub fn ch17_evt_id(&mut self) -> CH17_EVT_ID_W<0> {
        CH17_EVT_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel17 event id register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch17_evt_id](index.html) module"]
pub struct CH17_EVT_ID_SPEC;
impl crate::RegisterSpec for CH17_EVT_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch17_evt_id::R](R) reader structure"]
impl crate::Readable for CH17_EVT_ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch17_evt_id::W](W) writer structure"]
impl crate::Writable for CH17_EVT_ID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH17_EVT_ID to value 0"]
impl crate::Resettable for CH17_EVT_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
