#[doc = "Register `FPGA_DEBUG` reader"]
pub struct R(crate::R<FPGA_DEBUG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPGA_DEBUG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPGA_DEBUG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPGA_DEBUG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPGA_DEBUG` writer"]
pub struct W(crate::W<FPGA_DEBUG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPGA_DEBUG_SPEC>;
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
impl From<crate::W<FPGA_DEBUG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPGA_DEBUG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPGA_DEBUG` reader - Only used in fpga debug."]
pub type FPGA_DEBUG_R = crate::FieldReader<u32>;
#[doc = "Field `FPGA_DEBUG` writer - Only used in fpga debug."]
pub type FPGA_DEBUG_W<'a, const O: u8> = crate::FieldWriter<'a, FPGA_DEBUG_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Only used in fpga debug."]
    #[inline(always)]
    pub fn fpga_debug(&self) -> FPGA_DEBUG_R {
        FPGA_DEBUG_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPGA_DEBUG")
            .field("fpga_debug", &format_args!("{}", self.fpga_debug().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FPGA_DEBUG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Only used in fpga debug."]
    #[inline(always)]
    #[must_use]
    pub fn fpga_debug(&mut self) -> FPGA_DEBUG_W<0> {
        FPGA_DEBUG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "fpga debug register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpga_debug](index.html) module"]
pub struct FPGA_DEBUG_SPEC;
impl crate::RegisterSpec for FPGA_DEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpga_debug::R](R) reader structure"]
impl crate::Readable for FPGA_DEBUG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpga_debug::W](W) writer structure"]
impl crate::Writable for FPGA_DEBUG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPGA_DEBUG to value 0xffff_ffff"]
impl crate::Resettable for FPGA_DEBUG_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
