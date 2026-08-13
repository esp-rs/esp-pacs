#[doc = "Register `CORE0_CPU_INT_CLEAR` reader"]
pub type R = crate::R<CORE0_CPU_INT_CLEAR_SPEC>;
#[doc = "Register `CORE0_CPU_INT_CLEAR` writer"]
pub type W = crate::W<CORE0_CPU_INT_CLEAR_SPEC>;
#[doc = "Field `CORE0_CPU_INT_CLEAR` reader - Need add description"]
pub type CORE0_CPU_INT_CLEAR_R = crate::FieldReader<u32>;
#[doc = "Field `CORE0_CPU_INT_CLEAR` writer - Need add description"]
pub type CORE0_CPU_INT_CLEAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Need add description"]
    #[inline(always)]
    pub fn core0_cpu_int_clear(&self) -> CORE0_CPU_INT_CLEAR_R {
        CORE0_CPU_INT_CLEAR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE0_CPU_INT_CLEAR")
            .field("core0_cpu_int_clear", &self.core0_cpu_int_clear())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Need add description"]
    #[inline(always)]
    pub fn core0_cpu_int_clear(&mut self) -> CORE0_CPU_INT_CLEAR_W<'_, CORE0_CPU_INT_CLEAR_SPEC> {
        CORE0_CPU_INT_CLEAR_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`core0_cpu_int_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core0_cpu_int_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE0_CPU_INT_CLEAR_SPEC;
impl crate::RegisterSpec for CORE0_CPU_INT_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core0_cpu_int_clear::R`](R) reader structure"]
impl crate::Readable for CORE0_CPU_INT_CLEAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core0_cpu_int_clear::W`](W) writer structure"]
impl crate::Writable for CORE0_CPU_INT_CLEAR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE0_CPU_INT_CLEAR to value 0"]
impl crate::Resettable for CORE0_CPU_INT_CLEAR_SPEC {}
