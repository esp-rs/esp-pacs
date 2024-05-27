///Register `L2_CACHE_DATA_MEM_ACS_CONF` reader
pub type R = crate::R<L2_CACHE_DATA_MEM_ACS_CONF_SPEC>;
///Field `L2_CACHE_DATA_MEM_RD_EN` reader - The bit is used to enable config-bus read L2-Cache data memoryory. 0: disable, 1: enable.
pub type L2_CACHE_DATA_MEM_RD_EN_R = crate::BitReader;
///Field `L2_CACHE_DATA_MEM_WR_EN` reader - The bit is used to enable config-bus write L2-Cache data memoryory. 0: disable, 1: enable.
pub type L2_CACHE_DATA_MEM_WR_EN_R = crate::BitReader;
impl R {
    ///Bit 20 - The bit is used to enable config-bus read L2-Cache data memoryory. 0: disable, 1: enable.
    #[inline(always)]
    pub fn l2_cache_data_mem_rd_en(&self) -> L2_CACHE_DATA_MEM_RD_EN_R {
        L2_CACHE_DATA_MEM_RD_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - The bit is used to enable config-bus write L2-Cache data memoryory. 0: disable, 1: enable.
    #[inline(always)]
    pub fn l2_cache_data_mem_wr_en(&self) -> L2_CACHE_DATA_MEM_WR_EN_R {
        L2_CACHE_DATA_MEM_WR_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_DATA_MEM_ACS_CONF")
            .field("l2_cache_data_mem_rd_en", &self.l2_cache_data_mem_rd_en())
            .field("l2_cache_data_mem_wr_en", &self.l2_cache_data_mem_wr_en())
            .finish()
    }
}
/**Cache data memory access configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_data_mem_acs_conf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L2_CACHE_DATA_MEM_ACS_CONF_SPEC;
impl crate::RegisterSpec for L2_CACHE_DATA_MEM_ACS_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l2_cache_data_mem_acs_conf::R`](R) reader structure
impl crate::Readable for L2_CACHE_DATA_MEM_ACS_CONF_SPEC {}
///`reset()` method sets L2_CACHE_DATA_MEM_ACS_CONF to value 0
impl crate::Resettable for L2_CACHE_DATA_MEM_ACS_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
