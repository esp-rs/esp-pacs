#[doc = "Register `CORE0_CPU_INT_THRESH` reader"]
pub type R = crate::R<CORE0_CPU_INT_THRESH_SPEC>;
#[doc = "Register `CORE0_CPU_INT_THRESH` writer"]
pub type W = crate::W<CORE0_CPU_INT_THRESH_SPEC>;
#[doc = "Field `CORE0_CPU_INT_THRESH` reader - Need add description"]
pub type CORE0_CPU_INT_THRESH_R = crate::FieldReader;
#[doc = "Field `CORE0_CPU_INT_THRESH` writer - Need add description"]
pub type CORE0_CPU_INT_THRESH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Need add description"]
    #[inline(always)]
    pub fn core0_cpu_int_thresh(&self) -> CORE0_CPU_INT_THRESH_R {
        CORE0_CPU_INT_THRESH_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE0_CPU_INT_THRESH")
            .field("core0_cpu_int_thresh", &self.core0_cpu_int_thresh())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Need add description"]
    #[inline(always)]
    pub fn core0_cpu_int_thresh(
        &mut self,
    ) -> CORE0_CPU_INT_THRESH_W<'_, CORE0_CPU_INT_THRESH_SPEC> {
        CORE0_CPU_INT_THRESH_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`core0_cpu_int_thresh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core0_cpu_int_thresh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE0_CPU_INT_THRESH_SPEC;
impl crate::RegisterSpec for CORE0_CPU_INT_THRESH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core0_cpu_int_thresh::R`](R) reader structure"]
impl crate::Readable for CORE0_CPU_INT_THRESH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core0_cpu_int_thresh::W`](W) writer structure"]
impl crate::Writable for CORE0_CPU_INT_THRESH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE0_CPU_INT_THRESH to value 0"]
impl crate::Resettable for CORE0_CPU_INT_THRESH_SPEC {}
