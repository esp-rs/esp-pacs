#[doc = "Register `FLASH_ACE2_ADDR` reader"]
pub struct R(crate::R<FLASH_ACE2_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_ACE2_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_ACE2_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_ACE2_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_ACE2_ADDR` writer"]
pub struct W(crate::W<FLASH_ACE2_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_ACE2_ADDR_SPEC>;
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
impl From<crate::W<FLASH_ACE2_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_ACE2_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `S` reader - "]
pub type S_R = crate::FieldReader<u32, u32>;
#[doc = "Field `S` writer - "]
pub type S_W<'a, const O: u8> = crate::FieldWriter<'a, FLASH_ACE2_ADDR_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_ACE2_ADDR")
            .field("s", &format_args!("{}", self.s().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FLASH_ACE2_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn s(&mut self) -> S_W<0> {
        S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ace2_addr](index.html) module"]
pub struct FLASH_ACE2_ADDR_SPEC;
impl crate::RegisterSpec for FLASH_ACE2_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_ace2_addr::R](R) reader structure"]
impl crate::Readable for FLASH_ACE2_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_ace2_addr::W](W) writer structure"]
impl crate::Writable for FLASH_ACE2_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_ACE2_ADDR to value 0x2000_0000"]
impl crate::Resettable for FLASH_ACE2_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_0000;
}
