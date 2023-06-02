#[doc = "Register `CMD15` reader"]
pub struct R(crate::R<CMD15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD15` writer"]
pub struct W(crate::W<CMD15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD15_SPEC>;
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
impl From<crate::W<CMD15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND15` reader - command15"]
pub type COMMAND15_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMMAND15` writer - command15"]
pub type COMMAND15_W<'a, const O: u8> = crate::FieldWriter<'a, CMD15_SPEC, 14, O, u16, u16>;
#[doc = "Field `COMMAND15_DONE` reader - command15_done"]
pub type COMMAND15_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - command15"]
    #[inline(always)]
    pub fn command15(&self) -> COMMAND15_R {
        COMMAND15_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command15_done"]
    #[inline(always)]
    pub fn command15_done(&self) -> COMMAND15_DONE_R {
        COMMAND15_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD15")
            .field("command15", &format_args!("{}", self.command15().bits()))
            .field(
                "command15_done",
                &format_args!("{}", self.command15_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD15_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13 - command15"]
    #[inline(always)]
    #[must_use]
    pub fn command15(&mut self) -> COMMAND15_W<0> {
        COMMAND15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c commond15 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd15](index.html) module"]
pub struct CMD15_SPEC;
impl crate::RegisterSpec for CMD15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd15::R](R) reader structure"]
impl crate::Readable for CMD15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd15::W](W) writer structure"]
impl crate::Writable for CMD15_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD15 to value 0"]
impl crate::Resettable for CMD15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
