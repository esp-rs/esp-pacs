#[doc = "Register `SEARCH_POS` reader"]
pub struct R(crate::R<SEARCH_POS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEARCH_POS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEARCH_POS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEARCH_POS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEARCH_POS` writer"]
pub struct W(crate::W<SEARCH_POS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEARCH_POS_SPEC>;
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
impl From<crate::W<SEARCH_POS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEARCH_POS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEARCH_POS` reader - Configures the starting address to start search. This field should be used together with RSA_SEARCH_ENABLE. The field is only valid when RSA_SEARCH_ENABLE is high."]
pub type SEARCH_POS_R = crate::FieldReader<u16>;
#[doc = "Field `SEARCH_POS` writer - Configures the starting address to start search. This field should be used together with RSA_SEARCH_ENABLE. The field is only valid when RSA_SEARCH_ENABLE is high."]
pub type SEARCH_POS_W<'a, const O: u8> = crate::FieldWriter<'a, SEARCH_POS_SPEC, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Configures the starting address to start search. This field should be used together with RSA_SEARCH_ENABLE. The field is only valid when RSA_SEARCH_ENABLE is high."]
    #[inline(always)]
    pub fn search_pos(&self) -> SEARCH_POS_R {
        SEARCH_POS_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEARCH_POS")
            .field("search_pos", &format_args!("{}", self.search_pos().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SEARCH_POS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11 - Configures the starting address to start search. This field should be used together with RSA_SEARCH_ENABLE. The field is only valid when RSA_SEARCH_ENABLE is high."]
    #[inline(always)]
    #[must_use]
    pub fn search_pos(&mut self) -> SEARCH_POS_W<0> {
        SEARCH_POS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures the search position\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [search_pos](index.html) module"]
pub struct SEARCH_POS_SPEC;
impl crate::RegisterSpec for SEARCH_POS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [search_pos::R](R) reader structure"]
impl crate::Readable for SEARCH_POS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [search_pos::W](W) writer structure"]
impl crate::Writable for SEARCH_POS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEARCH_POS to value 0"]
impl crate::Resettable for SEARCH_POS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
