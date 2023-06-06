#[doc = "Register `CMD3` reader"]
pub struct R(crate::R<CMD3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD3` writer"]
pub struct W(crate::W<CMD3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD3_SPEC>;
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
impl From<crate::W<CMD3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND3` reader - command3"]
pub type COMMAND3_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND3` writer - command3"]
pub type COMMAND3_W<'a, const O: u8> = crate::FieldWriter<'a, CMD3_SPEC, 14, O, u16>;
#[doc = "Field `COMMAND3_DONE` reader - command3_done"]
pub type COMMAND3_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - command3"]
    #[inline(always)]
    pub fn command3(&self) -> COMMAND3_R {
        COMMAND3_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command3_done"]
    #[inline(always)]
    pub fn command3_done(&self) -> COMMAND3_DONE_R {
        COMMAND3_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD3")
            .field("command3", &format_args!("{}", self.command3().bits()))
            .field(
                "command3_done",
                &format_args!("{}", self.command3_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13 - command3"]
    #[inline(always)]
    #[must_use]
    pub fn command3(&mut self) -> COMMAND3_W<0> {
        COMMAND3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c commond3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd3](index.html) module"]
pub struct CMD3_SPEC;
impl crate::RegisterSpec for CMD3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd3::R](R) reader structure"]
impl crate::Readable for CMD3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd3::W](W) writer structure"]
impl crate::Writable for CMD3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD3 to value 0x0101"]
impl crate::Resettable for CMD3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101;
}
