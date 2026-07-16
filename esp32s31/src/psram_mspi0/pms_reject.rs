#[doc = "Register `PMS_REJECT` reader"]
pub type R = crate::R<PMS_REJECT_SPEC>;
#[doc = "Register `PMS_REJECT` writer"]
pub type W = crate::W<PMS_REJECT_SPEC>;
#[doc = "Field `PM_EN` reader - "]
pub type PM_EN_R = crate::BitReader;
#[doc = "Field `PM_EN` writer - "]
pub type PM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMS_LD` reader - "]
pub type PMS_LD_R = crate::BitReader;
#[doc = "Field `PMS_LD` writer - "]
pub type PMS_LD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMS_ST` reader - "]
pub type PMS_ST_R = crate::BitReader;
#[doc = "Field `PMS_ST` writer - "]
pub type PMS_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMS_MULTI_HIT` reader - "]
pub type PMS_MULTI_HIT_R = crate::BitReader;
#[doc = "Field `PMS_MULTI_HIT` writer - "]
pub type PMS_MULTI_HIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMS_IVD` reader - "]
pub type PMS_IVD_R = crate::BitReader;
#[doc = "Field `PMS_IVD` writer - "]
pub type PMS_IVD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pm_en(&self) -> PM_EN_R {
        PM_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pms_ld(&self) -> PMS_LD_R {
        PMS_LD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pms_st(&self) -> PMS_ST_R {
        PMS_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pms_multi_hit(&self) -> PMS_MULTI_HIT_R {
        PMS_MULTI_HIT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pms_ivd(&self) -> PMS_IVD_R {
        PMS_IVD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMS_REJECT")
            .field("pm_en", &self.pm_en())
            .field("pms_ld", &self.pms_ld())
            .field("pms_st", &self.pms_st())
            .field("pms_multi_hit", &self.pms_multi_hit())
            .field("pms_ivd", &self.pms_ivd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pm_en(&mut self) -> PM_EN_W<'_, PMS_REJECT_SPEC> {
        PM_EN_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pms_ld(&mut self) -> PMS_LD_W<'_, PMS_REJECT_SPEC> {
        PMS_LD_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pms_st(&mut self) -> PMS_ST_W<'_, PMS_REJECT_SPEC> {
        PMS_ST_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pms_multi_hit(&mut self) -> PMS_MULTI_HIT_W<'_, PMS_REJECT_SPEC> {
        PMS_MULTI_HIT_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pms_ivd(&mut self) -> PMS_IVD_W<'_, PMS_REJECT_SPEC> {
        PMS_IVD_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pms_reject::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pms_reject::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMS_REJECT_SPEC;
impl crate::RegisterSpec for PMS_REJECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pms_reject::R`](R) reader structure"]
impl crate::Readable for PMS_REJECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pms_reject::W`](W) writer structure"]
impl crate::Writable for PMS_REJECT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMS_REJECT to value 0"]
impl crate::Resettable for PMS_REJECT_SPEC {}
