#[doc = "Register `CMD10` reader"]
pub struct R(crate::R<CMD10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD10` writer"]
pub struct W(crate::W<CMD10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD10_SPEC>;
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
impl From<crate::W<CMD10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND10` reader - command10"]
pub type COMMAND10_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMMAND10` writer - command10"]
pub type COMMAND10_W<'a, const O: u8> = crate::FieldWriter<'a, CMD10_SPEC, 14, O, u16, u16>;
#[doc = "Field `COMMAND10_DONE` reader - command10_done"]
pub type COMMAND10_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - command10"]
    #[inline(always)]
    pub fn command10(&self) -> COMMAND10_R {
        COMMAND10_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command10_done"]
    #[inline(always)]
    pub fn command10_done(&self) -> COMMAND10_DONE_R {
        COMMAND10_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD10")
            .field("command10", &format_args!("{}", self.command10().bits()))
            .field(
                "command10_done",
                &format_args!("{}", self.command10_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD10_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13 - command10"]
    #[inline(always)]
    #[must_use]
    pub fn command10(&mut self) -> COMMAND10_W<0> {
        COMMAND10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c commond10 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd10](index.html) module"]
pub struct CMD10_SPEC;
impl crate::RegisterSpec for CMD10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd10::R](R) reader structure"]
impl crate::Readable for CMD10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd10::W](W) writer structure"]
impl crate::Writable for CMD10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD10 to value 0x0101"]
impl crate::Resettable for CMD10_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101;
}
