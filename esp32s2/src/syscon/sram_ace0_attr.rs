#[doc = "Register `SRAM_ACE0_ATTR` reader"]
pub struct R(crate::R<SRAM_ACE0_ATTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_ACE0_ATTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_ACE0_ATTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_ACE0_ATTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_ACE0_ATTR` writer"]
pub struct W(crate::W<SRAM_ACE0_ATTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_ACE0_ATTR_SPEC>;
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
impl From<crate::W<SRAM_ACE0_ATTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_ACE0_ATTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM_ACE0_ATTR` reader - "]
pub type SRAM_ACE0_ATTR_R = crate::FieldReader;
#[doc = "Field `SRAM_ACE0_ATTR` writer - "]
pub type SRAM_ACE0_ATTR_W<'a, const O: u8> = crate::FieldWriter<'a, SRAM_ACE0_ATTR_SPEC, 3, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn sram_ace0_attr(&self) -> SRAM_ACE0_ATTR_R {
        SRAM_ACE0_ATTR_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_ACE0_ATTR")
            .field(
                "sram_ace0_attr",
                &format_args!("{}", self.sram_ace0_attr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SRAM_ACE0_ATTR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn sram_ace0_attr(&mut self) -> SRAM_ACE0_ATTR_W<0> {
        SRAM_ACE0_ATTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_ace0_attr](index.html) module"]
pub struct SRAM_ACE0_ATTR_SPEC;
impl crate::RegisterSpec for SRAM_ACE0_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_ace0_attr::R](R) reader structure"]
impl crate::Readable for SRAM_ACE0_ATTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_ace0_attr::W](W) writer structure"]
impl crate::Writable for SRAM_ACE0_ATTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAM_ACE0_ATTR to value 0x07"]
impl crate::Resettable for SRAM_ACE0_ATTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
