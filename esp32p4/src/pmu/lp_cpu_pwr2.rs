#[doc = "Register `LP_CPU_PWR2` reader"]
pub type R = crate::R<LP_CPU_PWR2_SPEC>;
#[doc = "Register `LP_CPU_PWR2` writer"]
pub type W = crate::W<LP_CPU_PWR2_SPEC>;
#[doc = "Field `LP_CPU_WAKEUP_EN` reader - need_des"]
pub type LP_CPU_WAKEUP_EN_R = crate::FieldReader<u32>;
#[doc = "Field `LP_CPU_WAKEUP_EN` writer - need_des"]
pub type LP_CPU_WAKEUP_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_wakeup_en(&self) -> LP_CPU_WAKEUP_EN_R {
        LP_CPU_WAKEUP_EN_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_CPU_PWR2")
            .field("lp_cpu_wakeup_en", &self.lp_cpu_wakeup_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_wakeup_en(&mut self) -> LP_CPU_WAKEUP_EN_W<LP_CPU_PWR2_SPEC> {
        LP_CPU_WAKEUP_EN_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_cpu_pwr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_cpu_pwr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_CPU_PWR2_SPEC;
impl crate::RegisterSpec for LP_CPU_PWR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_cpu_pwr2::R`](R) reader structure"]
impl crate::Readable for LP_CPU_PWR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_cpu_pwr2::W`](W) writer structure"]
impl crate::Writable for LP_CPU_PWR2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_CPU_PWR2 to value 0"]
impl crate::Resettable for LP_CPU_PWR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
