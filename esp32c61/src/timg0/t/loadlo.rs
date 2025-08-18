#[doc = "Register `LOADLO` reader"]
pub type R = crate::R<LOADLO_SPEC>;
#[doc = "Register `LOADLO` writer"]
pub type W = crate::W<LOADLO_SPEC>;
#[doc = "Field `T0_LOAD_LO` reader - Configures low 32 bits of the value that a reload will load onto timer 0 time-base counter. \\\\ Measurement unit: T0_clk \\\\"]
pub type T0_LOAD_LO_R = crate::FieldReader<u32>;
#[doc = "Field `T0_LOAD_LO` writer - Configures low 32 bits of the value that a reload will load onto timer 0 time-base counter. \\\\ Measurement unit: T0_clk \\\\"]
pub type T0_LOAD_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures low 32 bits of the value that a reload will load onto timer 0 time-base counter. \\\\ Measurement unit: T0_clk \\\\"]
    #[inline(always)]
    pub fn t0_load_lo(&self) -> T0_LOAD_LO_R {
        T0_LOAD_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOADLO")
            .field("t0_load_lo", &self.t0_load_lo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures low 32 bits of the value that a reload will load onto timer 0 time-base counter. \\\\ Measurement unit: T0_clk \\\\"]
    #[inline(always)]
    pub fn t0_load_lo(&mut self) -> T0_LOAD_LO_W<'_, LOADLO_SPEC> {
        T0_LOAD_LO_W::new(self, 0)
    }
}
#[doc = "Timer 0 reload value, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`loadlo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loadlo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOADLO_SPEC;
impl crate::RegisterSpec for LOADLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`loadlo::R`](R) reader structure"]
impl crate::Readable for LOADLO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`loadlo::W`](W) writer structure"]
impl crate::Writable for LOADLO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOADLO to value 0"]
impl crate::Resettable for LOADLO_SPEC {}
