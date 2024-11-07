#[doc = "Register `APP_CACHE_CTRL` reader"]
pub type R = crate::R<APP_CACHE_CTRL_SPEC>;
#[doc = "Register `APP_CACHE_CTRL` writer"]
pub type W = crate::W<APP_CACHE_CTRL_SPEC>;
#[doc = "Field `APP_CACHE_MODE` reader - "]
pub type APP_CACHE_MODE_R = crate::BitReader;
#[doc = "Field `APP_CACHE_MODE` writer - "]
pub type APP_CACHE_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CACHE_ENABLE` reader - "]
pub type APP_CACHE_ENABLE_R = crate::BitReader;
#[doc = "Field `APP_CACHE_ENABLE` writer - "]
pub type APP_CACHE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CACHE_FLUSH_ENA` reader - "]
pub type APP_CACHE_FLUSH_ENA_R = crate::BitReader;
#[doc = "Field `APP_CACHE_FLUSH_ENA` writer - "]
pub type APP_CACHE_FLUSH_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CACHE_FLUSH_DONE` reader - "]
pub type APP_CACHE_FLUSH_DONE_R = crate::BitReader;
#[doc = "Field `APP_CACHE_LOCK_0_EN` reader - "]
pub type APP_CACHE_LOCK_0_EN_R = crate::BitReader;
#[doc = "Field `APP_CACHE_LOCK_0_EN` writer - "]
pub type APP_CACHE_LOCK_0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CACHE_LOCK_1_EN` reader - "]
pub type APP_CACHE_LOCK_1_EN_R = crate::BitReader;
#[doc = "Field `APP_CACHE_LOCK_1_EN` writer - "]
pub type APP_CACHE_LOCK_1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CACHE_LOCK_2_EN` reader - "]
pub type APP_CACHE_LOCK_2_EN_R = crate::BitReader;
#[doc = "Field `APP_CACHE_LOCK_2_EN` writer - "]
pub type APP_CACHE_LOCK_2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CACHE_LOCK_3_EN` reader - "]
pub type APP_CACHE_LOCK_3_EN_R = crate::BitReader;
#[doc = "Field `APP_CACHE_LOCK_3_EN` writer - "]
pub type APP_CACHE_LOCK_3_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_SINGLE_IRAM_ENA` reader - "]
pub type APP_SINGLE_IRAM_ENA_R = crate::BitReader;
#[doc = "Field `APP_SINGLE_IRAM_ENA` writer - "]
pub type APP_SINGLE_IRAM_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_DRAM_SPLIT` reader - "]
pub type APP_DRAM_SPLIT_R = crate::BitReader;
#[doc = "Field `APP_DRAM_SPLIT` writer - "]
pub type APP_DRAM_SPLIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_AHB_SPI_REQ` reader - "]
pub type APP_AHB_SPI_REQ_R = crate::BitReader;
#[doc = "Field `APP_SLAVE_REQ` reader - "]
pub type APP_SLAVE_REQ_R = crate::BitReader;
#[doc = "Field `APP_DRAM_HL` reader - "]
pub type APP_DRAM_HL_R = crate::BitReader;
#[doc = "Field `APP_DRAM_HL` writer - "]
pub type APP_DRAM_HL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn app_cache_mode(&self) -> APP_CACHE_MODE_R {
        APP_CACHE_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn app_cache_enable(&self) -> APP_CACHE_ENABLE_R {
        APP_CACHE_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn app_cache_flush_ena(&self) -> APP_CACHE_FLUSH_ENA_R {
        APP_CACHE_FLUSH_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn app_cache_flush_done(&self) -> APP_CACHE_FLUSH_DONE_R {
        APP_CACHE_FLUSH_DONE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn app_cache_lock_0_en(&self) -> APP_CACHE_LOCK_0_EN_R {
        APP_CACHE_LOCK_0_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn app_cache_lock_1_en(&self) -> APP_CACHE_LOCK_1_EN_R {
        APP_CACHE_LOCK_1_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_cache_lock_2_en(&self) -> APP_CACHE_LOCK_2_EN_R {
        APP_CACHE_LOCK_2_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn app_cache_lock_3_en(&self) -> APP_CACHE_LOCK_3_EN_R {
        APP_CACHE_LOCK_3_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn app_single_iram_ena(&self) -> APP_SINGLE_IRAM_ENA_R {
        APP_SINGLE_IRAM_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn app_dram_split(&self) -> APP_DRAM_SPLIT_R {
        APP_DRAM_SPLIT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn app_ahb_spi_req(&self) -> APP_AHB_SPI_REQ_R {
        APP_AHB_SPI_REQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn app_slave_req(&self) -> APP_SLAVE_REQ_R {
        APP_SLAVE_REQ_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn app_dram_hl(&self) -> APP_DRAM_HL_R {
        APP_DRAM_HL_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_CACHE_CTRL")
            .field("app_cache_mode", &self.app_cache_mode())
            .field("app_cache_enable", &self.app_cache_enable())
            .field("app_cache_flush_ena", &self.app_cache_flush_ena())
            .field("app_cache_flush_done", &self.app_cache_flush_done())
            .field("app_cache_lock_0_en", &self.app_cache_lock_0_en())
            .field("app_cache_lock_1_en", &self.app_cache_lock_1_en())
            .field("app_cache_lock_2_en", &self.app_cache_lock_2_en())
            .field("app_cache_lock_3_en", &self.app_cache_lock_3_en())
            .field("app_single_iram_ena", &self.app_single_iram_ena())
            .field("app_dram_split", &self.app_dram_split())
            .field("app_ahb_spi_req", &self.app_ahb_spi_req())
            .field("app_slave_req", &self.app_slave_req())
            .field("app_dram_hl", &self.app_dram_hl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn app_cache_mode(&mut self) -> APP_CACHE_MODE_W<APP_CACHE_CTRL_SPEC> {
        APP_CACHE_MODE_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn app_cache_enable(&mut self) -> APP_CACHE_ENABLE_W<APP_CACHE_CTRL_SPEC> {
        APP_CACHE_ENABLE_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn app_cache_flush_ena(&mut self) -> APP_CACHE_FLUSH_ENA_W<APP_CACHE_CTRL_SPEC> {
        APP_CACHE_FLUSH_ENA_W::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn app_cache_lock_0_en(&mut self) -> APP_CACHE_LOCK_0_EN_W<APP_CACHE_CTRL_SPEC> {
        APP_CACHE_LOCK_0_EN_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn app_cache_lock_1_en(&mut self) -> APP_CACHE_LOCK_1_EN_W<APP_CACHE_CTRL_SPEC> {
        APP_CACHE_LOCK_1_EN_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_cache_lock_2_en(&mut self) -> APP_CACHE_LOCK_2_EN_W<APP_CACHE_CTRL_SPEC> {
        APP_CACHE_LOCK_2_EN_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn app_cache_lock_3_en(&mut self) -> APP_CACHE_LOCK_3_EN_W<APP_CACHE_CTRL_SPEC> {
        APP_CACHE_LOCK_3_EN_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn app_single_iram_ena(&mut self) -> APP_SINGLE_IRAM_ENA_W<APP_CACHE_CTRL_SPEC> {
        APP_SINGLE_IRAM_ENA_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn app_dram_split(&mut self) -> APP_DRAM_SPLIT_W<APP_CACHE_CTRL_SPEC> {
        APP_DRAM_SPLIT_W::new(self, 11)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn app_dram_hl(&mut self) -> APP_DRAM_HL_W<APP_CACHE_CTRL_SPEC> {
        APP_DRAM_HL_W::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cache_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cache_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_CACHE_CTRL_SPEC;
impl crate::RegisterSpec for APP_CACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cache_ctrl::R`](R) reader structure"]
impl crate::Readable for APP_CACHE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`app_cache_ctrl::W`](W) writer structure"]
impl crate::Writable for APP_CACHE_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CACHE_CTRL to value 0x10"]
impl crate::Resettable for APP_CACHE_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
