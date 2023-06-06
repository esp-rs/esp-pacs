#[doc = "Register `REGION7_ADDR_END` reader"]
pub struct R(crate::R<REGION7_ADDR_END_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGION7_ADDR_END_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGION7_ADDR_END_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGION7_ADDR_END_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGION7_ADDR_END` writer"]
pub struct W(crate::W<REGION7_ADDR_END_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGION7_ADDR_END_SPEC>;
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
impl From<crate::W<REGION7_ADDR_END_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGION7_ADDR_END_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGION7_ADDR_END` reader - End address of region7"]
pub type REGION7_ADDR_END_R = crate::FieldReader<u32>;
#[doc = "Field `REGION7_ADDR_END` writer - End address of region7"]
pub type REGION7_ADDR_END_W<'a, const O: u8> =
    crate::FieldWriter<'a, REGION7_ADDR_END_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - End address of region7"]
    #[inline(always)]
    pub fn region7_addr_end(&self) -> REGION7_ADDR_END_R {
        REGION7_ADDR_END_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION7_ADDR_END")
            .field(
                "region7_addr_end",
                &format_args!("{}", self.region7_addr_end().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGION7_ADDR_END_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - End address of region7"]
    #[inline(always)]
    #[must_use]
    pub fn region7_addr_end(&mut self) -> REGION7_ADDR_END_W<0> {
        REGION7_ADDR_END_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Region address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region7_addr_end](index.html) module"]
pub struct REGION7_ADDR_END_SPEC;
impl crate::RegisterSpec for REGION7_ADDR_END_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [region7_addr_end::R](R) reader structure"]
impl crate::Readable for REGION7_ADDR_END_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [region7_addr_end::W](W) writer structure"]
impl crate::Writable for REGION7_ADDR_END_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGION7_ADDR_END to value 0xffff_ffff"]
impl crate::Resettable for REGION7_ADDR_END_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
