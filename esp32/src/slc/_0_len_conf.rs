#[doc = "Register `_0_LEN_CONF` reader"]
pub type R = crate::R<_0_LEN_CONF_SPEC>;
#[doc = "Register `_0_LEN_CONF` writer"]
pub type W = crate::W<_0_LEN_CONF_SPEC>;
#[doc = "Field `SLC0_LEN_WDATA` writer - "]
pub type SLC0_LEN_WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `SLC0_LEN_WR` writer - "]
pub type SLC0_LEN_WR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_LEN_INC` writer - "]
pub type SLC0_LEN_INC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_LEN_INC_MORE` writer - "]
pub type SLC0_LEN_INC_MORE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_RX_PACKET_LOAD_EN` reader - "]
pub type SLC0_RX_PACKET_LOAD_EN_R = crate::BitReader;
#[doc = "Field `SLC0_RX_PACKET_LOAD_EN` writer - "]
pub type SLC0_RX_PACKET_LOAD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_TX_PACKET_LOAD_EN` reader - "]
pub type SLC0_TX_PACKET_LOAD_EN_R = crate::BitReader;
#[doc = "Field `SLC0_TX_PACKET_LOAD_EN` writer - "]
pub type SLC0_TX_PACKET_LOAD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_RX_GET_USED_DSCR` writer - "]
pub type SLC0_RX_GET_USED_DSCR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_TX_GET_USED_DSCR` writer - "]
pub type SLC0_TX_GET_USED_DSCR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_RX_NEW_PKT_IND` reader - "]
pub type SLC0_RX_NEW_PKT_IND_R = crate::BitReader;
#[doc = "Field `SLC0_TX_NEW_PKT_IND` reader - "]
pub type SLC0_TX_NEW_PKT_IND_R = crate::BitReader;
impl R {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc0_rx_packet_load_en(&self) -> SLC0_RX_PACKET_LOAD_EN_R {
        SLC0_RX_PACKET_LOAD_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc0_tx_packet_load_en(&self) -> SLC0_TX_PACKET_LOAD_EN_R {
        SLC0_TX_PACKET_LOAD_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn slc0_rx_new_pkt_ind(&self) -> SLC0_RX_NEW_PKT_IND_R {
        SLC0_RX_NEW_PKT_IND_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn slc0_tx_new_pkt_ind(&self) -> SLC0_TX_NEW_PKT_IND_R {
        SLC0_TX_NEW_PKT_IND_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0_LEN_CONF")
            .field("slc0_rx_packet_load_en", &self.slc0_rx_packet_load_en())
            .field("slc0_tx_packet_load_en", &self.slc0_tx_packet_load_en())
            .field("slc0_rx_new_pkt_ind", &self.slc0_rx_new_pkt_ind())
            .field("slc0_tx_new_pkt_ind", &self.slc0_tx_new_pkt_ind())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_len_wdata(&mut self) -> SLC0_LEN_WDATA_W<_0_LEN_CONF_SPEC> {
        SLC0_LEN_WDATA_W::new(self, 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_len_wr(&mut self) -> SLC0_LEN_WR_W<_0_LEN_CONF_SPEC> {
        SLC0_LEN_WR_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_len_inc(&mut self) -> SLC0_LEN_INC_W<_0_LEN_CONF_SPEC> {
        SLC0_LEN_INC_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_len_inc_more(&mut self) -> SLC0_LEN_INC_MORE_W<_0_LEN_CONF_SPEC> {
        SLC0_LEN_INC_MORE_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_packet_load_en(&mut self) -> SLC0_RX_PACKET_LOAD_EN_W<_0_LEN_CONF_SPEC> {
        SLC0_RX_PACKET_LOAD_EN_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tx_packet_load_en(&mut self) -> SLC0_TX_PACKET_LOAD_EN_W<_0_LEN_CONF_SPEC> {
        SLC0_TX_PACKET_LOAD_EN_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_get_used_dscr(&mut self) -> SLC0_RX_GET_USED_DSCR_W<_0_LEN_CONF_SPEC> {
        SLC0_RX_GET_USED_DSCR_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tx_get_used_dscr(&mut self) -> SLC0_TX_GET_USED_DSCR_W<_0_LEN_CONF_SPEC> {
        SLC0_TX_GET_USED_DSCR_W::new(self, 26)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_len_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_len_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _0_LEN_CONF_SPEC;
impl crate::RegisterSpec for _0_LEN_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_0_len_conf::R`](R) reader structure"]
impl crate::Readable for _0_LEN_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_0_len_conf::W`](W) writer structure"]
impl crate::Writable for _0_LEN_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets _0_LEN_CONF to value 0"]
impl crate::Resettable for _0_LEN_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
