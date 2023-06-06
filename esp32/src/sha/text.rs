#[doc = "Register `TEXT%s` reader"]
pub struct R(crate::R<TEXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEXT%s` writer"]
pub struct W(crate::W<TEXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEXT_SPEC>;
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
impl From<crate::W<TEXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEXT` reader - SHA Message block and hash result register."]
pub type TEXT_R = crate::FieldReader<u32>;
#[doc = "Field `TEXT` writer - SHA Message block and hash result register."]
pub type TEXT_W<'a, const O: u8> = crate::FieldWriter<'a, TEXT_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - SHA Message block and hash result register."]
    #[inline(always)]
    pub fn text(&self) -> TEXT_R {
        TEXT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEXT")
            .field("text", &format_args!("{}", self.text().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TEXT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - SHA Message block and hash result register."]
    #[inline(always)]
    #[must_use]
    pub fn text(&mut self) -> TEXT_W<0> {
        TEXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [text](index.html) module"]
pub struct TEXT_SPEC;
impl crate::RegisterSpec for TEXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [text::R](R) reader structure"]
impl crate::Readable for TEXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [text::W](W) writer structure"]
impl crate::Writable for TEXT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEXT%s to value 0"]
impl crate::Resettable for TEXT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
