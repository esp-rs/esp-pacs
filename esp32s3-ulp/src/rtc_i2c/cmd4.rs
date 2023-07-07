#[doc = "Register `CMD4` reader"]
pub struct R(crate::R<CMD4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD4` writer"]
pub struct W(crate::W<CMD4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD4_SPEC>;
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
impl From<crate::W<CMD4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND4` reader - command4"]
pub type COMMAND4_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND4` writer - command4"]
pub type COMMAND4_W<'a, const O: u8> = crate::FieldWriter<'a, CMD4_SPEC, 14, O, u16>;
#[doc = "Field `COMMAND4_DONE` reader - command4_done"]
pub type COMMAND4_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - command4"]
    #[inline(always)]
    pub fn command4(&self) -> COMMAND4_R {
        COMMAND4_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command4_done"]
    #[inline(always)]
    pub fn command4_done(&self) -> COMMAND4_DONE_R {
        COMMAND4_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD4")
            .field("command4", &format_args!("{}", self.command4().bits()))
            .field(
                "command4_done",
                &format_args!("{}", self.command4_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13 - command4"]
    #[inline(always)]
    #[must_use]
    pub fn command4(&mut self) -> COMMAND4_W<0> {
        COMMAND4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c commond4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd4](index.html) module"]
pub struct CMD4_SPEC;
impl crate::RegisterSpec for CMD4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd4::R](R) reader structure"]
impl crate::Readable for CMD4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd4::W](W) writer structure"]
impl crate::Writable for CMD4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD4 to value 0x0901"]
impl crate::Resettable for CMD4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0901;
}
