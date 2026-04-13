#[doc = "Register `POWER_PD_HP_CPU_MASK` reader"]
pub type R = crate::R<POWER_PD_HP_CPU_MASK_SPEC>;
#[doc = "Register `POWER_PD_HP_CPU_MASK` writer"]
pub type W = crate::W<POWER_PD_HP_CPU_MASK_SPEC>;
#[doc = "Field `VAL` reader - "]
pub type VAL_R = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - "]
pub type VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_PD_HP_CPU_MASK")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W<'_, POWER_PD_HP_CPU_MASK_SPEC> {
        VAL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_hp_cpu_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_hp_cpu_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_PD_HP_CPU_MASK_SPEC;
impl crate::RegisterSpec for POWER_PD_HP_CPU_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_pd_hp_cpu_mask::R`](R) reader structure"]
impl crate::Readable for POWER_PD_HP_CPU_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power_pd_hp_cpu_mask::W`](W) writer structure"]
impl crate::Writable for POWER_PD_HP_CPU_MASK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER_PD_HP_CPU_MASK to value 0"]
impl crate::Resettable for POWER_PD_HP_CPU_MASK_SPEC {}
