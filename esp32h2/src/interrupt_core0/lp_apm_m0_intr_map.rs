#[doc = "Register `LP_APM_M0_INTR_MAP` reader"]
pub type R = crate::R<LP_APM_M0_INTR_MAP_SPEC>;
#[doc = "Register `LP_APM_M0_INTR_MAP` writer"]
pub type W = crate::W<LP_APM_M0_INTR_MAP_SPEC>;
#[doc = "Field `LP_APM_M0_INTR_MAP` reader - CORE0_LP_APM_M0_INTR mapping register"]
pub type LP_APM_M0_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `LP_APM_M0_INTR_MAP` writer - CORE0_LP_APM_M0_INTR mapping register"]
pub type LP_APM_M0_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - CORE0_LP_APM_M0_INTR mapping register"]
    #[inline(always)]
    pub fn lp_apm_m0_intr_map(&self) -> LP_APM_M0_INTR_MAP_R {
        LP_APM_M0_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_APM_M0_INTR_MAP")
            .field("lp_apm_m0_intr_map", &self.lp_apm_m0_intr_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - CORE0_LP_APM_M0_INTR mapping register"]
    #[inline(always)]
    pub fn lp_apm_m0_intr_map(&mut self) -> LP_APM_M0_INTR_MAP_W<LP_APM_M0_INTR_MAP_SPEC> {
        LP_APM_M0_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_apm_m0_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_apm_m0_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_APM_M0_INTR_MAP_SPEC;
impl crate::RegisterSpec for LP_APM_M0_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_apm_m0_intr_map::R`](R) reader structure"]
impl crate::Readable for LP_APM_M0_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_apm_m0_intr_map::W`](W) writer structure"]
impl crate::Writable for LP_APM_M0_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_APM_M0_INTR_MAP to value 0"]
impl crate::Resettable for LP_APM_M0_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
