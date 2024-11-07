#[doc = "Register `CPU_INT_PRI%s` reader"]
pub type R = crate::R<CPU_INT_PRI_SPEC>;
#[doc = "Register `CPU_INT_PRI%s` writer"]
pub type W = crate::W<CPU_INT_PRI_SPEC>;
#[doc = "Field `MAP` reader - reg_core0_cpu_pri_0_map"]
pub type MAP_R = crate::FieldReader;
#[doc = "Field `MAP` writer - reg_core0_cpu_pri_0_map"]
pub type MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - reg_core0_cpu_pri_0_map"]
    #[inline(always)]
    pub fn map(&self) -> MAP_R {
        MAP_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_INT_PRI")
            .field("map", &self.map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - reg_core0_cpu_pri_0_map"]
    #[inline(always)]
    pub fn map(&mut self) -> MAP_W<CPU_INT_PRI_SPEC> {
        MAP_W::new(self, 0)
    }
}
#[doc = "mac intr map register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_int_pri::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_int_pri::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_INT_PRI_SPEC;
impl crate::RegisterSpec for CPU_INT_PRI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_int_pri::R`](R) reader structure"]
impl crate::Readable for CPU_INT_PRI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_int_pri::W`](W) writer structure"]
impl crate::Writable for CPU_INT_PRI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPU_INT_PRI%s to value 0"]
impl crate::Resettable for CPU_INT_PRI_SPEC {
    const RESET_VALUE: u32 = 0;
}
