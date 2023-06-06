#[doc = "Register `CMD2` reader"]
pub struct R(crate::R<CMD2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD2` writer"]
pub struct W(crate::W<CMD2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD2_SPEC>;
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
impl From<crate::W<CMD2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND2` reader - command2"]
pub type COMMAND2_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND2` writer - command2"]
pub type COMMAND2_W<'a, const O: u8> = crate::FieldWriter<'a, CMD2_SPEC, 14, O, u16>;
#[doc = "Field `COMMAND2_DONE` reader - command2_done"]
pub type COMMAND2_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - command2"]
    #[inline(always)]
    pub fn command2(&self) -> COMMAND2_R {
        COMMAND2_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command2_done"]
    #[inline(always)]
    pub fn command2_done(&self) -> COMMAND2_DONE_R {
        COMMAND2_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD2")
            .field("command2", &format_args!("{}", self.command2().bits()))
            .field(
                "command2_done",
                &format_args!("{}", self.command2_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13 - command2"]
    #[inline(always)]
    #[must_use]
    pub fn command2(&mut self) -> COMMAND2_W<0> {
        COMMAND2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c commond2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd2](index.html) module"]
pub struct CMD2_SPEC;
impl crate::RegisterSpec for CMD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd2::R](R) reader structure"]
impl crate::Readable for CMD2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd2::W](W) writer structure"]
impl crate::Writable for CMD2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD2 to value 0x0902"]
impl crate::Resettable for CMD2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0902;
}
