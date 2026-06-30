#[doc = "Register `T0LOADLO` reader"]
pub type R = crate::R<T0LOADLO_SPEC>;
#[doc = "Register `T0LOADLO` writer"]
pub type W = crate::W<T0LOADLO_SPEC>;
#[doc = "Field `T_LOAD_LO` reader - Configures low 32 bits of the value that a reload will load onto timer 0 time-base counter. \\\\ Measurement unit: T0_clk \\\\"]
pub type T_LOAD_LO_R = crate::FieldReader<u32>;
#[doc = "Field `T_LOAD_LO` writer - Configures low 32 bits of the value that a reload will load onto timer 0 time-base counter. \\\\ Measurement unit: T0_clk \\\\"]
pub type T_LOAD_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures low 32 bits of the value that a reload will load onto timer 0 time-base counter. \\\\ Measurement unit: T0_clk \\\\"]
    #[inline(always)]
    pub fn t_load_lo(&self) -> T_LOAD_LO_R {
        T_LOAD_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0LOADLO")
            .field("t_load_lo", &self.t_load_lo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures low 32 bits of the value that a reload will load onto timer 0 time-base counter. \\\\ Measurement unit: T0_clk \\\\"]
    #[inline(always)]
    pub fn t_load_lo(&mut self) -> T_LOAD_LO_W<'_, T0LOADLO_SPEC> {
        T_LOAD_LO_W::new(self, 0)
    }
}
#[doc = "Timer 0 reload value, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`t0loadlo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t0loadlo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0LOADLO_SPEC;
impl crate::RegisterSpec for T0LOADLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0loadlo::R`](R) reader structure"]
impl crate::Readable for T0LOADLO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t0loadlo::W`](W) writer structure"]
impl crate::Writable for T0LOADLO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets T0LOADLO to value 0"]
impl crate::Resettable for T0LOADLO_SPEC {}
