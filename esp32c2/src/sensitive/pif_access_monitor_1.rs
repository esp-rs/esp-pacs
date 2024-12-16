#[doc = "Register `PIF_ACCESS_MONITOR_1` reader"]
pub type R = crate::R<PIF_ACCESS_MONITOR_1_SPEC>;
#[doc = "Register `PIF_ACCESS_MONITOR_1` writer"]
pub type W = crate::W<PIF_ACCESS_MONITOR_1_SPEC>;
#[doc = "Field `PIF_ACCESS_MONITOR_NONWORD_VIOLATE_CLR` reader - Need add description"]
pub type PIF_ACCESS_MONITOR_NONWORD_VIOLATE_CLR_R = crate::BitReader;
#[doc = "Field `PIF_ACCESS_MONITOR_NONWORD_VIOLATE_CLR` writer - Need add description"]
pub type PIF_ACCESS_MONITOR_NONWORD_VIOLATE_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF_ACCESS_MONITOR_NONWORD_VIOLATE_EN` reader - Need add description"]
pub type PIF_ACCESS_MONITOR_NONWORD_VIOLATE_EN_R = crate::BitReader;
#[doc = "Field `PIF_ACCESS_MONITOR_NONWORD_VIOLATE_EN` writer - Need add description"]
pub type PIF_ACCESS_MONITOR_NONWORD_VIOLATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Need add description"]
    #[inline(always)]
    pub fn pif_access_monitor_nonword_violate_clr(
        &self,
    ) -> PIF_ACCESS_MONITOR_NONWORD_VIOLATE_CLR_R {
        PIF_ACCESS_MONITOR_NONWORD_VIOLATE_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Need add description"]
    #[inline(always)]
    pub fn pif_access_monitor_nonword_violate_en(&self) -> PIF_ACCESS_MONITOR_NONWORD_VIOLATE_EN_R {
        PIF_ACCESS_MONITOR_NONWORD_VIOLATE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIF_ACCESS_MONITOR_1")
            .field(
                "pif_access_monitor_nonword_violate_clr",
                &self.pif_access_monitor_nonword_violate_clr(),
            )
            .field(
                "pif_access_monitor_nonword_violate_en",
                &self.pif_access_monitor_nonword_violate_en(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Need add description"]
    #[inline(always)]
    pub fn pif_access_monitor_nonword_violate_clr(
        &mut self,
    ) -> PIF_ACCESS_MONITOR_NONWORD_VIOLATE_CLR_W<PIF_ACCESS_MONITOR_1_SPEC> {
        PIF_ACCESS_MONITOR_NONWORD_VIOLATE_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Need add description"]
    #[inline(always)]
    pub fn pif_access_monitor_nonword_violate_en(
        &mut self,
    ) -> PIF_ACCESS_MONITOR_NONWORD_VIOLATE_EN_W<PIF_ACCESS_MONITOR_1_SPEC> {
        PIF_ACCESS_MONITOR_NONWORD_VIOLATE_EN_W::new(self, 1)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pif_access_monitor_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pif_access_monitor_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIF_ACCESS_MONITOR_1_SPEC;
impl crate::RegisterSpec for PIF_ACCESS_MONITOR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pif_access_monitor_1::R`](R) reader structure"]
impl crate::Readable for PIF_ACCESS_MONITOR_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pif_access_monitor_1::W`](W) writer structure"]
impl crate::Writable for PIF_ACCESS_MONITOR_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIF_ACCESS_MONITOR_1 to value 0x03"]
impl crate::Resettable for PIF_ACCESS_MONITOR_1_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
