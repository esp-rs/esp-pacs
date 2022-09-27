#[doc = "Register `CMD7` reader"]
pub struct R(crate::R<CMD7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD7` writer"]
pub struct W(crate::W<CMD7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD7_SPEC>;
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
impl From<crate::W<CMD7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND7` reader - command7"]
pub type COMMAND7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMMAND7` writer - command7"]
pub type COMMAND7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD7_SPEC, u16, u16, 14, O>;
#[doc = "Field `COMMAND7_DONE` reader - command7_done"]
pub type COMMAND7_DONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:13 - command7"]
    #[inline(always)]
    pub fn command7(&self) -> COMMAND7_R {
        COMMAND7_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command7_done"]
    #[inline(always)]
    pub fn command7_done(&self) -> COMMAND7_DONE_R {
        COMMAND7_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - command7"]
    #[inline(always)]
    pub fn command7(&mut self) -> COMMAND7_W<0> {
        COMMAND7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c commond7 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd7](index.html) module"]
pub struct CMD7_SPEC;
impl crate::RegisterSpec for CMD7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd7::R](R) reader structure"]
impl crate::Readable for CMD7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd7::W](W) writer structure"]
impl crate::Writable for CMD7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD7 to value 0x0904"]
impl crate::Resettable for CMD7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0904
    }
}
