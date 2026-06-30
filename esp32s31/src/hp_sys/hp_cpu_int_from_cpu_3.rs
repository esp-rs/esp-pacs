#[doc = "Register `HP_CPU_INT_FROM_CPU_3` reader"]
pub type R = crate::R<HP_CPU_INT_FROM_CPU_3_SPEC>;
#[doc = "Register `HP_CPU_INT_FROM_CPU_3` writer"]
pub type W = crate::W<HP_CPU_INT_FROM_CPU_3_SPEC>;
#[doc = "Field `HP_CPU_INT_FROM_CPU_3` reader - set 1 will triger a interrupt"]
pub type HP_CPU_INT_FROM_CPU_3_R = crate::BitReader;
#[doc = "Field `HP_CPU_INT_FROM_CPU_3` writer - set 1 will triger a interrupt"]
pub type HP_CPU_INT_FROM_CPU_3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - set 1 will triger a interrupt"]
    #[inline(always)]
    pub fn hp_cpu_int_from_cpu_3(&self) -> HP_CPU_INT_FROM_CPU_3_R {
        HP_CPU_INT_FROM_CPU_3_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_CPU_INT_FROM_CPU_3")
            .field("hp_cpu_int_from_cpu_3", &self.hp_cpu_int_from_cpu_3())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - set 1 will triger a interrupt"]
    #[inline(always)]
    pub fn hp_cpu_int_from_cpu_3(
        &mut self,
    ) -> HP_CPU_INT_FROM_CPU_3_W<'_, HP_CPU_INT_FROM_CPU_3_SPEC> {
        HP_CPU_INT_FROM_CPU_3_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_cpu_int_from_cpu_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_cpu_int_from_cpu_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CPU_INT_FROM_CPU_3_SPEC;
impl crate::RegisterSpec for HP_CPU_INT_FROM_CPU_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_cpu_int_from_cpu_3::R`](R) reader structure"]
impl crate::Readable for HP_CPU_INT_FROM_CPU_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_cpu_int_from_cpu_3::W`](W) writer structure"]
impl crate::Writable for HP_CPU_INT_FROM_CPU_3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_CPU_INT_FROM_CPU_3 to value 0"]
impl crate::Resettable for HP_CPU_INT_FROM_CPU_3_SPEC {}
