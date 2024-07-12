#[doc = "Register `CONF1` reader"]
pub type R = crate::R<CONF1_SPEC>;
#[doc = "Register `CONF1` writer"]
pub type W = crate::W<CONF1_SPEC>;
#[doc = "Field `SLC0_CHECK_OWNER` reader - "]
pub type SLC0_CHECK_OWNER_R = crate::BitReader;
#[doc = "Field `SLC0_CHECK_OWNER` writer - "]
pub type SLC0_CHECK_OWNER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_TX_CHECK_SUM_EN` reader - "]
pub type SLC0_TX_CHECK_SUM_EN_R = crate::BitReader;
#[doc = "Field `SLC0_TX_CHECK_SUM_EN` writer - "]
pub type SLC0_TX_CHECK_SUM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_RX_CHECK_SUM_EN` reader - "]
pub type SLC0_RX_CHECK_SUM_EN_R = crate::BitReader;
#[doc = "Field `SLC0_RX_CHECK_SUM_EN` writer - "]
pub type SLC0_RX_CHECK_SUM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_HOLD_EN` reader - "]
pub type CMD_HOLD_EN_R = crate::BitReader;
#[doc = "Field `CMD_HOLD_EN` writer - "]
pub type CMD_HOLD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_LEN_AUTO_CLR` reader - "]
pub type SLC0_LEN_AUTO_CLR_R = crate::BitReader;
#[doc = "Field `SLC0_LEN_AUTO_CLR` writer - "]
pub type SLC0_LEN_AUTO_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_TX_STITCH_EN` reader - "]
pub type SLC0_TX_STITCH_EN_R = crate::BitReader;
#[doc = "Field `SLC0_TX_STITCH_EN` writer - "]
pub type SLC0_TX_STITCH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_RX_STITCH_EN` reader - "]
pub type SLC0_RX_STITCH_EN_R = crate::BitReader;
#[doc = "Field `SLC0_RX_STITCH_EN` writer - "]
pub type SLC0_RX_STITCH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_CHECK_OWNER` reader - "]
pub type SLC1_CHECK_OWNER_R = crate::BitReader;
#[doc = "Field `SLC1_CHECK_OWNER` writer - "]
pub type SLC1_CHECK_OWNER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_TX_CHECK_SUM_EN` reader - "]
pub type SLC1_TX_CHECK_SUM_EN_R = crate::BitReader;
#[doc = "Field `SLC1_TX_CHECK_SUM_EN` writer - "]
pub type SLC1_TX_CHECK_SUM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_RX_CHECK_SUM_EN` reader - "]
pub type SLC1_RX_CHECK_SUM_EN_R = crate::BitReader;
#[doc = "Field `SLC1_RX_CHECK_SUM_EN` writer - "]
pub type SLC1_RX_CHECK_SUM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_INT_LEVEL_SEL` reader - "]
pub type HOST_INT_LEVEL_SEL_R = crate::BitReader;
#[doc = "Field `HOST_INT_LEVEL_SEL` writer - "]
pub type HOST_INT_LEVEL_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_TX_STITCH_EN` reader - "]
pub type SLC1_TX_STITCH_EN_R = crate::BitReader;
#[doc = "Field `SLC1_TX_STITCH_EN` writer - "]
pub type SLC1_TX_STITCH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_RX_STITCH_EN` reader - "]
pub type SLC1_RX_STITCH_EN_R = crate::BitReader;
#[doc = "Field `SLC1_RX_STITCH_EN` writer - "]
pub type SLC1_RX_STITCH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - "]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - "]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_check_owner(&self) -> SLC0_CHECK_OWNER_R {
        SLC0_CHECK_OWNER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_tx_check_sum_en(&self) -> SLC0_TX_CHECK_SUM_EN_R {
        SLC0_TX_CHECK_SUM_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc0_rx_check_sum_en(&self) -> SLC0_RX_CHECK_SUM_EN_R {
        SLC0_RX_CHECK_SUM_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cmd_hold_en(&self) -> CMD_HOLD_EN_R {
        CMD_HOLD_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc0_len_auto_clr(&self) -> SLC0_LEN_AUTO_CLR_R {
        SLC0_LEN_AUTO_CLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slc0_tx_stitch_en(&self) -> SLC0_TX_STITCH_EN_R {
        SLC0_TX_STITCH_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slc0_rx_stitch_en(&self) -> SLC0_RX_STITCH_EN_R {
        SLC0_RX_STITCH_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_check_owner(&self) -> SLC1_CHECK_OWNER_R {
        SLC1_CHECK_OWNER_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_tx_check_sum_en(&self) -> SLC1_TX_CHECK_SUM_EN_R {
        SLC1_TX_CHECK_SUM_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc1_rx_check_sum_en(&self) -> SLC1_RX_CHECK_SUM_EN_R {
        SLC1_RX_CHECK_SUM_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn host_int_level_sel(&self) -> HOST_INT_LEVEL_SEL_R {
        HOST_INT_LEVEL_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_tx_stitch_en(&self) -> SLC1_TX_STITCH_EN_R {
        SLC1_TX_STITCH_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc1_rx_stitch_en(&self) -> SLC1_RX_STITCH_EN_R {
        SLC1_RX_STITCH_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF1")
            .field("slc0_check_owner", &self.slc0_check_owner())
            .field("slc0_tx_check_sum_en", &self.slc0_tx_check_sum_en())
            .field("slc0_rx_check_sum_en", &self.slc0_rx_check_sum_en())
            .field("cmd_hold_en", &self.cmd_hold_en())
            .field("slc0_len_auto_clr", &self.slc0_len_auto_clr())
            .field("slc0_tx_stitch_en", &self.slc0_tx_stitch_en())
            .field("slc0_rx_stitch_en", &self.slc0_rx_stitch_en())
            .field("slc1_check_owner", &self.slc1_check_owner())
            .field("slc1_tx_check_sum_en", &self.slc1_tx_check_sum_en())
            .field("slc1_rx_check_sum_en", &self.slc1_rx_check_sum_en())
            .field("host_int_level_sel", &self.host_int_level_sel())
            .field("slc1_tx_stitch_en", &self.slc1_tx_stitch_en())
            .field("slc1_rx_stitch_en", &self.slc1_rx_stitch_en())
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_check_owner(&mut self) -> SLC0_CHECK_OWNER_W<CONF1_SPEC> {
        SLC0_CHECK_OWNER_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tx_check_sum_en(&mut self) -> SLC0_TX_CHECK_SUM_EN_W<CONF1_SPEC> {
        SLC0_TX_CHECK_SUM_EN_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_check_sum_en(&mut self) -> SLC0_RX_CHECK_SUM_EN_W<CONF1_SPEC> {
        SLC0_RX_CHECK_SUM_EN_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_hold_en(&mut self) -> CMD_HOLD_EN_W<CONF1_SPEC> {
        CMD_HOLD_EN_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_len_auto_clr(&mut self) -> SLC0_LEN_AUTO_CLR_W<CONF1_SPEC> {
        SLC0_LEN_AUTO_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tx_stitch_en(&mut self) -> SLC0_TX_STITCH_EN_W<CONF1_SPEC> {
        SLC0_TX_STITCH_EN_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_stitch_en(&mut self) -> SLC0_RX_STITCH_EN_W<CONF1_SPEC> {
        SLC0_RX_STITCH_EN_W::new(self, 6)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_check_owner(&mut self) -> SLC1_CHECK_OWNER_W<CONF1_SPEC> {
        SLC1_CHECK_OWNER_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tx_check_sum_en(&mut self) -> SLC1_TX_CHECK_SUM_EN_W<CONF1_SPEC> {
        SLC1_TX_CHECK_SUM_EN_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_check_sum_en(&mut self) -> SLC1_RX_CHECK_SUM_EN_W<CONF1_SPEC> {
        SLC1_RX_CHECK_SUM_EN_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn host_int_level_sel(&mut self) -> HOST_INT_LEVEL_SEL_W<CONF1_SPEC> {
        HOST_INT_LEVEL_SEL_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tx_stitch_en(&mut self) -> SLC1_TX_STITCH_EN_W<CONF1_SPEC> {
        SLC1_TX_STITCH_EN_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_stitch_en(&mut self) -> SLC1_RX_STITCH_EN_W<CONF1_SPEC> {
        SLC1_RX_STITCH_EN_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CONF1_SPEC> {
        CLK_EN_W::new(self, 22)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf1::R`](R) reader structure"]
impl crate::Readable for CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf1::W`](W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF1 to value 0x0030_0078"]
impl crate::Resettable for CONF1_SPEC {
    const RESET_VALUE: u32 = 0x0030_0078;
}
