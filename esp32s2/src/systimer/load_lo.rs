#[doc = "Register `LOAD_LO` reader"]
pub type R = crate::R<LOAD_LO_SPEC>;
#[doc = "Register `LOAD_LO` writer"]
pub type W = crate::W<LOAD_LO_SPEC>;
#[doc = "Field `TIMER_LOAD_LO` reader - The value to be loaded into system timer, low 32 bits."]
pub type TIMER_LOAD_LO_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER_LOAD_LO` writer - The value to be loaded into system timer, low 32 bits."]
pub type TIMER_LOAD_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The value to be loaded into system timer, low 32 bits."]
    #[inline(always)]
    pub fn timer_load_lo(&self) -> TIMER_LOAD_LO_R {
        TIMER_LOAD_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOAD_LO")
            .field(
                "timer_load_lo",
                &format_args!("{}", self.timer_load_lo().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOAD_LO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The value to be loaded into system timer, low 32 bits."]
    #[inline(always)]
    #[must_use]
    pub fn timer_load_lo(&mut self) -> TIMER_LOAD_LO_W<LOAD_LO_SPEC> {
        TIMER_LOAD_LO_W::new(self, 0)
    }
}
#[doc = "Low 32 bits to be loaded to system timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`load_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOAD_LO_SPEC;
impl crate::RegisterSpec for LOAD_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`load_lo::R`](R) reader structure"]
impl crate::Readable for LOAD_LO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`load_lo::W`](W) writer structure"]
impl crate::Writable for LOAD_LO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOAD_LO to value 0"]
impl crate::Resettable for LOAD_LO_SPEC {
    const RESET_VALUE: u32 = 0;
}
