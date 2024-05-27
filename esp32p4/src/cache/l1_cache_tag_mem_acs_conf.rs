///Register `L1_CACHE_TAG_MEM_ACS_CONF` reader
pub type R = crate::R<L1_CACHE_TAG_MEM_ACS_CONF_SPEC>;
///Register `L1_CACHE_TAG_MEM_ACS_CONF` writer
pub type W = crate::W<L1_CACHE_TAG_MEM_ACS_CONF_SPEC>;
///Field `L1_ICACHE0_TAG_MEM_RD_EN` reader - The bit is used to enable config-bus read L1-ICache0 tag memoryory. 0: disable, 1: enable.
pub type L1_ICACHE0_TAG_MEM_RD_EN_R = crate::BitReader;
///Field `L1_ICACHE0_TAG_MEM_RD_EN` writer - The bit is used to enable config-bus read L1-ICache0 tag memoryory. 0: disable, 1: enable.
pub type L1_ICACHE0_TAG_MEM_RD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_ICACHE0_TAG_MEM_WR_EN` reader - The bit is used to enable config-bus write L1-ICache0 tag memoryory. 0: disable, 1: enable.
pub type L1_ICACHE0_TAG_MEM_WR_EN_R = crate::BitReader;
///Field `L1_ICACHE0_TAG_MEM_WR_EN` writer - The bit is used to enable config-bus write L1-ICache0 tag memoryory. 0: disable, 1: enable.
pub type L1_ICACHE0_TAG_MEM_WR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_ICACHE1_TAG_MEM_RD_EN` reader - The bit is used to enable config-bus read L1-ICache1 tag memoryory. 0: disable, 1: enable.
pub type L1_ICACHE1_TAG_MEM_RD_EN_R = crate::BitReader;
///Field `L1_ICACHE1_TAG_MEM_RD_EN` writer - The bit is used to enable config-bus read L1-ICache1 tag memoryory. 0: disable, 1: enable.
pub type L1_ICACHE1_TAG_MEM_RD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_ICACHE1_TAG_MEM_WR_EN` reader - The bit is used to enable config-bus write L1-ICache1 tag memoryory. 0: disable, 1: enable.
pub type L1_ICACHE1_TAG_MEM_WR_EN_R = crate::BitReader;
///Field `L1_ICACHE1_TAG_MEM_WR_EN` writer - The bit is used to enable config-bus write L1-ICache1 tag memoryory. 0: disable, 1: enable.
pub type L1_ICACHE1_TAG_MEM_WR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_ICACHE2_TAG_MEM_RD_EN` reader - Reserved
pub type L1_ICACHE2_TAG_MEM_RD_EN_R = crate::BitReader;
///Field `L1_ICACHE2_TAG_MEM_WR_EN` reader - Reserved
pub type L1_ICACHE2_TAG_MEM_WR_EN_R = crate::BitReader;
///Field `L1_ICACHE3_TAG_MEM_RD_EN` reader - Reserved
pub type L1_ICACHE3_TAG_MEM_RD_EN_R = crate::BitReader;
///Field `L1_ICACHE3_TAG_MEM_WR_EN` reader - Reserved
pub type L1_ICACHE3_TAG_MEM_WR_EN_R = crate::BitReader;
///Field `L1_DCACHE_TAG_MEM_RD_EN` reader - The bit is used to enable config-bus read L1-DCache tag memoryory. 0: disable, 1: enable.
pub type L1_DCACHE_TAG_MEM_RD_EN_R = crate::BitReader;
///Field `L1_DCACHE_TAG_MEM_RD_EN` writer - The bit is used to enable config-bus read L1-DCache tag memoryory. 0: disable, 1: enable.
pub type L1_DCACHE_TAG_MEM_RD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_DCACHE_TAG_MEM_WR_EN` reader - The bit is used to enable config-bus write L1-DCache tag memoryory. 0: disable, 1: enable.
pub type L1_DCACHE_TAG_MEM_WR_EN_R = crate::BitReader;
///Field `L1_DCACHE_TAG_MEM_WR_EN` writer - The bit is used to enable config-bus write L1-DCache tag memoryory. 0: disable, 1: enable.
pub type L1_DCACHE_TAG_MEM_WR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - The bit is used to enable config-bus read L1-ICache0 tag memoryory. 0: disable, 1: enable.
    #[inline(always)]
    pub fn l1_icache0_tag_mem_rd_en(&self) -> L1_ICACHE0_TAG_MEM_RD_EN_R {
        L1_ICACHE0_TAG_MEM_RD_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The bit is used to enable config-bus write L1-ICache0 tag memoryory. 0: disable, 1: enable.
    #[inline(always)]
    pub fn l1_icache0_tag_mem_wr_en(&self) -> L1_ICACHE0_TAG_MEM_WR_EN_R {
        L1_ICACHE0_TAG_MEM_WR_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - The bit is used to enable config-bus read L1-ICache1 tag memoryory. 0: disable, 1: enable.
    #[inline(always)]
    pub fn l1_icache1_tag_mem_rd_en(&self) -> L1_ICACHE1_TAG_MEM_RD_EN_R {
        L1_ICACHE1_TAG_MEM_RD_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - The bit is used to enable config-bus write L1-ICache1 tag memoryory. 0: disable, 1: enable.
    #[inline(always)]
    pub fn l1_icache1_tag_mem_wr_en(&self) -> L1_ICACHE1_TAG_MEM_WR_EN_R {
        L1_ICACHE1_TAG_MEM_WR_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Reserved
    #[inline(always)]
    pub fn l1_icache2_tag_mem_rd_en(&self) -> L1_ICACHE2_TAG_MEM_RD_EN_R {
        L1_ICACHE2_TAG_MEM_RD_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Reserved
    #[inline(always)]
    pub fn l1_icache2_tag_mem_wr_en(&self) -> L1_ICACHE2_TAG_MEM_WR_EN_R {
        L1_ICACHE2_TAG_MEM_WR_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - Reserved
    #[inline(always)]
    pub fn l1_icache3_tag_mem_rd_en(&self) -> L1_ICACHE3_TAG_MEM_RD_EN_R {
        L1_ICACHE3_TAG_MEM_RD_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Reserved
    #[inline(always)]
    pub fn l1_icache3_tag_mem_wr_en(&self) -> L1_ICACHE3_TAG_MEM_WR_EN_R {
        L1_ICACHE3_TAG_MEM_WR_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - The bit is used to enable config-bus read L1-DCache tag memoryory. 0: disable, 1: enable.
    #[inline(always)]
    pub fn l1_dcache_tag_mem_rd_en(&self) -> L1_DCACHE_TAG_MEM_RD_EN_R {
        L1_DCACHE_TAG_MEM_RD_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - The bit is used to enable config-bus write L1-DCache tag memoryory. 0: disable, 1: enable.
    #[inline(always)]
    pub fn l1_dcache_tag_mem_wr_en(&self) -> L1_DCACHE_TAG_MEM_WR_EN_R {
        L1_DCACHE_TAG_MEM_WR_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_TAG_MEM_ACS_CONF")
            .field("l1_icache0_tag_mem_rd_en", &self.l1_icache0_tag_mem_rd_en())
            .field("l1_icache0_tag_mem_wr_en", &self.l1_icache0_tag_mem_wr_en())
            .field("l1_icache1_tag_mem_rd_en", &self.l1_icache1_tag_mem_rd_en())
            .field("l1_icache1_tag_mem_wr_en", &self.l1_icache1_tag_mem_wr_en())
            .field("l1_icache2_tag_mem_rd_en", &self.l1_icache2_tag_mem_rd_en())
            .field("l1_icache2_tag_mem_wr_en", &self.l1_icache2_tag_mem_wr_en())
            .field("l1_icache3_tag_mem_rd_en", &self.l1_icache3_tag_mem_rd_en())
            .field("l1_icache3_tag_mem_wr_en", &self.l1_icache3_tag_mem_wr_en())
            .field("l1_dcache_tag_mem_rd_en", &self.l1_dcache_tag_mem_rd_en())
            .field("l1_dcache_tag_mem_wr_en", &self.l1_dcache_tag_mem_wr_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - The bit is used to enable config-bus read L1-ICache0 tag memoryory. 0: disable, 1: enable.
    #[inline(always)]
    #[must_use]
    pub fn l1_icache0_tag_mem_rd_en(
        &mut self,
    ) -> L1_ICACHE0_TAG_MEM_RD_EN_W<L1_CACHE_TAG_MEM_ACS_CONF_SPEC> {
        L1_ICACHE0_TAG_MEM_RD_EN_W::new(self, 0)
    }
    ///Bit 1 - The bit is used to enable config-bus write L1-ICache0 tag memoryory. 0: disable, 1: enable.
    #[inline(always)]
    #[must_use]
    pub fn l1_icache0_tag_mem_wr_en(
        &mut self,
    ) -> L1_ICACHE0_TAG_MEM_WR_EN_W<L1_CACHE_TAG_MEM_ACS_CONF_SPEC> {
        L1_ICACHE0_TAG_MEM_WR_EN_W::new(self, 1)
    }
    ///Bit 4 - The bit is used to enable config-bus read L1-ICache1 tag memoryory. 0: disable, 1: enable.
    #[inline(always)]
    #[must_use]
    pub fn l1_icache1_tag_mem_rd_en(
        &mut self,
    ) -> L1_ICACHE1_TAG_MEM_RD_EN_W<L1_CACHE_TAG_MEM_ACS_CONF_SPEC> {
        L1_ICACHE1_TAG_MEM_RD_EN_W::new(self, 4)
    }
    ///Bit 5 - The bit is used to enable config-bus write L1-ICache1 tag memoryory. 0: disable, 1: enable.
    #[inline(always)]
    #[must_use]
    pub fn l1_icache1_tag_mem_wr_en(
        &mut self,
    ) -> L1_ICACHE1_TAG_MEM_WR_EN_W<L1_CACHE_TAG_MEM_ACS_CONF_SPEC> {
        L1_ICACHE1_TAG_MEM_WR_EN_W::new(self, 5)
    }
    ///Bit 16 - The bit is used to enable config-bus read L1-DCache tag memoryory. 0: disable, 1: enable.
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_tag_mem_rd_en(
        &mut self,
    ) -> L1_DCACHE_TAG_MEM_RD_EN_W<L1_CACHE_TAG_MEM_ACS_CONF_SPEC> {
        L1_DCACHE_TAG_MEM_RD_EN_W::new(self, 16)
    }
    ///Bit 17 - The bit is used to enable config-bus write L1-DCache tag memoryory. 0: disable, 1: enable.
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_tag_mem_wr_en(
        &mut self,
    ) -> L1_DCACHE_TAG_MEM_WR_EN_W<L1_CACHE_TAG_MEM_ACS_CONF_SPEC> {
        L1_DCACHE_TAG_MEM_WR_EN_W::new(self, 17)
    }
}
/**Cache tag memory access configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_tag_mem_acs_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_tag_mem_acs_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L1_CACHE_TAG_MEM_ACS_CONF_SPEC;
impl crate::RegisterSpec for L1_CACHE_TAG_MEM_ACS_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l1_cache_tag_mem_acs_conf::R`](R) reader structure
impl crate::Readable for L1_CACHE_TAG_MEM_ACS_CONF_SPEC {}
///`write(|w| ..)` method takes [`l1_cache_tag_mem_acs_conf::W`](W) writer structure
impl crate::Writable for L1_CACHE_TAG_MEM_ACS_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L1_CACHE_TAG_MEM_ACS_CONF to value 0x0003_3333
impl crate::Resettable for L1_CACHE_TAG_MEM_ACS_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0003_3333;
}
