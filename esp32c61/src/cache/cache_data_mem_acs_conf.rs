#[doc = "Register `CACHE_DATA_MEM_ACS_CONF` reader"]
pub type R = crate::R<CACHE_DATA_MEM_ACS_CONF_SPEC>;
#[doc = "Register `CACHE_DATA_MEM_ACS_CONF` writer"]
pub type W = crate::W<CACHE_DATA_MEM_ACS_CONF_SPEC>;
#[doc = "Field `ICACHE0_DATA_MEM_RD_EN` reader - The bit is used to enable config-bus read L1-ICache0 data memoryory. 0: disable, 1: enable."]
pub type ICACHE0_DATA_MEM_RD_EN_R = crate::BitReader;
#[doc = "Field `ICACHE0_DATA_MEM_WR_EN` reader - The bit is used to enable config-bus write L1-ICache0 data memoryory. 0: disable, 1: enable."]
pub type ICACHE0_DATA_MEM_WR_EN_R = crate::BitReader;
#[doc = "Field `ICACHE1_DATA_MEM_RD_EN` reader - The bit is used to enable config-bus read L1-ICache1 data memoryory. 0: disable, 1: enable."]
pub type ICACHE1_DATA_MEM_RD_EN_R = crate::BitReader;
#[doc = "Field `ICACHE1_DATA_MEM_WR_EN` reader - The bit is used to enable config-bus write L1-ICache1 data memoryory. 0: disable, 1: enable."]
pub type ICACHE1_DATA_MEM_WR_EN_R = crate::BitReader;
#[doc = "Field `ICACHE2_DATA_MEM_RD_EN` reader - Reserved"]
pub type ICACHE2_DATA_MEM_RD_EN_R = crate::BitReader;
#[doc = "Field `ICACHE2_DATA_MEM_WR_EN` reader - Reserved"]
pub type ICACHE2_DATA_MEM_WR_EN_R = crate::BitReader;
#[doc = "Field `ICACHE3_DATA_MEM_RD_EN` reader - Reserved"]
pub type ICACHE3_DATA_MEM_RD_EN_R = crate::BitReader;
#[doc = "Field `ICACHE3_DATA_MEM_WR_EN` reader - Reserved"]
pub type ICACHE3_DATA_MEM_WR_EN_R = crate::BitReader;
#[doc = "Field `CACHE_DATA_MEM_RD_EN` reader - The bit is used to enable config-bus read L1-Cache data memoryory. 0: disable, 1: enable."]
pub type CACHE_DATA_MEM_RD_EN_R = crate::BitReader;
#[doc = "Field `CACHE_DATA_MEM_RD_EN` writer - The bit is used to enable config-bus read L1-Cache data memoryory. 0: disable, 1: enable."]
pub type CACHE_DATA_MEM_RD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_DATA_MEM_WR_EN` reader - The bit is used to enable config-bus write L1-Cache data memoryory. 0: disable, 1: enable."]
pub type CACHE_DATA_MEM_WR_EN_R = crate::BitReader;
#[doc = "Field `CACHE_DATA_MEM_WR_EN` writer - The bit is used to enable config-bus write L1-Cache data memoryory. 0: disable, 1: enable."]
pub type CACHE_DATA_MEM_WR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable config-bus read L1-ICache0 data memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    pub fn icache0_data_mem_rd_en(&self) -> ICACHE0_DATA_MEM_RD_EN_R {
        ICACHE0_DATA_MEM_RD_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable config-bus write L1-ICache0 data memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    pub fn icache0_data_mem_wr_en(&self) -> ICACHE0_DATA_MEM_WR_EN_R {
        ICACHE0_DATA_MEM_WR_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable config-bus read L1-ICache1 data memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    pub fn icache1_data_mem_rd_en(&self) -> ICACHE1_DATA_MEM_RD_EN_R {
        ICACHE1_DATA_MEM_RD_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is used to enable config-bus write L1-ICache1 data memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    pub fn icache1_data_mem_wr_en(&self) -> ICACHE1_DATA_MEM_WR_EN_R {
        ICACHE1_DATA_MEM_WR_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn icache2_data_mem_rd_en(&self) -> ICACHE2_DATA_MEM_RD_EN_R {
        ICACHE2_DATA_MEM_RD_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn icache2_data_mem_wr_en(&self) -> ICACHE2_DATA_MEM_WR_EN_R {
        ICACHE2_DATA_MEM_WR_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn icache3_data_mem_rd_en(&self) -> ICACHE3_DATA_MEM_RD_EN_R {
        ICACHE3_DATA_MEM_RD_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn icache3_data_mem_wr_en(&self) -> ICACHE3_DATA_MEM_WR_EN_R {
        ICACHE3_DATA_MEM_WR_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - The bit is used to enable config-bus read L1-Cache data memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    pub fn cache_data_mem_rd_en(&self) -> CACHE_DATA_MEM_RD_EN_R {
        CACHE_DATA_MEM_RD_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The bit is used to enable config-bus write L1-Cache data memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    pub fn cache_data_mem_wr_en(&self) -> CACHE_DATA_MEM_WR_EN_R {
        CACHE_DATA_MEM_WR_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_DATA_MEM_ACS_CONF")
            .field("icache0_data_mem_rd_en", &self.icache0_data_mem_rd_en())
            .field("icache0_data_mem_wr_en", &self.icache0_data_mem_wr_en())
            .field("icache1_data_mem_rd_en", &self.icache1_data_mem_rd_en())
            .field("icache1_data_mem_wr_en", &self.icache1_data_mem_wr_en())
            .field("icache2_data_mem_rd_en", &self.icache2_data_mem_rd_en())
            .field("icache2_data_mem_wr_en", &self.icache2_data_mem_wr_en())
            .field("icache3_data_mem_rd_en", &self.icache3_data_mem_rd_en())
            .field("icache3_data_mem_wr_en", &self.icache3_data_mem_wr_en())
            .field("cache_data_mem_rd_en", &self.cache_data_mem_rd_en())
            .field("cache_data_mem_wr_en", &self.cache_data_mem_wr_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 16 - The bit is used to enable config-bus read L1-Cache data memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    pub fn cache_data_mem_rd_en(&mut self) -> CACHE_DATA_MEM_RD_EN_W<CACHE_DATA_MEM_ACS_CONF_SPEC> {
        CACHE_DATA_MEM_RD_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - The bit is used to enable config-bus write L1-Cache data memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    pub fn cache_data_mem_wr_en(&mut self) -> CACHE_DATA_MEM_WR_EN_W<CACHE_DATA_MEM_ACS_CONF_SPEC> {
        CACHE_DATA_MEM_WR_EN_W::new(self, 17)
    }
}
#[doc = "Cache data memory access configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_data_mem_acs_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_data_mem_acs_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_DATA_MEM_ACS_CONF_SPEC;
impl crate::RegisterSpec for CACHE_DATA_MEM_ACS_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_data_mem_acs_conf::R`](R) reader structure"]
impl crate::Readable for CACHE_DATA_MEM_ACS_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_data_mem_acs_conf::W`](W) writer structure"]
impl crate::Writable for CACHE_DATA_MEM_ACS_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_DATA_MEM_ACS_CONF to value 0"]
impl crate::Resettable for CACHE_DATA_MEM_ACS_CONF_SPEC {}
