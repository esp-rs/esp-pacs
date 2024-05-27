#[doc = "Register `PWC` reader"]
pub type R = crate::R<PWC_SPEC>;
#[doc = "Register `PWC` writer"]
pub type W = crate::W<PWC_SPEC>;
#[doc = "Field `FASTMEM_FORCE_NOISO` reader - Fast RTC memory force no ISO"]
pub type FASTMEM_FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `FASTMEM_FORCE_NOISO` writer - Fast RTC memory force no ISO"]
pub type FASTMEM_FORCE_NOISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FASTMEM_FORCE_ISO` reader - Fast RTC memory force ISO"]
pub type FASTMEM_FORCE_ISO_R = crate::BitReader;
#[doc = "Field `FASTMEM_FORCE_ISO` writer - Fast RTC memory force ISO"]
pub type FASTMEM_FORCE_ISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOWMEM_FORCE_NOISO` reader - RTC memory force no ISO"]
pub type SLOWMEM_FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `SLOWMEM_FORCE_NOISO` writer - RTC memory force no ISO"]
pub type SLOWMEM_FORCE_NOISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOWMEM_FORCE_ISO` reader - RTC memory force ISO"]
pub type SLOWMEM_FORCE_ISO_R = crate::BitReader;
#[doc = "Field `SLOWMEM_FORCE_ISO` writer - RTC memory force ISO"]
pub type SLOWMEM_FORCE_ISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ISO` reader - rtc_peri force ISO"]
pub type FORCE_ISO_R = crate::BitReader;
#[doc = "Field `FORCE_ISO` writer - rtc_peri force ISO"]
pub type FORCE_ISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_NOISO` reader - rtc_peri force no ISO"]
pub type FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `FORCE_NOISO` writer - rtc_peri force no ISO"]
pub type FORCE_NOISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FASTMEM_FOLW_CPU` reader - 1: Fast RTC memory PD following CPU 0: fast RTC memory PD following RTC state machine"]
pub type FASTMEM_FOLW_CPU_R = crate::BitReader;
#[doc = "Field `FASTMEM_FOLW_CPU` writer - 1: Fast RTC memory PD following CPU 0: fast RTC memory PD following RTC state machine"]
pub type FASTMEM_FOLW_CPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FASTMEM_FORCE_LPD` reader - Fast RTC memory force PD"]
pub type FASTMEM_FORCE_LPD_R = crate::BitReader;
#[doc = "Field `FASTMEM_FORCE_LPD` writer - Fast RTC memory force PD"]
pub type FASTMEM_FORCE_LPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FASTMEM_FORCE_LPU` reader - Fast RTC memory force no PD"]
pub type FASTMEM_FORCE_LPU_R = crate::BitReader;
#[doc = "Field `FASTMEM_FORCE_LPU` writer - Fast RTC memory force no PD"]
pub type FASTMEM_FORCE_LPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOWMEM_FOLW_CPU` reader - 1: RTC memory PD following CPU 0: RTC memory PD following RTC state machine"]
pub type SLOWMEM_FOLW_CPU_R = crate::BitReader;
#[doc = "Field `SLOWMEM_FOLW_CPU` writer - 1: RTC memory PD following CPU 0: RTC memory PD following RTC state machine"]
pub type SLOWMEM_FOLW_CPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOWMEM_FORCE_LPD` reader - RTC memory force PD"]
pub type SLOWMEM_FORCE_LPD_R = crate::BitReader;
#[doc = "Field `SLOWMEM_FORCE_LPD` writer - RTC memory force PD"]
pub type SLOWMEM_FORCE_LPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOWMEM_FORCE_LPU` reader - RTC memory force no PD"]
pub type SLOWMEM_FORCE_LPU_R = crate::BitReader;
#[doc = "Field `SLOWMEM_FORCE_LPU` writer - RTC memory force no PD"]
pub type SLOWMEM_FORCE_LPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FASTMEM_FORCE_PD` reader - Fast RTC memory force power down"]
pub type FASTMEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `FASTMEM_FORCE_PD` writer - Fast RTC memory force power down"]
pub type FASTMEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FASTMEM_FORCE_PU` reader - Fast RTC memory force power up"]
pub type FASTMEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `FASTMEM_FORCE_PU` writer - Fast RTC memory force power up"]
pub type FASTMEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FASTMEM_PD_EN` reader - enable power down fast RTC memory in sleep"]
pub type FASTMEM_PD_EN_R = crate::BitReader;
#[doc = "Field `FASTMEM_PD_EN` writer - enable power down fast RTC memory in sleep"]
pub type FASTMEM_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOWMEM_FORCE_PD` reader - RTC memory force power down"]
pub type SLOWMEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `SLOWMEM_FORCE_PD` writer - RTC memory force power down"]
pub type SLOWMEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOWMEM_FORCE_PU` reader - RTC memory force power up"]
pub type SLOWMEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `SLOWMEM_FORCE_PU` writer - RTC memory force power up"]
pub type SLOWMEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOWMEM_PD_EN` reader - enable power down RTC memory in sleep"]
pub type SLOWMEM_PD_EN_R = crate::BitReader;
#[doc = "Field `SLOWMEM_PD_EN` writer - enable power down RTC memory in sleep"]
pub type SLOWMEM_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_PD` reader - rtc_peri force power down"]
pub type FORCE_PD_R = crate::BitReader;
#[doc = "Field `FORCE_PD` writer - rtc_peri force power down"]
pub type FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_PU` reader - rtc_peri force power up"]
pub type FORCE_PU_R = crate::BitReader;
#[doc = "Field `FORCE_PU` writer - rtc_peri force power up"]
pub type FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD_EN` reader - enable power down rtc_peri in sleep"]
pub type PD_EN_R = crate::BitReader;
#[doc = "Field `PD_EN` writer - enable power down rtc_peri in sleep"]
pub type PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fast RTC memory force no ISO"]
    #[inline(always)]
    pub fn fastmem_force_noiso(&self) -> FASTMEM_FORCE_NOISO_R {
        FASTMEM_FORCE_NOISO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fast RTC memory force ISO"]
    #[inline(always)]
    pub fn fastmem_force_iso(&self) -> FASTMEM_FORCE_ISO_R {
        FASTMEM_FORCE_ISO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC memory force no ISO"]
    #[inline(always)]
    pub fn slowmem_force_noiso(&self) -> SLOWMEM_FORCE_NOISO_R {
        SLOWMEM_FORCE_NOISO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC memory force ISO"]
    #[inline(always)]
    pub fn slowmem_force_iso(&self) -> SLOWMEM_FORCE_ISO_R {
        SLOWMEM_FORCE_ISO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - rtc_peri force ISO"]
    #[inline(always)]
    pub fn force_iso(&self) -> FORCE_ISO_R {
        FORCE_ISO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - rtc_peri force no ISO"]
    #[inline(always)]
    pub fn force_noiso(&self) -> FORCE_NOISO_R {
        FORCE_NOISO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Fast RTC memory PD following CPU 0: fast RTC memory PD following RTC state machine"]
    #[inline(always)]
    pub fn fastmem_folw_cpu(&self) -> FASTMEM_FOLW_CPU_R {
        FASTMEM_FOLW_CPU_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fast RTC memory force PD"]
    #[inline(always)]
    pub fn fastmem_force_lpd(&self) -> FASTMEM_FORCE_LPD_R {
        FASTMEM_FORCE_LPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fast RTC memory force no PD"]
    #[inline(always)]
    pub fn fastmem_force_lpu(&self) -> FASTMEM_FORCE_LPU_R {
        FASTMEM_FORCE_LPU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: RTC memory PD following CPU 0: RTC memory PD following RTC state machine"]
    #[inline(always)]
    pub fn slowmem_folw_cpu(&self) -> SLOWMEM_FOLW_CPU_R {
        SLOWMEM_FOLW_CPU_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC memory force PD"]
    #[inline(always)]
    pub fn slowmem_force_lpd(&self) -> SLOWMEM_FORCE_LPD_R {
        SLOWMEM_FORCE_LPD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RTC memory force no PD"]
    #[inline(always)]
    pub fn slowmem_force_lpu(&self) -> SLOWMEM_FORCE_LPU_R {
        SLOWMEM_FORCE_LPU_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Fast RTC memory force power down"]
    #[inline(always)]
    pub fn fastmem_force_pd(&self) -> FASTMEM_FORCE_PD_R {
        FASTMEM_FORCE_PD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Fast RTC memory force power up"]
    #[inline(always)]
    pub fn fastmem_force_pu(&self) -> FASTMEM_FORCE_PU_R {
        FASTMEM_FORCE_PU_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - enable power down fast RTC memory in sleep"]
    #[inline(always)]
    pub fn fastmem_pd_en(&self) -> FASTMEM_PD_EN_R {
        FASTMEM_PD_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC memory force power down"]
    #[inline(always)]
    pub fn slowmem_force_pd(&self) -> SLOWMEM_FORCE_PD_R {
        SLOWMEM_FORCE_PD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC memory force power up"]
    #[inline(always)]
    pub fn slowmem_force_pu(&self) -> SLOWMEM_FORCE_PU_R {
        SLOWMEM_FORCE_PU_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - enable power down RTC memory in sleep"]
    #[inline(always)]
    pub fn slowmem_pd_en(&self) -> SLOWMEM_PD_EN_R {
        SLOWMEM_PD_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - rtc_peri force power down"]
    #[inline(always)]
    pub fn force_pd(&self) -> FORCE_PD_R {
        FORCE_PD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - rtc_peri force power up"]
    #[inline(always)]
    pub fn force_pu(&self) -> FORCE_PU_R {
        FORCE_PU_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - enable power down rtc_peri in sleep"]
    #[inline(always)]
    pub fn pd_en(&self) -> PD_EN_R {
        PD_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWC")
            .field("fastmem_force_noiso", &self.fastmem_force_noiso())
            .field("fastmem_force_iso", &self.fastmem_force_iso())
            .field("slowmem_force_noiso", &self.slowmem_force_noiso())
            .field("slowmem_force_iso", &self.slowmem_force_iso())
            .field("force_iso", &self.force_iso())
            .field("force_noiso", &self.force_noiso())
            .field("fastmem_folw_cpu", &self.fastmem_folw_cpu())
            .field("fastmem_force_lpd", &self.fastmem_force_lpd())
            .field("fastmem_force_lpu", &self.fastmem_force_lpu())
            .field("slowmem_folw_cpu", &self.slowmem_folw_cpu())
            .field("slowmem_force_lpd", &self.slowmem_force_lpd())
            .field("slowmem_force_lpu", &self.slowmem_force_lpu())
            .field("fastmem_force_pd", &self.fastmem_force_pd())
            .field("fastmem_force_pu", &self.fastmem_force_pu())
            .field("fastmem_pd_en", &self.fastmem_pd_en())
            .field("slowmem_force_pd", &self.slowmem_force_pd())
            .field("slowmem_force_pu", &self.slowmem_force_pu())
            .field("slowmem_pd_en", &self.slowmem_pd_en())
            .field("force_pd", &self.force_pd())
            .field("force_pu", &self.force_pu())
            .field("pd_en", &self.pd_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Fast RTC memory force no ISO"]
    #[inline(always)]
    #[must_use]
    pub fn fastmem_force_noiso(&mut self) -> FASTMEM_FORCE_NOISO_W<PWC_SPEC> {
        FASTMEM_FORCE_NOISO_W::new(self, 0)
    }
    #[doc = "Bit 1 - Fast RTC memory force ISO"]
    #[inline(always)]
    #[must_use]
    pub fn fastmem_force_iso(&mut self) -> FASTMEM_FORCE_ISO_W<PWC_SPEC> {
        FASTMEM_FORCE_ISO_W::new(self, 1)
    }
    #[doc = "Bit 2 - RTC memory force no ISO"]
    #[inline(always)]
    #[must_use]
    pub fn slowmem_force_noiso(&mut self) -> SLOWMEM_FORCE_NOISO_W<PWC_SPEC> {
        SLOWMEM_FORCE_NOISO_W::new(self, 2)
    }
    #[doc = "Bit 3 - RTC memory force ISO"]
    #[inline(always)]
    #[must_use]
    pub fn slowmem_force_iso(&mut self) -> SLOWMEM_FORCE_ISO_W<PWC_SPEC> {
        SLOWMEM_FORCE_ISO_W::new(self, 3)
    }
    #[doc = "Bit 4 - rtc_peri force ISO"]
    #[inline(always)]
    #[must_use]
    pub fn force_iso(&mut self) -> FORCE_ISO_W<PWC_SPEC> {
        FORCE_ISO_W::new(self, 4)
    }
    #[doc = "Bit 5 - rtc_peri force no ISO"]
    #[inline(always)]
    #[must_use]
    pub fn force_noiso(&mut self) -> FORCE_NOISO_W<PWC_SPEC> {
        FORCE_NOISO_W::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Fast RTC memory PD following CPU 0: fast RTC memory PD following RTC state machine"]
    #[inline(always)]
    #[must_use]
    pub fn fastmem_folw_cpu(&mut self) -> FASTMEM_FOLW_CPU_W<PWC_SPEC> {
        FASTMEM_FOLW_CPU_W::new(self, 6)
    }
    #[doc = "Bit 7 - Fast RTC memory force PD"]
    #[inline(always)]
    #[must_use]
    pub fn fastmem_force_lpd(&mut self) -> FASTMEM_FORCE_LPD_W<PWC_SPEC> {
        FASTMEM_FORCE_LPD_W::new(self, 7)
    }
    #[doc = "Bit 8 - Fast RTC memory force no PD"]
    #[inline(always)]
    #[must_use]
    pub fn fastmem_force_lpu(&mut self) -> FASTMEM_FORCE_LPU_W<PWC_SPEC> {
        FASTMEM_FORCE_LPU_W::new(self, 8)
    }
    #[doc = "Bit 9 - 1: RTC memory PD following CPU 0: RTC memory PD following RTC state machine"]
    #[inline(always)]
    #[must_use]
    pub fn slowmem_folw_cpu(&mut self) -> SLOWMEM_FOLW_CPU_W<PWC_SPEC> {
        SLOWMEM_FOLW_CPU_W::new(self, 9)
    }
    #[doc = "Bit 10 - RTC memory force PD"]
    #[inline(always)]
    #[must_use]
    pub fn slowmem_force_lpd(&mut self) -> SLOWMEM_FORCE_LPD_W<PWC_SPEC> {
        SLOWMEM_FORCE_LPD_W::new(self, 10)
    }
    #[doc = "Bit 11 - RTC memory force no PD"]
    #[inline(always)]
    #[must_use]
    pub fn slowmem_force_lpu(&mut self) -> SLOWMEM_FORCE_LPU_W<PWC_SPEC> {
        SLOWMEM_FORCE_LPU_W::new(self, 11)
    }
    #[doc = "Bit 12 - Fast RTC memory force power down"]
    #[inline(always)]
    #[must_use]
    pub fn fastmem_force_pd(&mut self) -> FASTMEM_FORCE_PD_W<PWC_SPEC> {
        FASTMEM_FORCE_PD_W::new(self, 12)
    }
    #[doc = "Bit 13 - Fast RTC memory force power up"]
    #[inline(always)]
    #[must_use]
    pub fn fastmem_force_pu(&mut self) -> FASTMEM_FORCE_PU_W<PWC_SPEC> {
        FASTMEM_FORCE_PU_W::new(self, 13)
    }
    #[doc = "Bit 14 - enable power down fast RTC memory in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn fastmem_pd_en(&mut self) -> FASTMEM_PD_EN_W<PWC_SPEC> {
        FASTMEM_PD_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - RTC memory force power down"]
    #[inline(always)]
    #[must_use]
    pub fn slowmem_force_pd(&mut self) -> SLOWMEM_FORCE_PD_W<PWC_SPEC> {
        SLOWMEM_FORCE_PD_W::new(self, 15)
    }
    #[doc = "Bit 16 - RTC memory force power up"]
    #[inline(always)]
    #[must_use]
    pub fn slowmem_force_pu(&mut self) -> SLOWMEM_FORCE_PU_W<PWC_SPEC> {
        SLOWMEM_FORCE_PU_W::new(self, 16)
    }
    #[doc = "Bit 17 - enable power down RTC memory in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn slowmem_pd_en(&mut self) -> SLOWMEM_PD_EN_W<PWC_SPEC> {
        SLOWMEM_PD_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - rtc_peri force power down"]
    #[inline(always)]
    #[must_use]
    pub fn force_pd(&mut self) -> FORCE_PD_W<PWC_SPEC> {
        FORCE_PD_W::new(self, 18)
    }
    #[doc = "Bit 19 - rtc_peri force power up"]
    #[inline(always)]
    #[must_use]
    pub fn force_pu(&mut self) -> FORCE_PU_W<PWC_SPEC> {
        FORCE_PU_W::new(self, 19)
    }
    #[doc = "Bit 20 - enable power down rtc_peri in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn pd_en(&mut self) -> PD_EN_W<PWC_SPEC> {
        PD_EN_W::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWC_SPEC;
impl crate::RegisterSpec for PWC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwc::R`](R) reader structure"]
impl crate::Readable for PWC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwc::W`](W) writer structure"]
impl crate::Writable for PWC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWC to value 0x0001_2925"]
impl crate::Resettable for PWC_SPEC {
    const RESET_VALUE: u32 = 0x0001_2925;
}
