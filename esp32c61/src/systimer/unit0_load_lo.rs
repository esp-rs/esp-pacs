#[doc = "Register `UNIT0_LOAD_LO` reader"]
pub type R = crate::R<UNIT0_LOAD_LO_SPEC>;
#[doc = "Register `UNIT0_LOAD_LO` writer"]
pub type W = crate::W<UNIT0_LOAD_LO_SPEC>;
#[doc = "Field `TIMER_UNIT0_LOAD_LO` reader - Configures the value to be loaded to UNIT0, low 32 bits."]
pub type TIMER_UNIT0_LOAD_LO_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER_UNIT0_LOAD_LO` writer - Configures the value to be loaded to UNIT0, low 32 bits."]
pub type TIMER_UNIT0_LOAD_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the value to be loaded to UNIT0, low 32 bits."]
    #[inline(always)]
    pub fn timer_unit0_load_lo(&self) -> TIMER_UNIT0_LOAD_LO_R {
        TIMER_UNIT0_LOAD_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UNIT0_LOAD_LO")
            .field("timer_unit0_load_lo", &self.timer_unit0_load_lo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the value to be loaded to UNIT0, low 32 bits."]
    #[inline(always)]
    pub fn timer_unit0_load_lo(&mut self) -> TIMER_UNIT0_LOAD_LO_W<UNIT0_LOAD_LO_SPEC> {
        TIMER_UNIT0_LOAD_LO_W::new(self, 0)
    }
}
#[doc = "Low 32 bits to be loaded to UNIT0\n\nYou can [`read`](crate::Reg::read) this register and get [`unit0_load_lo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unit0_load_lo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNIT0_LOAD_LO_SPEC;
impl crate::RegisterSpec for UNIT0_LOAD_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`unit0_load_lo::R`](R) reader structure"]
impl crate::Readable for UNIT0_LOAD_LO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`unit0_load_lo::W`](W) writer structure"]
impl crate::Writable for UNIT0_LOAD_LO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UNIT0_LOAD_LO to value 0"]
impl crate::Resettable for UNIT0_LOAD_LO_SPEC {}
