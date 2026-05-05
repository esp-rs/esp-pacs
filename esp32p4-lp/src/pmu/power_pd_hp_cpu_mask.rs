#[doc = "Register `POWER_PD_HP_CPU_MASK` reader"]
pub type R = crate::R<POWER_PD_HP_CPU_MASK_SPEC>;
#[doc = "Register `POWER_PD_HP_CPU_MASK` writer"]
pub type W = crate::W<POWER_PD_HP_CPU_MASK_SPEC>;
#[doc = "Field `XPD_HP_CPU_MASK` reader - PMU_XPD_HP_CPU_MASK"]
pub type XPD_HP_CPU_MASK_R = crate::FieldReader;
#[doc = "Field `XPD_HP_CPU_MASK` writer - PMU_XPD_HP_CPU_MASK"]
pub type XPD_HP_CPU_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PD_HP_CPU_MASK` reader - PMU_PD_HP_CPU_MASK"]
pub type PD_HP_CPU_MASK_R = crate::FieldReader;
#[doc = "Field `PD_HP_CPU_MASK` writer - PMU_PD_HP_CPU_MASK"]
pub type PD_HP_CPU_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - PMU_XPD_HP_CPU_MASK"]
    #[inline(always)]
    pub fn xpd_hp_cpu_mask(&self) -> XPD_HP_CPU_MASK_R {
        XPD_HP_CPU_MASK_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - PMU_PD_HP_CPU_MASK"]
    #[inline(always)]
    pub fn pd_hp_cpu_mask(&self) -> PD_HP_CPU_MASK_R {
        PD_HP_CPU_MASK_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_PD_HP_CPU_MASK")
            .field("xpd_hp_cpu_mask", &self.xpd_hp_cpu_mask())
            .field("pd_hp_cpu_mask", &self.pd_hp_cpu_mask())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - PMU_XPD_HP_CPU_MASK"]
    #[inline(always)]
    pub fn xpd_hp_cpu_mask(&mut self) -> XPD_HP_CPU_MASK_W<'_, POWER_PD_HP_CPU_MASK_SPEC> {
        XPD_HP_CPU_MASK_W::new(self, 0)
    }
    #[doc = "Bits 27:31 - PMU_PD_HP_CPU_MASK"]
    #[inline(always)]
    pub fn pd_hp_cpu_mask(&mut self) -> PD_HP_CPU_MASK_W<'_, POWER_PD_HP_CPU_MASK_SPEC> {
        PD_HP_CPU_MASK_W::new(self, 27)
    }
}
#[doc = "PMU_POWER_PD_HP_CPU_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_hp_cpu_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_hp_cpu_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
