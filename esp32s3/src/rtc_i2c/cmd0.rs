#[doc = "Register `CMD0` reader"]
pub struct R(crate::R<CMD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD0` writer"]
pub struct W(crate::W<CMD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD0_SPEC>;
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
impl From<crate::W<CMD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND0` reader - command0"]
pub type COMMAND0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMMAND0` writer - command0"]
pub type COMMAND0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD0_SPEC, u16, u16, 14, O>;
#[doc = "Field `COMMAND0_DONE` reader - command0_done"]
pub type COMMAND0_DONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:13 - command0"]
    #[inline(always)]
    pub fn command0(&self) -> COMMAND0_R {
        COMMAND0_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command0_done"]
    #[inline(always)]
    pub fn command0_done(&self) -> COMMAND0_DONE_R {
        COMMAND0_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - command0"]
    #[inline(always)]
    pub fn command0(&mut self) -> COMMAND0_W<0> {
        COMMAND0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c commond0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd0](index.html) module"]
pub struct CMD0_SPEC;
impl crate::RegisterSpec for CMD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd0::R](R) reader structure"]
impl crate::Readable for CMD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd0::W](W) writer structure"]
impl crate::Writable for CMD0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD0 to value 0x0903"]
impl crate::Resettable for CMD0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0903
    }
}
