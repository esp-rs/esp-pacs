#[doc = "Register `POWER_PD_TOP_CNTL` reader"]
pub type R = crate::R<POWER_PD_TOP_CNTL_SPEC>;
#[doc = "Register `POWER_PD_TOP_CNTL` writer"]
pub type W = crate::W<POWER_PD_TOP_CNTL_SPEC>;
#[doc = "Field `FORCE_TOP_RESET` reader - need_des"]
pub type FORCE_TOP_RESET_R = crate::BitReader;
#[doc = "Field `FORCE_TOP_RESET` writer - need_des"]
pub type FORCE_TOP_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_TOP_ISO` reader - need_des"]
pub type FORCE_TOP_ISO_R = crate::BitReader;
#[doc = "Field `FORCE_TOP_ISO` writer - need_des"]
pub type FORCE_TOP_ISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_TOP_PU` reader - need_des"]
pub type FORCE_TOP_PU_R = crate::BitReader;
#[doc = "Field `FORCE_TOP_PU` writer - need_des"]
pub type FORCE_TOP_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_TOP_NO_RESET` reader - need_des"]
pub type FORCE_TOP_NO_RESET_R = crate::BitReader;
#[doc = "Field `FORCE_TOP_NO_RESET` writer - need_des"]
pub type FORCE_TOP_NO_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_TOP_NO_ISO` reader - need_des"]
pub type FORCE_TOP_NO_ISO_R = crate::BitReader;
#[doc = "Field `FORCE_TOP_NO_ISO` writer - need_des"]
pub type FORCE_TOP_NO_ISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_TOP_PD` reader - need_des"]
pub type FORCE_TOP_PD_R = crate::BitReader;
#[doc = "Field `FORCE_TOP_PD` writer - need_des"]
pub type FORCE_TOP_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn force_top_reset(&self) -> FORCE_TOP_RESET_R {
        FORCE_TOP_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn force_top_iso(&self) -> FORCE_TOP_ISO_R {
        FORCE_TOP_ISO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_top_pu(&self) -> FORCE_TOP_PU_R {
        FORCE_TOP_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn force_top_no_reset(&self) -> FORCE_TOP_NO_RESET_R {
        FORCE_TOP_NO_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn force_top_no_iso(&self) -> FORCE_TOP_NO_ISO_R {
        FORCE_TOP_NO_ISO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn force_top_pd(&self) -> FORCE_TOP_PD_R {
        FORCE_TOP_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_PD_TOP_CNTL")
            .field("force_top_reset", &self.force_top_reset())
            .field("force_top_iso", &self.force_top_iso())
            .field("force_top_pu", &self.force_top_pu())
            .field("force_top_no_reset", &self.force_top_no_reset())
            .field("force_top_no_iso", &self.force_top_no_iso())
            .field("force_top_pd", &self.force_top_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn force_top_reset(&mut self) -> FORCE_TOP_RESET_W<POWER_PD_TOP_CNTL_SPEC> {
        FORCE_TOP_RESET_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn force_top_iso(&mut self) -> FORCE_TOP_ISO_W<POWER_PD_TOP_CNTL_SPEC> {
        FORCE_TOP_ISO_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_top_pu(&mut self) -> FORCE_TOP_PU_W<POWER_PD_TOP_CNTL_SPEC> {
        FORCE_TOP_PU_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn force_top_no_reset(&mut self) -> FORCE_TOP_NO_RESET_W<POWER_PD_TOP_CNTL_SPEC> {
        FORCE_TOP_NO_RESET_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn force_top_no_iso(&mut self) -> FORCE_TOP_NO_ISO_W<POWER_PD_TOP_CNTL_SPEC> {
        FORCE_TOP_NO_ISO_W::new(self, 4)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn force_top_pd(&mut self) -> FORCE_TOP_PD_W<POWER_PD_TOP_CNTL_SPEC> {
        FORCE_TOP_PD_W::new(self, 5)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_top_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_top_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_PD_TOP_CNTL_SPEC;
impl crate::RegisterSpec for POWER_PD_TOP_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_pd_top_cntl::R`](R) reader structure"]
impl crate::Readable for POWER_PD_TOP_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power_pd_top_cntl::W`](W) writer structure"]
impl crate::Writable for POWER_PD_TOP_CNTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER_PD_TOP_CNTL to value 0x1c"]
impl crate::Resettable for POWER_PD_TOP_CNTL_SPEC {
    const RESET_VALUE: u32 = 0x1c;
}
