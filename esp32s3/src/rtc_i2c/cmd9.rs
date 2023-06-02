#[doc = "Register `CMD9` reader"]
pub struct R(crate::R<CMD9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD9` writer"]
pub struct W(crate::W<CMD9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD9_SPEC>;
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
impl From<crate::W<CMD9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND9` reader - command9"]
pub type COMMAND9_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMMAND9` writer - command9"]
pub type COMMAND9_W<'a, const O: u8> = crate::FieldWriter<'a, CMD9_SPEC, 14, O, u16, u16>;
#[doc = "Field `COMMAND9_DONE` reader - command9_done"]
pub type COMMAND9_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - command9"]
    #[inline(always)]
    pub fn command9(&self) -> COMMAND9_R {
        COMMAND9_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command9_done"]
    #[inline(always)]
    pub fn command9_done(&self) -> COMMAND9_DONE_R {
        COMMAND9_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD9")
            .field("command9", &format_args!("{}", self.command9().bits()))
            .field(
                "command9_done",
                &format_args!("{}", self.command9_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD9_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13 - command9"]
    #[inline(always)]
    #[must_use]
    pub fn command9(&mut self) -> COMMAND9_W<0> {
        COMMAND9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c commond9 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd9](index.html) module"]
pub struct CMD9_SPEC;
impl crate::RegisterSpec for CMD9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd9::R](R) reader structure"]
impl crate::Readable for CMD9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd9::W](W) writer structure"]
impl crate::Writable for CMD9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD9 to value 0x0903"]
impl crate::Resettable for CMD9_SPEC {
    const RESET_VALUE: Self::Ux = 0x0903;
}
