#[doc = "Register `T1LOADLO` reader"]
pub type R = crate::R<T1LOADLO_SPEC>;
#[doc = "Register `T1LOADLO` writer"]
pub type W = crate::W<T1LOADLO_SPEC>;
#[doc = "Field `T_LOAD_LO` reader - Configures low 32 bits of the value that a reload will load onto timer 1 time-base counter. \\\\ Measurement unit: T1_clk \\\\"]
pub type T_LOAD_LO_R = crate::FieldReader<u32>;
#[doc = "Field `T_LOAD_LO` writer - Configures low 32 bits of the value that a reload will load onto timer 1 time-base counter. \\\\ Measurement unit: T1_clk \\\\"]
pub type T_LOAD_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures low 32 bits of the value that a reload will load onto timer 1 time-base counter. \\\\ Measurement unit: T1_clk \\\\"]
    #[inline(always)]
    pub fn t_load_lo(&self) -> T_LOAD_LO_R {
        T_LOAD_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T1LOADLO")
            .field("t_load_lo", &self.t_load_lo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures low 32 bits of the value that a reload will load onto timer 1 time-base counter. \\\\ Measurement unit: T1_clk \\\\"]
    #[inline(always)]
    pub fn t_load_lo(&mut self) -> T_LOAD_LO_W<'_, T1LOADLO_SPEC> {
        T_LOAD_LO_W::new(self, 0)
    }
}
#[doc = "Timer 1 reload value, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`t1loadlo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t1loadlo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T1LOADLO_SPEC;
impl crate::RegisterSpec for T1LOADLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t1loadlo::R`](R) reader structure"]
impl crate::Readable for T1LOADLO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t1loadlo::W`](W) writer structure"]
impl crate::Writable for T1LOADLO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets T1LOADLO to value 0"]
impl crate::Resettable for T1LOADLO_SPEC {}
