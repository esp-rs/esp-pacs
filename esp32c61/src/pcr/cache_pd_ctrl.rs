#[doc = "Register `CACHE_PD_CTRL` reader"]
pub type R = crate::R<CACHE_PD_CTRL_SPEC>;
#[doc = "Register `CACHE_PD_CTRL` writer"]
pub type W = crate::W<CACHE_PD_CTRL_SPEC>;
#[doc = "Field `CACHE_MEM_FORCE_PU` reader - Set this bit to force power down CACHE memory."]
pub type CACHE_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `CACHE_MEM_FORCE_PU` writer - Set this bit to force power down CACHE memory."]
pub type CACHE_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_MEM_FORCE_PD` reader - Set this bit to force power up CACHE memory."]
pub type CACHE_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `CACHE_MEM_FORCE_PD` writer - Set this bit to force power up CACHE memory."]
pub type CACHE_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Set this bit to force power down CACHE memory."]
    #[inline(always)]
    pub fn cache_mem_force_pu(&self) -> CACHE_MEM_FORCE_PU_R {
        CACHE_MEM_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force power up CACHE memory."]
    #[inline(always)]
    pub fn cache_mem_force_pd(&self) -> CACHE_MEM_FORCE_PD_R {
        CACHE_MEM_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_PD_CTRL")
            .field("cache_mem_force_pu", &self.cache_mem_force_pu())
            .field("cache_mem_force_pd", &self.cache_mem_force_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Set this bit to force power down CACHE memory."]
    #[inline(always)]
    pub fn cache_mem_force_pu(&mut self) -> CACHE_MEM_FORCE_PU_W<'_, CACHE_PD_CTRL_SPEC> {
        CACHE_MEM_FORCE_PU_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force power up CACHE memory."]
    #[inline(always)]
    pub fn cache_mem_force_pd(&mut self) -> CACHE_MEM_FORCE_PD_W<'_, CACHE_PD_CTRL_SPEC> {
        CACHE_MEM_FORCE_PD_W::new(self, 2)
    }
}
#[doc = "CACHE power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_pd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_pd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_PD_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_PD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_pd_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_PD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_pd_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_PD_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_PD_CTRL to value 0x02"]
impl crate::Resettable for CACHE_PD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
