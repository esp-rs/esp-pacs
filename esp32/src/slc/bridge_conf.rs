///Register `BRIDGE_CONF` reader
pub type R = crate::R<BRIDGE_CONF_SPEC>;
///Register `BRIDGE_CONF` writer
pub type W = crate::W<BRIDGE_CONF_SPEC>;
///Field `TXEOF_ENA` reader -
pub type TXEOF_ENA_R = crate::FieldReader;
///Field `TXEOF_ENA` writer -
pub type TXEOF_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `FIFO_MAP_ENA` reader -
pub type FIFO_MAP_ENA_R = crate::FieldReader;
///Field `FIFO_MAP_ENA` writer -
pub type FIFO_MAP_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SLC0_TX_DUMMY_MODE` reader -
pub type SLC0_TX_DUMMY_MODE_R = crate::BitReader;
///Field `SLC0_TX_DUMMY_MODE` writer -
pub type SLC0_TX_DUMMY_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HDA_MAP_128K` reader -
pub type HDA_MAP_128K_R = crate::BitReader;
///Field `HDA_MAP_128K` writer -
pub type HDA_MAP_128K_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLC1_TX_DUMMY_MODE` reader -
pub type SLC1_TX_DUMMY_MODE_R = crate::BitReader;
///Field `SLC1_TX_DUMMY_MODE` writer -
pub type SLC1_TX_DUMMY_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_PUSH_IDLE_NUM` reader -
pub type TX_PUSH_IDLE_NUM_R = crate::FieldReader<u16>;
///Field `TX_PUSH_IDLE_NUM` writer -
pub type TX_PUSH_IDLE_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:5
    #[inline(always)]
    pub fn txeof_ena(&self) -> TXEOF_ENA_R {
        TXEOF_ENA_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:11
    #[inline(always)]
    pub fn fifo_map_ena(&self) -> FIFO_MAP_ENA_R {
        FIFO_MAP_ENA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12
    #[inline(always)]
    pub fn slc0_tx_dummy_mode(&self) -> SLC0_TX_DUMMY_MODE_R {
        SLC0_TX_DUMMY_MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13
    #[inline(always)]
    pub fn hda_map_128k(&self) -> HDA_MAP_128K_R {
        HDA_MAP_128K_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14
    #[inline(always)]
    pub fn slc1_tx_dummy_mode(&self) -> SLC1_TX_DUMMY_MODE_R {
        SLC1_TX_DUMMY_MODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn tx_push_idle_num(&self) -> TX_PUSH_IDLE_NUM_R {
        TX_PUSH_IDLE_NUM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BRIDGE_CONF")
            .field("txeof_ena", &self.txeof_ena())
            .field("fifo_map_ena", &self.fifo_map_ena())
            .field("slc0_tx_dummy_mode", &self.slc0_tx_dummy_mode())
            .field("hda_map_128k", &self.hda_map_128k())
            .field("slc1_tx_dummy_mode", &self.slc1_tx_dummy_mode())
            .field("tx_push_idle_num", &self.tx_push_idle_num())
            .finish()
    }
}
impl W {
    ///Bits 0:5
    #[inline(always)]
    #[must_use]
    pub fn txeof_ena(&mut self) -> TXEOF_ENA_W<BRIDGE_CONF_SPEC> {
        TXEOF_ENA_W::new(self, 0)
    }
    ///Bits 8:11
    #[inline(always)]
    #[must_use]
    pub fn fifo_map_ena(&mut self) -> FIFO_MAP_ENA_W<BRIDGE_CONF_SPEC> {
        FIFO_MAP_ENA_W::new(self, 8)
    }
    ///Bit 12
    #[inline(always)]
    #[must_use]
    pub fn slc0_tx_dummy_mode(&mut self) -> SLC0_TX_DUMMY_MODE_W<BRIDGE_CONF_SPEC> {
        SLC0_TX_DUMMY_MODE_W::new(self, 12)
    }
    ///Bit 13
    #[inline(always)]
    #[must_use]
    pub fn hda_map_128k(&mut self) -> HDA_MAP_128K_W<BRIDGE_CONF_SPEC> {
        HDA_MAP_128K_W::new(self, 13)
    }
    ///Bit 14
    #[inline(always)]
    #[must_use]
    pub fn slc1_tx_dummy_mode(&mut self) -> SLC1_TX_DUMMY_MODE_W<BRIDGE_CONF_SPEC> {
        SLC1_TX_DUMMY_MODE_W::new(self, 14)
    }
    ///Bits 16:31
    #[inline(always)]
    #[must_use]
    pub fn tx_push_idle_num(&mut self) -> TX_PUSH_IDLE_NUM_W<BRIDGE_CONF_SPEC> {
        TX_PUSH_IDLE_NUM_W::new(self, 16)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`bridge_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bridge_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BRIDGE_CONF_SPEC;
impl crate::RegisterSpec for BRIDGE_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`bridge_conf::R`](R) reader structure
impl crate::Readable for BRIDGE_CONF_SPEC {}
///`write(|w| ..)` method takes [`bridge_conf::W`](W) writer structure
impl crate::Writable for BRIDGE_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BRIDGE_CONF to value 0x000a_7720
impl crate::Resettable for BRIDGE_CONF_SPEC {
    const RESET_VALUE: u32 = 0x000a_7720;
}
