#[doc = "Register `TRACE_INTR_MAP` reader"]
pub type R = crate::R<TRACE_INTR_MAP_SPEC>;
#[doc = "Register `TRACE_INTR_MAP` writer"]
pub type W = crate::W<TRACE_INTR_MAP_SPEC>;
#[doc = "Field `TRACE_INTR_MAP` reader - Configures the interrupt source into one CPU interrupt."]
pub type TRACE_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `TRACE_INTR_MAP` writer - Configures the interrupt source into one CPU interrupt."]
pub type TRACE_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TRACE_INTR_PASS_IN_SEC` reader - Configures the PASS_IN_SEC flag of the interrupt source."]
pub type TRACE_INTR_PASS_IN_SEC_R = crate::BitReader;
#[doc = "Field `TRACE_INTR_PASS_IN_SEC` writer - Configures the PASS_IN_SEC flag of the interrupt source."]
pub type TRACE_INTR_PASS_IN_SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn trace_intr_map(&self) -> TRACE_INTR_MAP_R {
        TRACE_INTR_MAP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Configures the PASS_IN_SEC flag of the interrupt source."]
    #[inline(always)]
    pub fn trace_intr_pass_in_sec(&self) -> TRACE_INTR_PASS_IN_SEC_R {
        TRACE_INTR_PASS_IN_SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRACE_INTR_MAP")
            .field("trace_intr_map", &self.trace_intr_map())
            .field("trace_intr_pass_in_sec", &self.trace_intr_pass_in_sec())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn trace_intr_map(&mut self) -> TRACE_INTR_MAP_W<'_, TRACE_INTR_MAP_SPEC> {
        TRACE_INTR_MAP_W::new(self, 0)
    }
    #[doc = "Bit 8 - Configures the PASS_IN_SEC flag of the interrupt source."]
    #[inline(always)]
    pub fn trace_intr_pass_in_sec(&mut self) -> TRACE_INTR_PASS_IN_SEC_W<'_, TRACE_INTR_MAP_SPEC> {
        TRACE_INTR_PASS_IN_SEC_W::new(self, 8)
    }
}
#[doc = "TRACE_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`trace_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trace_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRACE_INTR_MAP_SPEC;
impl crate::RegisterSpec for TRACE_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trace_intr_map::R`](R) reader structure"]
impl crate::Readable for TRACE_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trace_intr_map::W`](W) writer structure"]
impl crate::Writable for TRACE_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRACE_INTR_MAP to value 0"]
impl crate::Resettable for TRACE_INTR_MAP_SPEC {}
