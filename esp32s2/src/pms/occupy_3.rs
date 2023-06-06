#[doc = "Register `OCCUPY_3` reader"]
pub struct R(crate::R<OCCUPY_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCCUPY_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCCUPY_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCCUPY_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCCUPY_3` writer"]
pub struct W(crate::W<OCCUPY_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCCUPY_3_SPEC>;
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
impl From<crate::W<OCCUPY_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCCUPY_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCCUPY_PRO_TRACE` reader - Configure one block of SRAM Block 4-21 is used as trace memory."]
pub type OCCUPY_PRO_TRACE_R = crate::FieldReader<u32>;
#[doc = "Field `OCCUPY_PRO_TRACE` writer - Configure one block of SRAM Block 4-21 is used as trace memory."]
pub type OCCUPY_PRO_TRACE_W<'a, const O: u8> = crate::FieldWriter<'a, OCCUPY_3_SPEC, 18, O, u32>;
impl R {
    #[doc = "Bits 0:17 - Configure one block of SRAM Block 4-21 is used as trace memory."]
    #[inline(always)]
    pub fn occupy_pro_trace(&self) -> OCCUPY_PRO_TRACE_R {
        OCCUPY_PRO_TRACE_R::new(self.bits & 0x0003_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OCCUPY_3")
            .field(
                "occupy_pro_trace",
                &format_args!("{}", self.occupy_pro_trace().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OCCUPY_3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:17 - Configure one block of SRAM Block 4-21 is used as trace memory."]
    #[inline(always)]
    #[must_use]
    pub fn occupy_pro_trace(&mut self) -> OCCUPY_PRO_TRACE_W<0> {
        OCCUPY_PRO_TRACE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Occupy permission control register 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [occupy_3](index.html) module"]
pub struct OCCUPY_3_SPEC;
impl crate::RegisterSpec for OCCUPY_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [occupy_3::R](R) reader structure"]
impl crate::Readable for OCCUPY_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [occupy_3::W](W) writer structure"]
impl crate::Writable for OCCUPY_3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCCUPY_3 to value 0"]
impl crate::Resettable for OCCUPY_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
