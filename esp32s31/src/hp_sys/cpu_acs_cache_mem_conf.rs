#[doc = "Register `CPU_ACS_CACHE_MEM_CONF` reader"]
pub type R = crate::R<CPU_ACS_CACHE_MEM_CONF_SPEC>;
#[doc = "Register `CPU_ACS_CACHE_MEM_CONF` writer"]
pub type W = crate::W<CPU_ACS_CACHE_MEM_CONF_SPEC>;
#[doc = "Field `CPU_ACCESS_AC0_DATAMEM_EN` reader - Set this bit to enable CPU accessROMCache0 data mem path"]
pub type CPU_ACCESS_AC0_DATAMEM_EN_R = crate::BitReader;
#[doc = "Field `CPU_ACCESS_AC1_DATAMEM_EN` reader - Set this bit to enable CPU access ROMCache1 data mem path"]
pub type CPU_ACCESS_AC1_DATAMEM_EN_R = crate::BitReader;
#[doc = "Field `CPU_ACCESS_IC0_DATAMEM_EN` reader - Set this bit to enable CPU access ICache0 data mem path"]
pub type CPU_ACCESS_IC0_DATAMEM_EN_R = crate::BitReader;
#[doc = "Field `CPU_ACCESS_IC0_DATAMEM_EN` writer - Set this bit to enable CPU access ICache0 data mem path"]
pub type CPU_ACCESS_IC0_DATAMEM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_ACCESS_IC1_DATAMEM_EN` reader - Set this bit to enable CPU access ICache1 data mem path"]
pub type CPU_ACCESS_IC1_DATAMEM_EN_R = crate::BitReader;
#[doc = "Field `CPU_ACCESS_IC1_DATAMEM_EN` writer - Set this bit to enable CPU access ICache1 data mem path"]
pub type CPU_ACCESS_IC1_DATAMEM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_ACCESS_DC_DATAMEM_EN` reader - Set this bit to enable CPU access DCache data mem path"]
pub type CPU_ACCESS_DC_DATAMEM_EN_R = crate::BitReader;
#[doc = "Field `CPU_ACCESS_DC_DATAMEM_EN` writer - Set this bit to enable CPU access DCache data mem path"]
pub type CPU_ACCESS_DC_DATAMEM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_ACS_CACHE_MEM_LOCK` reader - Set 1 to lock cpu acs mem data mem path configuration"]
pub type CPU_ACS_CACHE_MEM_LOCK_R = crate::BitReader;
#[doc = "Field `CPU_ACS_CACHE_MEM_LOCK` writer - Set 1 to lock cpu acs mem data mem path configuration"]
pub type CPU_ACS_CACHE_MEM_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable CPU accessROMCache0 data mem path"]
    #[inline(always)]
    pub fn cpu_access_ac0_datamem_en(&self) -> CPU_ACCESS_AC0_DATAMEM_EN_R {
        CPU_ACCESS_AC0_DATAMEM_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable CPU access ROMCache1 data mem path"]
    #[inline(always)]
    pub fn cpu_access_ac1_datamem_en(&self) -> CPU_ACCESS_AC1_DATAMEM_EN_R {
        CPU_ACCESS_AC1_DATAMEM_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable CPU access ICache0 data mem path"]
    #[inline(always)]
    pub fn cpu_access_ic0_datamem_en(&self) -> CPU_ACCESS_IC0_DATAMEM_EN_R {
        CPU_ACCESS_IC0_DATAMEM_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable CPU access ICache1 data mem path"]
    #[inline(always)]
    pub fn cpu_access_ic1_datamem_en(&self) -> CPU_ACCESS_IC1_DATAMEM_EN_R {
        CPU_ACCESS_IC1_DATAMEM_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable CPU access DCache data mem path"]
    #[inline(always)]
    pub fn cpu_access_dc_datamem_en(&self) -> CPU_ACCESS_DC_DATAMEM_EN_R {
        CPU_ACCESS_DC_DATAMEM_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set 1 to lock cpu acs mem data mem path configuration"]
    #[inline(always)]
    pub fn cpu_acs_cache_mem_lock(&self) -> CPU_ACS_CACHE_MEM_LOCK_R {
        CPU_ACS_CACHE_MEM_LOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_ACS_CACHE_MEM_CONF")
            .field(
                "cpu_access_ac0_datamem_en",
                &self.cpu_access_ac0_datamem_en(),
            )
            .field(
                "cpu_access_ac1_datamem_en",
                &self.cpu_access_ac1_datamem_en(),
            )
            .field(
                "cpu_access_ic0_datamem_en",
                &self.cpu_access_ic0_datamem_en(),
            )
            .field(
                "cpu_access_ic1_datamem_en",
                &self.cpu_access_ic1_datamem_en(),
            )
            .field("cpu_access_dc_datamem_en", &self.cpu_access_dc_datamem_en())
            .field("cpu_acs_cache_mem_lock", &self.cpu_acs_cache_mem_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Set this bit to enable CPU access ICache0 data mem path"]
    #[inline(always)]
    pub fn cpu_access_ic0_datamem_en(
        &mut self,
    ) -> CPU_ACCESS_IC0_DATAMEM_EN_W<'_, CPU_ACS_CACHE_MEM_CONF_SPEC> {
        CPU_ACCESS_IC0_DATAMEM_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to enable CPU access ICache1 data mem path"]
    #[inline(always)]
    pub fn cpu_access_ic1_datamem_en(
        &mut self,
    ) -> CPU_ACCESS_IC1_DATAMEM_EN_W<'_, CPU_ACS_CACHE_MEM_CONF_SPEC> {
        CPU_ACCESS_IC1_DATAMEM_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to enable CPU access DCache data mem path"]
    #[inline(always)]
    pub fn cpu_access_dc_datamem_en(
        &mut self,
    ) -> CPU_ACCESS_DC_DATAMEM_EN_W<'_, CPU_ACS_CACHE_MEM_CONF_SPEC> {
        CPU_ACCESS_DC_DATAMEM_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set 1 to lock cpu acs mem data mem path configuration"]
    #[inline(always)]
    pub fn cpu_acs_cache_mem_lock(
        &mut self,
    ) -> CPU_ACS_CACHE_MEM_LOCK_W<'_, CPU_ACS_CACHE_MEM_CONF_SPEC> {
        CPU_ACS_CACHE_MEM_LOCK_W::new(self, 5)
    }
}
#[doc = "CPU access Cache data mem configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_acs_cache_mem_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_acs_cache_mem_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_ACS_CACHE_MEM_CONF_SPEC;
impl crate::RegisterSpec for CPU_ACS_CACHE_MEM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_acs_cache_mem_conf::R`](R) reader structure"]
impl crate::Readable for CPU_ACS_CACHE_MEM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_acs_cache_mem_conf::W`](W) writer structure"]
impl crate::Writable for CPU_ACS_CACHE_MEM_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_ACS_CACHE_MEM_CONF to value 0"]
impl crate::Resettable for CPU_ACS_CACHE_MEM_CONF_SPEC {}
