#[doc = "Register `PRO_DCACHE_DBUG0` reader"]
pub type R = crate::R<PRO_DCACHE_DBUG0_SPEC>;
#[doc = "Register `PRO_DCACHE_DBUG0` writer"]
pub type W = crate::W<PRO_DCACHE_DBUG0_SPEC>;
#[doc = "Field `PRO_SLAVE_WDATA` reader - "]
pub type PRO_SLAVE_WDATA_R = crate::BitReader;
#[doc = "Field `PRO_SLAVE_WDATA` writer - "]
pub type PRO_SLAVE_WDATA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CACHE_MMU_IA` reader - "]
pub type PRO_CACHE_MMU_IA_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_IA` reader - "]
pub type PRO_CACHE_IA_R = crate::FieldReader;
#[doc = "Field `PRO_CACHE_STATE` reader - "]
pub type PRO_CACHE_STATE_R = crate::FieldReader<u16>;
#[doc = "Field `PRO_WR_BAK_TO_READ` reader - "]
pub type PRO_WR_BAK_TO_READ_R = crate::BitReader;
#[doc = "Field `PRO_TX_END` reader - "]
pub type PRO_TX_END_R = crate::BitReader;
#[doc = "Field `PRO_SLAVE_WR` reader - "]
pub type PRO_SLAVE_WR_R = crate::BitReader;
#[doc = "Field `PRO_SLAVE_WDATA_V` reader - "]
pub type PRO_SLAVE_WDATA_V_R = crate::BitReader;
#[doc = "Field `PRO_RX_END` reader - "]
pub type PRO_RX_END_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_slave_wdata(&self) -> PRO_SLAVE_WDATA_R {
        PRO_SLAVE_WDATA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_cache_mmu_ia(&self) -> PRO_CACHE_MMU_IA_R {
        PRO_CACHE_MMU_IA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6"]
    #[inline(always)]
    pub fn pro_cache_ia(&self) -> PRO_CACHE_IA_R {
        PRO_CACHE_IA_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bits 7:18"]
    #[inline(always)]
    pub fn pro_cache_state(&self) -> PRO_CACHE_STATE_R {
        PRO_CACHE_STATE_R::new(((self.bits >> 7) & 0x0fff) as u16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pro_wr_bak_to_read(&self) -> PRO_WR_BAK_TO_READ_R {
        PRO_WR_BAK_TO_READ_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pro_tx_end(&self) -> PRO_TX_END_R {
        PRO_TX_END_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pro_slave_wr(&self) -> PRO_SLAVE_WR_R {
        PRO_SLAVE_WR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pro_slave_wdata_v(&self) -> PRO_SLAVE_WDATA_V_R {
        PRO_SLAVE_WDATA_V_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pro_rx_end(&self) -> PRO_RX_END_R {
        PRO_RX_END_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DCACHE_DBUG0")
            .field("pro_slave_wdata", &self.pro_slave_wdata())
            .field("pro_cache_mmu_ia", &self.pro_cache_mmu_ia())
            .field("pro_cache_ia", &self.pro_cache_ia())
            .field("pro_cache_state", &self.pro_cache_state())
            .field("pro_wr_bak_to_read", &self.pro_wr_bak_to_read())
            .field("pro_tx_end", &self.pro_tx_end())
            .field("pro_slave_wr", &self.pro_slave_wr())
            .field("pro_slave_wdata_v", &self.pro_slave_wdata_v())
            .field("pro_rx_end", &self.pro_rx_end())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_slave_wdata(&mut self) -> PRO_SLAVE_WDATA_W<'_, PRO_DCACHE_DBUG0_SPEC> {
        PRO_SLAVE_WDATA_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_dbug0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dcache_dbug0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DCACHE_DBUG0_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_DBUG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dcache_dbug0::R`](R) reader structure"]
impl crate::Readable for PRO_DCACHE_DBUG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_dcache_dbug0::W`](W) writer structure"]
impl crate::Writable for PRO_DCACHE_DBUG0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRO_DCACHE_DBUG0 to value 0"]
impl crate::Resettable for PRO_DCACHE_DBUG0_SPEC {}
