#[doc = "Register `APP_DCACHE_DBUG0` reader"]
pub type R = crate::R<APP_DCACHE_DBUG0_SPEC>;
#[doc = "Register `APP_DCACHE_DBUG0` writer"]
pub type W = crate::W<APP_DCACHE_DBUG0_SPEC>;
#[doc = "Field `APP_SLAVE_WDATA` reader - "]
pub type APP_SLAVE_WDATA_R = crate::BitReader;
#[doc = "Field `APP_SLAVE_WDATA` writer - "]
pub type APP_SLAVE_WDATA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CACHE_MMU_IA` reader - "]
pub type APP_CACHE_MMU_IA_R = crate::BitReader;
#[doc = "Field `APP_CACHE_IA` reader - "]
pub type APP_CACHE_IA_R = crate::FieldReader;
#[doc = "Field `APP_CACHE_STATE` reader - "]
pub type APP_CACHE_STATE_R = crate::FieldReader<u16>;
#[doc = "Field `APP_WR_BAK_TO_READ` reader - "]
pub type APP_WR_BAK_TO_READ_R = crate::BitReader;
#[doc = "Field `APP_TX_END` reader - "]
pub type APP_TX_END_R = crate::BitReader;
#[doc = "Field `APP_SLAVE_WR` reader - "]
pub type APP_SLAVE_WR_R = crate::BitReader;
#[doc = "Field `APP_SLAVE_WDATA_V` reader - "]
pub type APP_SLAVE_WDATA_V_R = crate::BitReader;
#[doc = "Field `APP_RX_END` reader - "]
pub type APP_RX_END_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_slave_wdata(&self) -> APP_SLAVE_WDATA_R {
        APP_SLAVE_WDATA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_cache_mmu_ia(&self) -> APP_CACHE_MMU_IA_R {
        APP_CACHE_MMU_IA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6"]
    #[inline(always)]
    pub fn app_cache_ia(&self) -> APP_CACHE_IA_R {
        APP_CACHE_IA_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bits 7:18"]
    #[inline(always)]
    pub fn app_cache_state(&self) -> APP_CACHE_STATE_R {
        APP_CACHE_STATE_R::new(((self.bits >> 7) & 0x0fff) as u16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn app_wr_bak_to_read(&self) -> APP_WR_BAK_TO_READ_R {
        APP_WR_BAK_TO_READ_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn app_tx_end(&self) -> APP_TX_END_R {
        APP_TX_END_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn app_slave_wr(&self) -> APP_SLAVE_WR_R {
        APP_SLAVE_WR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn app_slave_wdata_v(&self) -> APP_SLAVE_WDATA_V_R {
        APP_SLAVE_WDATA_V_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn app_rx_end(&self) -> APP_RX_END_R {
        APP_RX_END_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_DCACHE_DBUG0")
            .field("app_slave_wdata", &self.app_slave_wdata())
            .field("app_cache_mmu_ia", &self.app_cache_mmu_ia())
            .field("app_cache_ia", &self.app_cache_ia())
            .field("app_cache_state", &self.app_cache_state())
            .field("app_wr_bak_to_read", &self.app_wr_bak_to_read())
            .field("app_tx_end", &self.app_tx_end())
            .field("app_slave_wr", &self.app_slave_wr())
            .field("app_slave_wdata_v", &self.app_slave_wdata_v())
            .field("app_rx_end", &self.app_rx_end())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_slave_wdata(&mut self) -> APP_SLAVE_WDATA_W<'_, APP_DCACHE_DBUG0_SPEC> {
        APP_SLAVE_WDATA_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`app_dcache_dbug0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_dcache_dbug0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_DCACHE_DBUG0_SPEC;
impl crate::RegisterSpec for APP_DCACHE_DBUG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_dcache_dbug0::R`](R) reader structure"]
impl crate::Readable for APP_DCACHE_DBUG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`app_dcache_dbug0::W`](W) writer structure"]
impl crate::Writable for APP_DCACHE_DBUG0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APP_DCACHE_DBUG0 to value 0"]
impl crate::Resettable for APP_DCACHE_DBUG0_SPEC {}
