#[doc = "Register `UNIT0_LOAD_HI` reader"]
pub type R = crate::R<UNIT0_LOAD_HI_SPEC>;
#[doc = "Register `UNIT0_LOAD_HI` writer"]
pub type W = crate::W<UNIT0_LOAD_HI_SPEC>;
#[doc = "Field `TIMER_UNIT0_LOAD_HI` reader - Configures the value to be loaded to UNIT0, high 20 bits."]
pub type TIMER_UNIT0_LOAD_HI_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER_UNIT0_LOAD_HI` writer - Configures the value to be loaded to UNIT0, high 20 bits."]
pub type TIMER_UNIT0_LOAD_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Configures the value to be loaded to UNIT0, high 20 bits."]
    #[inline(always)]
    pub fn timer_unit0_load_hi(&self) -> TIMER_UNIT0_LOAD_HI_R {
        TIMER_UNIT0_LOAD_HI_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UNIT0_LOAD_HI")
            .field("timer_unit0_load_hi", &self.timer_unit0_load_hi())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - Configures the value to be loaded to UNIT0, high 20 bits."]
    #[inline(always)]
    pub fn timer_unit0_load_hi(&mut self) -> TIMER_UNIT0_LOAD_HI_W<UNIT0_LOAD_HI_SPEC> {
        TIMER_UNIT0_LOAD_HI_W::new(self, 0)
    }
}
#[doc = "High 20 bits to be loaded to UNIT0\n\nYou can [`read`](crate::Reg::read) this register and get [`unit0_load_hi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unit0_load_hi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNIT0_LOAD_HI_SPEC;
impl crate::RegisterSpec for UNIT0_LOAD_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`unit0_load_hi::R`](R) reader structure"]
impl crate::Readable for UNIT0_LOAD_HI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`unit0_load_hi::W`](W) writer structure"]
impl crate::Writable for UNIT0_LOAD_HI_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UNIT0_LOAD_HI to value 0"]
impl crate::Resettable for UNIT0_LOAD_HI_SPEC {}
