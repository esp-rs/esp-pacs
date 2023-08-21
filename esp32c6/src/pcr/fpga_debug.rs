#[doc = "Register `FPGA_DEBUG` reader"]
pub type R = crate::R<FPGA_DEBUG_SPEC>;
#[doc = "Register `FPGA_DEBUG` writer"]
pub type W = crate::W<FPGA_DEBUG_SPEC>;
#[doc = "Field `FPGA_DEBUG` reader - Only used in fpga debug."]
pub type FPGA_DEBUG_R = crate::FieldReader<u32>;
#[doc = "Field `FPGA_DEBUG` writer - Only used in fpga debug."]
pub type FPGA_DEBUG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
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
    pub fn fpga_debug(&mut self) -> FPGA_DEBUG_W<FPGA_DEBUG_SPEC, 0> {
        FPGA_DEBUG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "fpga debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpga_debug::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpga_debug::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPGA_DEBUG_SPEC;
impl crate::RegisterSpec for FPGA_DEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpga_debug::R`](R) reader structure"]
impl crate::Readable for FPGA_DEBUG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fpga_debug::W`](W) writer structure"]
impl crate::Writable for FPGA_DEBUG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPGA_DEBUG to value 0xffff_ffff"]
impl crate::Resettable for FPGA_DEBUG_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
