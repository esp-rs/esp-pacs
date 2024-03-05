#[doc = "Register `CPU_INT_PRI_4` reader"]
pub type R = crate::R<CPU_INT_PRI_4_SPEC>;
#[doc = "Register `CPU_INT_PRI_4` writer"]
pub type W = crate::W<CPU_INT_PRI_4_SPEC>;
#[doc = "Field `CPU_PRI_4_MAP` reader - reg_core0_cpu_pri_4_map"]
pub type CPU_PRI_4_MAP_R = crate::FieldReader;
#[doc = "Field `CPU_PRI_4_MAP` writer - reg_core0_cpu_pri_4_map"]
pub type CPU_PRI_4_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - reg_core0_cpu_pri_4_map"]
    #[inline(always)]
    pub fn cpu_pri_4_map(&self) -> CPU_PRI_4_MAP_R {
        CPU_PRI_4_MAP_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_INT_PRI_4")
            .field(
                "cpu_pri_4_map",
                &format_args!("{}", self.cpu_pri_4_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPU_INT_PRI_4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - reg_core0_cpu_pri_4_map"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_pri_4_map(&mut self) -> CPU_PRI_4_MAP_W<CPU_INT_PRI_4_SPEC> {
        CPU_PRI_4_MAP_W::new(self, 0)
    }
}
#[doc = "mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_INT_PRI_4_SPEC;
impl crate::RegisterSpec for CPU_INT_PRI_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_int_pri_4::R`](R) reader structure"]
impl crate::Readable for CPU_INT_PRI_4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_int_pri_4::W`](W) writer structure"]
impl crate::Writable for CPU_INT_PRI_4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPU_INT_PRI_4 to value 0"]
impl crate::Resettable for CPU_INT_PRI_4_SPEC {
    const RESET_VALUE: u32 = 0;
}
