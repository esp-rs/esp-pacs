///Register `_1RX_LINK` reader
pub type R = crate::R<_1RX_LINK_SPEC>;
///Register `_1RX_LINK` writer
pub type W = crate::W<_1RX_LINK_SPEC>;
///Field `SLC1_RXLINK_ADDR` reader -
pub type SLC1_RXLINK_ADDR_R = crate::FieldReader<u32>;
///Field `SLC1_RXLINK_ADDR` writer -
pub type SLC1_RXLINK_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
///Field `SLC1_BT_PACKET` reader -
pub type SLC1_BT_PACKET_R = crate::BitReader;
///Field `SLC1_BT_PACKET` writer -
pub type SLC1_BT_PACKET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLC1_RXLINK_STOP` reader -
pub type SLC1_RXLINK_STOP_R = crate::BitReader;
///Field `SLC1_RXLINK_STOP` writer -
pub type SLC1_RXLINK_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLC1_RXLINK_START` reader -
pub type SLC1_RXLINK_START_R = crate::BitReader;
///Field `SLC1_RXLINK_START` writer -
pub type SLC1_RXLINK_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLC1_RXLINK_RESTART` reader -
pub type SLC1_RXLINK_RESTART_R = crate::BitReader;
///Field `SLC1_RXLINK_RESTART` writer -
pub type SLC1_RXLINK_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLC1_RXLINK_PARK` reader -
pub type SLC1_RXLINK_PARK_R = crate::BitReader;
impl R {
    ///Bits 0:19
    #[inline(always)]
    pub fn slc1_rxlink_addr(&self) -> SLC1_RXLINK_ADDR_R {
        SLC1_RXLINK_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    ///Bit 20
    #[inline(always)]
    pub fn slc1_bt_packet(&self) -> SLC1_BT_PACKET_R {
        SLC1_BT_PACKET_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 28
    #[inline(always)]
    pub fn slc1_rxlink_stop(&self) -> SLC1_RXLINK_STOP_R {
        SLC1_RXLINK_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29
    #[inline(always)]
    pub fn slc1_rxlink_start(&self) -> SLC1_RXLINK_START_R {
        SLC1_RXLINK_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30
    #[inline(always)]
    pub fn slc1_rxlink_restart(&self) -> SLC1_RXLINK_RESTART_R {
        SLC1_RXLINK_RESTART_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31
    #[inline(always)]
    pub fn slc1_rxlink_park(&self) -> SLC1_RXLINK_PARK_R {
        SLC1_RXLINK_PARK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_1RX_LINK")
            .field("slc1_rxlink_addr", &self.slc1_rxlink_addr())
            .field("slc1_bt_packet", &self.slc1_bt_packet())
            .field("slc1_rxlink_stop", &self.slc1_rxlink_stop())
            .field("slc1_rxlink_start", &self.slc1_rxlink_start())
            .field("slc1_rxlink_restart", &self.slc1_rxlink_restart())
            .field("slc1_rxlink_park", &self.slc1_rxlink_park())
            .finish()
    }
}
impl W {
    ///Bits 0:19
    #[inline(always)]
    #[must_use]
    pub fn slc1_rxlink_addr(&mut self) -> SLC1_RXLINK_ADDR_W<_1RX_LINK_SPEC> {
        SLC1_RXLINK_ADDR_W::new(self, 0)
    }
    ///Bit 20
    #[inline(always)]
    #[must_use]
    pub fn slc1_bt_packet(&mut self) -> SLC1_BT_PACKET_W<_1RX_LINK_SPEC> {
        SLC1_BT_PACKET_W::new(self, 20)
    }
    ///Bit 28
    #[inline(always)]
    #[must_use]
    pub fn slc1_rxlink_stop(&mut self) -> SLC1_RXLINK_STOP_W<_1RX_LINK_SPEC> {
        SLC1_RXLINK_STOP_W::new(self, 28)
    }
    ///Bit 29
    #[inline(always)]
    #[must_use]
    pub fn slc1_rxlink_start(&mut self) -> SLC1_RXLINK_START_W<_1RX_LINK_SPEC> {
        SLC1_RXLINK_START_W::new(self, 29)
    }
    ///Bit 30
    #[inline(always)]
    #[must_use]
    pub fn slc1_rxlink_restart(&mut self) -> SLC1_RXLINK_RESTART_W<_1RX_LINK_SPEC> {
        SLC1_RXLINK_RESTART_W::new(self, 30)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`_1rx_link::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1rx_link::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct _1RX_LINK_SPEC;
impl crate::RegisterSpec for _1RX_LINK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`_1rx_link::R`](R) reader structure
impl crate::Readable for _1RX_LINK_SPEC {}
///`write(|w| ..)` method takes [`_1rx_link::W`](W) writer structure
impl crate::Writable for _1RX_LINK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets _1RX_LINK to value 0x0010_0000
impl crate::Resettable for _1RX_LINK_SPEC {
    const RESET_VALUE: u32 = 0x0010_0000;
}
