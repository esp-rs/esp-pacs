#[doc = "Register `INTERNAL_SRAM_USAGE_2` reader"]
pub type R = crate::R<INTERNAL_SRAM_USAGE_2_SPEC>;
#[doc = "Register `INTERNAL_SRAM_USAGE_2` writer"]
pub type W = crate::W<INTERNAL_SRAM_USAGE_2_SPEC>;
#[doc = "Field `INTERNAL_SRAM_CORE0_TRACE_USAGE` reader - Set 1 to someone bit means corresponding internal SRAM level can be accessed by core0 trace bus."]
pub type INTERNAL_SRAM_CORE0_TRACE_USAGE_R = crate::FieldReader;
#[doc = "Field `INTERNAL_SRAM_CORE0_TRACE_USAGE` writer - Set 1 to someone bit means corresponding internal SRAM level can be accessed by core0 trace bus."]
pub type INTERNAL_SRAM_CORE0_TRACE_USAGE_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `INTERNAL_SRAM_CORE1_TRACE_USAGE` reader - Set 1 to someone bit means corresponding internal SRAM level can be accessed by core1 trace bus."]
pub type INTERNAL_SRAM_CORE1_TRACE_USAGE_R = crate::FieldReader;
#[doc = "Field `INTERNAL_SRAM_CORE1_TRACE_USAGE` writer - Set 1 to someone bit means corresponding internal SRAM level can be accessed by core1 trace bus."]
pub type INTERNAL_SRAM_CORE1_TRACE_USAGE_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `INTERNAL_SRAM_CORE0_TRACE_ALLOC` reader - Which internal SRAM bank (16KB) of 64KB can be accessed by core0 trace bus."]
pub type INTERNAL_SRAM_CORE0_TRACE_ALLOC_R = crate::FieldReader;
#[doc = "Field `INTERNAL_SRAM_CORE0_TRACE_ALLOC` writer - Which internal SRAM bank (16KB) of 64KB can be accessed by core0 trace bus."]
pub type INTERNAL_SRAM_CORE0_TRACE_ALLOC_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `INTERNAL_SRAM_CORE1_TRACE_ALLOC` reader - Which internal SRAM bank (16KB) of 64KB can be accessed by core1 trace bus."]
pub type INTERNAL_SRAM_CORE1_TRACE_ALLOC_R = crate::FieldReader;
#[doc = "Field `INTERNAL_SRAM_CORE1_TRACE_ALLOC` writer - Which internal SRAM bank (16KB) of 64KB can be accessed by core1 trace bus."]
pub type INTERNAL_SRAM_CORE1_TRACE_ALLOC_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:6 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by core0 trace bus."]
    #[inline(always)]
    pub fn internal_sram_core0_trace_usage(&self) -> INTERNAL_SRAM_CORE0_TRACE_USAGE_R {
        INTERNAL_SRAM_CORE0_TRACE_USAGE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by core1 trace bus."]
    #[inline(always)]
    pub fn internal_sram_core1_trace_usage(&self) -> INTERNAL_SRAM_CORE1_TRACE_USAGE_R {
        INTERNAL_SRAM_CORE1_TRACE_USAGE_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:15 - Which internal SRAM bank (16KB) of 64KB can be accessed by core0 trace bus."]
    #[inline(always)]
    pub fn internal_sram_core0_trace_alloc(&self) -> INTERNAL_SRAM_CORE0_TRACE_ALLOC_R {
        INTERNAL_SRAM_CORE0_TRACE_ALLOC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Which internal SRAM bank (16KB) of 64KB can be accessed by core1 trace bus."]
    #[inline(always)]
    pub fn internal_sram_core1_trace_alloc(&self) -> INTERNAL_SRAM_CORE1_TRACE_ALLOC_R {
        INTERNAL_SRAM_CORE1_TRACE_ALLOC_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERNAL_SRAM_USAGE_2")
            .field(
                "internal_sram_core0_trace_usage",
                &format_args!("{}", self.internal_sram_core0_trace_usage().bits()),
            )
            .field(
                "internal_sram_core1_trace_usage",
                &format_args!("{}", self.internal_sram_core1_trace_usage().bits()),
            )
            .field(
                "internal_sram_core0_trace_alloc",
                &format_args!("{}", self.internal_sram_core0_trace_alloc().bits()),
            )
            .field(
                "internal_sram_core1_trace_alloc",
                &format_args!("{}", self.internal_sram_core1_trace_alloc().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTERNAL_SRAM_USAGE_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by core0 trace bus."]
    #[inline(always)]
    #[must_use]
    pub fn internal_sram_core0_trace_usage(
        &mut self,
    ) -> INTERNAL_SRAM_CORE0_TRACE_USAGE_W<INTERNAL_SRAM_USAGE_2_SPEC, 0> {
        INTERNAL_SRAM_CORE0_TRACE_USAGE_W::new(self)
    }
    #[doc = "Bits 7:13 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by core1 trace bus."]
    #[inline(always)]
    #[must_use]
    pub fn internal_sram_core1_trace_usage(
        &mut self,
    ) -> INTERNAL_SRAM_CORE1_TRACE_USAGE_W<INTERNAL_SRAM_USAGE_2_SPEC, 7> {
        INTERNAL_SRAM_CORE1_TRACE_USAGE_W::new(self)
    }
    #[doc = "Bits 14:15 - Which internal SRAM bank (16KB) of 64KB can be accessed by core0 trace bus."]
    #[inline(always)]
    #[must_use]
    pub fn internal_sram_core0_trace_alloc(
        &mut self,
    ) -> INTERNAL_SRAM_CORE0_TRACE_ALLOC_W<INTERNAL_SRAM_USAGE_2_SPEC, 14> {
        INTERNAL_SRAM_CORE0_TRACE_ALLOC_W::new(self)
    }
    #[doc = "Bits 16:17 - Which internal SRAM bank (16KB) of 64KB can be accessed by core1 trace bus."]
    #[inline(always)]
    #[must_use]
    pub fn internal_sram_core1_trace_alloc(
        &mut self,
    ) -> INTERNAL_SRAM_CORE1_TRACE_ALLOC_W<INTERNAL_SRAM_USAGE_2_SPEC, 16> {
        INTERNAL_SRAM_CORE1_TRACE_ALLOC_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Internal SRAM configuration register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`internal_sram_usage_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`internal_sram_usage_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERNAL_SRAM_USAGE_2_SPEC;
impl crate::RegisterSpec for INTERNAL_SRAM_USAGE_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`internal_sram_usage_2::R`](R) reader structure"]
impl crate::Readable for INTERNAL_SRAM_USAGE_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`internal_sram_usage_2::W`](W) writer structure"]
impl crate::Writable for INTERNAL_SRAM_USAGE_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTERNAL_SRAM_USAGE_2 to value 0"]
impl crate::Resettable for INTERNAL_SRAM_USAGE_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
