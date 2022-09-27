#[doc = "Register `FLASH_ACE2_ATTR` reader"]
pub struct R(crate::R<FLASH_ACE2_ATTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_ACE2_ATTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_ACE2_ATTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_ACE2_ATTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_ACE2_ATTR` writer"]
pub struct W(crate::W<FLASH_ACE2_ATTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_ACE2_ATTR_SPEC>;
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
impl From<crate::W<FLASH_ACE2_ATTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_ACE2_ATTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_ACE2_ATTR` reader - ******* Description ***********"]
pub type FLASH_ACE2_ATTR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FLASH_ACE2_ATTR` writer - ******* Description ***********"]
pub type FLASH_ACE2_ATTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_ACE2_ATTR_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - ******* Description ***********"]
    #[inline(always)]
    pub fn flash_ace2_attr(&self) -> FLASH_ACE2_ATTR_R {
        FLASH_ACE2_ATTR_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - ******* Description ***********"]
    #[inline(always)]
    pub fn flash_ace2_attr(&mut self) -> FLASH_ACE2_ATTR_W<0> {
        FLASH_ACE2_ATTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ace2_attr](index.html) module"]
pub struct FLASH_ACE2_ATTR_SPEC;
impl crate::RegisterSpec for FLASH_ACE2_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_ace2_attr::R](R) reader structure"]
impl crate::Readable for FLASH_ACE2_ATTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_ace2_attr::W](W) writer structure"]
impl crate::Writable for FLASH_ACE2_ATTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_ACE2_ATTR to value 0xff"]
impl crate::Resettable for FLASH_ACE2_ATTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
