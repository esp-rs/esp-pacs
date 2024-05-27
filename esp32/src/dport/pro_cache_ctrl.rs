///Register `PRO_CACHE_CTRL` reader
pub type R = crate::R<PRO_CACHE_CTRL_SPEC>;
///Register `PRO_CACHE_CTRL` writer
pub type W = crate::W<PRO_CACHE_CTRL_SPEC>;
///Field `PRO_CACHE_MODE` reader -
pub type PRO_CACHE_MODE_R = crate::BitReader;
///Field `PRO_CACHE_MODE` writer -
pub type PRO_CACHE_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_CACHE_ENABLE` reader -
pub type PRO_CACHE_ENABLE_R = crate::BitReader;
///Field `PRO_CACHE_ENABLE` writer -
pub type PRO_CACHE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_CACHE_FLUSH_ENA` reader -
pub type PRO_CACHE_FLUSH_ENA_R = crate::BitReader;
///Field `PRO_CACHE_FLUSH_ENA` writer -
pub type PRO_CACHE_FLUSH_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_CACHE_FLUSH_DONE` reader -
pub type PRO_CACHE_FLUSH_DONE_R = crate::BitReader;
///Field `PRO_CACHE_LOCK_0_EN` reader -
pub type PRO_CACHE_LOCK_0_EN_R = crate::BitReader;
///Field `PRO_CACHE_LOCK_0_EN` writer -
pub type PRO_CACHE_LOCK_0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_CACHE_LOCK_1_EN` reader -
pub type PRO_CACHE_LOCK_1_EN_R = crate::BitReader;
///Field `PRO_CACHE_LOCK_1_EN` writer -
pub type PRO_CACHE_LOCK_1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_CACHE_LOCK_2_EN` reader -
pub type PRO_CACHE_LOCK_2_EN_R = crate::BitReader;
///Field `PRO_CACHE_LOCK_2_EN` writer -
pub type PRO_CACHE_LOCK_2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_CACHE_LOCK_3_EN` reader -
pub type PRO_CACHE_LOCK_3_EN_R = crate::BitReader;
///Field `PRO_CACHE_LOCK_3_EN` writer -
pub type PRO_CACHE_LOCK_3_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_SINGLE_IRAM_ENA` reader -
pub type PRO_SINGLE_IRAM_ENA_R = crate::BitReader;
///Field `PRO_SINGLE_IRAM_ENA` writer -
pub type PRO_SINGLE_IRAM_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_DRAM_SPLIT` reader -
pub type PRO_DRAM_SPLIT_R = crate::BitReader;
///Field `PRO_DRAM_SPLIT` writer -
pub type PRO_DRAM_SPLIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_AHB_SPI_REQ` reader -
pub type PRO_AHB_SPI_REQ_R = crate::BitReader;
///Field `PRO_SLAVE_REQ` reader -
pub type PRO_SLAVE_REQ_R = crate::BitReader;
///Field `AHB_SPI_REQ` reader -
pub type AHB_SPI_REQ_R = crate::BitReader;
///Field `SLAVE_REQ` reader -
pub type SLAVE_REQ_R = crate::BitReader;
///Field `PRO_DRAM_HL` reader -
pub type PRO_DRAM_HL_R = crate::BitReader;
///Field `PRO_DRAM_HL` writer -
pub type PRO_DRAM_HL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2
    #[inline(always)]
    pub fn pro_cache_mode(&self) -> PRO_CACHE_MODE_R {
        PRO_CACHE_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3
    #[inline(always)]
    pub fn pro_cache_enable(&self) -> PRO_CACHE_ENABLE_R {
        PRO_CACHE_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4
    #[inline(always)]
    pub fn pro_cache_flush_ena(&self) -> PRO_CACHE_FLUSH_ENA_R {
        PRO_CACHE_FLUSH_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5
    #[inline(always)]
    pub fn pro_cache_flush_done(&self) -> PRO_CACHE_FLUSH_DONE_R {
        PRO_CACHE_FLUSH_DONE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6
    #[inline(always)]
    pub fn pro_cache_lock_0_en(&self) -> PRO_CACHE_LOCK_0_EN_R {
        PRO_CACHE_LOCK_0_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7
    #[inline(always)]
    pub fn pro_cache_lock_1_en(&self) -> PRO_CACHE_LOCK_1_EN_R {
        PRO_CACHE_LOCK_1_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8
    #[inline(always)]
    pub fn pro_cache_lock_2_en(&self) -> PRO_CACHE_LOCK_2_EN_R {
        PRO_CACHE_LOCK_2_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9
    #[inline(always)]
    pub fn pro_cache_lock_3_en(&self) -> PRO_CACHE_LOCK_3_EN_R {
        PRO_CACHE_LOCK_3_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10
    #[inline(always)]
    pub fn pro_single_iram_ena(&self) -> PRO_SINGLE_IRAM_ENA_R {
        PRO_SINGLE_IRAM_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11
    #[inline(always)]
    pub fn pro_dram_split(&self) -> PRO_DRAM_SPLIT_R {
        PRO_DRAM_SPLIT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12
    #[inline(always)]
    pub fn pro_ahb_spi_req(&self) -> PRO_AHB_SPI_REQ_R {
        PRO_AHB_SPI_REQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13
    #[inline(always)]
    pub fn pro_slave_req(&self) -> PRO_SLAVE_REQ_R {
        PRO_SLAVE_REQ_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14
    #[inline(always)]
    pub fn ahb_spi_req(&self) -> AHB_SPI_REQ_R {
        AHB_SPI_REQ_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15
    #[inline(always)]
    pub fn slave_req(&self) -> SLAVE_REQ_R {
        SLAVE_REQ_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16
    #[inline(always)]
    pub fn pro_dram_hl(&self) -> PRO_DRAM_HL_R {
        PRO_DRAM_HL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CACHE_CTRL")
            .field("pro_cache_mode", &self.pro_cache_mode())
            .field("pro_cache_enable", &self.pro_cache_enable())
            .field("pro_cache_flush_ena", &self.pro_cache_flush_ena())
            .field("pro_cache_flush_done", &self.pro_cache_flush_done())
            .field("pro_cache_lock_0_en", &self.pro_cache_lock_0_en())
            .field("pro_cache_lock_1_en", &self.pro_cache_lock_1_en())
            .field("pro_cache_lock_2_en", &self.pro_cache_lock_2_en())
            .field("pro_cache_lock_3_en", &self.pro_cache_lock_3_en())
            .field("pro_single_iram_ena", &self.pro_single_iram_ena())
            .field("pro_dram_split", &self.pro_dram_split())
            .field("pro_ahb_spi_req", &self.pro_ahb_spi_req())
            .field("pro_slave_req", &self.pro_slave_req())
            .field("ahb_spi_req", &self.ahb_spi_req())
            .field("slave_req", &self.slave_req())
            .field("pro_dram_hl", &self.pro_dram_hl())
            .finish()
    }
}
impl W {
    ///Bit 2
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_mode(&mut self) -> PRO_CACHE_MODE_W<PRO_CACHE_CTRL_SPEC> {
        PRO_CACHE_MODE_W::new(self, 2)
    }
    ///Bit 3
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_enable(&mut self) -> PRO_CACHE_ENABLE_W<PRO_CACHE_CTRL_SPEC> {
        PRO_CACHE_ENABLE_W::new(self, 3)
    }
    ///Bit 4
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_flush_ena(&mut self) -> PRO_CACHE_FLUSH_ENA_W<PRO_CACHE_CTRL_SPEC> {
        PRO_CACHE_FLUSH_ENA_W::new(self, 4)
    }
    ///Bit 6
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_lock_0_en(&mut self) -> PRO_CACHE_LOCK_0_EN_W<PRO_CACHE_CTRL_SPEC> {
        PRO_CACHE_LOCK_0_EN_W::new(self, 6)
    }
    ///Bit 7
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_lock_1_en(&mut self) -> PRO_CACHE_LOCK_1_EN_W<PRO_CACHE_CTRL_SPEC> {
        PRO_CACHE_LOCK_1_EN_W::new(self, 7)
    }
    ///Bit 8
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_lock_2_en(&mut self) -> PRO_CACHE_LOCK_2_EN_W<PRO_CACHE_CTRL_SPEC> {
        PRO_CACHE_LOCK_2_EN_W::new(self, 8)
    }
    ///Bit 9
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_lock_3_en(&mut self) -> PRO_CACHE_LOCK_3_EN_W<PRO_CACHE_CTRL_SPEC> {
        PRO_CACHE_LOCK_3_EN_W::new(self, 9)
    }
    ///Bit 10
    #[inline(always)]
    #[must_use]
    pub fn pro_single_iram_ena(&mut self) -> PRO_SINGLE_IRAM_ENA_W<PRO_CACHE_CTRL_SPEC> {
        PRO_SINGLE_IRAM_ENA_W::new(self, 10)
    }
    ///Bit 11
    #[inline(always)]
    #[must_use]
    pub fn pro_dram_split(&mut self) -> PRO_DRAM_SPLIT_W<PRO_CACHE_CTRL_SPEC> {
        PRO_DRAM_SPLIT_W::new(self, 11)
    }
    ///Bit 16
    #[inline(always)]
    #[must_use]
    pub fn pro_dram_hl(&mut self) -> PRO_DRAM_HL_W<PRO_CACHE_CTRL_SPEC> {
        PRO_DRAM_HL_W::new(self, 16)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`pro_cache_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cache_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PRO_CACHE_CTRL_SPEC;
impl crate::RegisterSpec for PRO_CACHE_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pro_cache_ctrl::R`](R) reader structure
impl crate::Readable for PRO_CACHE_CTRL_SPEC {}
///`write(|w| ..)` method takes [`pro_cache_ctrl::W`](W) writer structure
impl crate::Writable for PRO_CACHE_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PRO_CACHE_CTRL to value 0x10
impl crate::Resettable for PRO_CACHE_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
