#[doc = "Register `CMD1` reader"]
pub struct R(crate::R<CMD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD1` writer"]
pub struct W(crate::W<CMD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD1_SPEC>;
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
impl From<crate::W<CMD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND1` reader - command1"]
pub type COMMAND1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMMAND1` writer - command1"]
pub type COMMAND1_W<'a, const O: u8> = crate::FieldWriter<'a, CMD1_SPEC, 14, O, u16, u16>;
#[doc = "Field `COMMAND1_DONE` reader - command1_done"]
pub type COMMAND1_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - command1"]
    #[inline(always)]
    pub fn command1(&self) -> COMMAND1_R {
        COMMAND1_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command1_done"]
    #[inline(always)]
    pub fn command1_done(&self) -> COMMAND1_DONE_R {
        COMMAND1_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD1")
            .field("command1", &format_args!("{}", self.command1().bits()))
            .field(
                "command1_done",
                &format_args!("{}", self.command1_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13 - command1"]
    #[inline(always)]
    #[must_use]
    pub fn command1(&mut self) -> COMMAND1_W<0> {
        COMMAND1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c commond1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd1](index.html) module"]
pub struct CMD1_SPEC;
impl crate::RegisterSpec for CMD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd1::R](R) reader structure"]
impl crate::Readable for CMD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd1::W](W) writer structure"]
impl crate::Writable for CMD1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD1 to value 0x1901"]
impl crate::Resettable for CMD1_SPEC {
    const RESET_VALUE: Self::Ux = 0x1901;
}
