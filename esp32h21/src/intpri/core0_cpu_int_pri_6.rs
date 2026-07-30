#[doc = "Register `CORE0_CPU_INT_PRI_6` reader"]
pub type R = crate::R<CORE0_CPU_INT_PRI_6_SPEC>;
#[doc = "Register `CORE0_CPU_INT_PRI_6` writer"]
pub type W = crate::W<CORE0_CPU_INT_PRI_6_SPEC>;
#[doc = "Field `CORE0_CPU_PRI_6_MAP` reader - Need add description"]
pub type CORE0_CPU_PRI_6_MAP_R = crate::FieldReader;
#[doc = "Field `CORE0_CPU_PRI_6_MAP` writer - Need add description"]
pub type CORE0_CPU_PRI_6_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Need add description"]
    #[inline(always)]
    pub fn core0_cpu_pri_6_map(&self) -> CORE0_CPU_PRI_6_MAP_R {
        CORE0_CPU_PRI_6_MAP_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE0_CPU_INT_PRI_6")
            .field("core0_cpu_pri_6_map", &self.core0_cpu_pri_6_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Need add description"]
    #[inline(always)]
    pub fn core0_cpu_pri_6_map(&mut self) -> CORE0_CPU_PRI_6_MAP_W<'_, CORE0_CPU_INT_PRI_6_SPEC> {
        CORE0_CPU_PRI_6_MAP_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`core0_cpu_int_pri_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core0_cpu_int_pri_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE0_CPU_INT_PRI_6_SPEC;
impl crate::RegisterSpec for CORE0_CPU_INT_PRI_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core0_cpu_int_pri_6::R`](R) reader structure"]
impl crate::Readable for CORE0_CPU_INT_PRI_6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core0_cpu_int_pri_6::W`](W) writer structure"]
impl crate::Writable for CORE0_CPU_INT_PRI_6_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE0_CPU_INT_PRI_6 to value 0"]
impl crate::Resettable for CORE0_CPU_INT_PRI_6_SPEC {}
