#[doc = "Register `CMD14` reader"]
pub struct R(crate::R<CMD14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD14` writer"]
pub struct W(crate::W<CMD14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD14_SPEC>;
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
impl From<crate::W<CMD14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND14` reader - command14"]
pub type COMMAND14_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMMAND14` writer - command14"]
pub type COMMAND14_W<'a, const O: u8> = crate::FieldWriter<'a, CMD14_SPEC, 14, O, u16, u16>;
#[doc = "Field `COMMAND14_DONE` reader - command14_done"]
pub type COMMAND14_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - command14"]
    #[inline(always)]
    pub fn command14(&self) -> COMMAND14_R {
        COMMAND14_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command14_done"]
    #[inline(always)]
    pub fn command14_done(&self) -> COMMAND14_DONE_R {
        COMMAND14_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD14")
            .field("command14", &format_args!("{}", self.command14().bits()))
            .field(
                "command14_done",
                &format_args!("{}", self.command14_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD14_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13 - command14"]
    #[inline(always)]
    #[must_use]
    pub fn command14(&mut self) -> COMMAND14_W<0> {
        COMMAND14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c commond14 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd14](index.html) module"]
pub struct CMD14_SPEC;
impl crate::RegisterSpec for CMD14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd14::R](R) reader structure"]
impl crate::Readable for CMD14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd14::W](W) writer structure"]
impl crate::Writable for CMD14_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD14 to value 0"]
impl crate::Resettable for CMD14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
