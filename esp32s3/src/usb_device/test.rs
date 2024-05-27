///Register `TEST` reader
pub type R = crate::R<TEST_SPEC>;
///Register `TEST` writer
pub type W = crate::W<TEST_SPEC>;
///Field `ENABLE` reader - Enable test of the USB pad
pub type ENABLE_R = crate::BitReader;
///Field `ENABLE` writer - Enable test of the USB pad
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USB_OE` reader - USB pad oen in test
pub type USB_OE_R = crate::BitReader;
///Field `USB_OE` writer - USB pad oen in test
pub type USB_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_DP` reader - USB D+ tx value in test
pub type TX_DP_R = crate::BitReader;
///Field `TX_DP` writer - USB D+ tx value in test
pub type TX_DP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_DM` reader - USB D- tx value in test
pub type TX_DM_R = crate::BitReader;
///Field `TX_DM` writer - USB D- tx value in test
pub type TX_DM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_RCV` reader - USB differential rx value in test
pub type RX_RCV_R = crate::BitReader;
///Field `RX_DP` reader - USB D+ rx value in test
pub type RX_DP_R = crate::BitReader;
///Field `RX_DM` reader - USB D- rx value in test
pub type RX_DM_R = crate::BitReader;
impl R {
    ///Bit 0 - Enable test of the USB pad
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USB pad oen in test
    #[inline(always)]
    pub fn usb_oe(&self) -> USB_OE_R {
        USB_OE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - USB D+ tx value in test
    #[inline(always)]
    pub fn tx_dp(&self) -> TX_DP_R {
        TX_DP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - USB D- tx value in test
    #[inline(always)]
    pub fn tx_dm(&self) -> TX_DM_R {
        TX_DM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - USB differential rx value in test
    #[inline(always)]
    pub fn rx_rcv(&self) -> RX_RCV_R {
        RX_RCV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - USB D+ rx value in test
    #[inline(always)]
    pub fn rx_dp(&self) -> RX_DP_R {
        RX_DP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - USB D- rx value in test
    #[inline(always)]
    pub fn rx_dm(&self) -> RX_DM_R {
        RX_DM_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST")
            .field("enable", &self.enable())
            .field("usb_oe", &self.usb_oe())
            .field("tx_dp", &self.tx_dp())
            .field("tx_dm", &self.tx_dm())
            .field("rx_rcv", &self.rx_rcv())
            .field("rx_dp", &self.rx_dp())
            .field("rx_dm", &self.rx_dm())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable test of the USB pad
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<TEST_SPEC> {
        ENABLE_W::new(self, 0)
    }
    ///Bit 1 - USB pad oen in test
    #[inline(always)]
    #[must_use]
    pub fn usb_oe(&mut self) -> USB_OE_W<TEST_SPEC> {
        USB_OE_W::new(self, 1)
    }
    ///Bit 2 - USB D+ tx value in test
    #[inline(always)]
    #[must_use]
    pub fn tx_dp(&mut self) -> TX_DP_W<TEST_SPEC> {
        TX_DP_W::new(self, 2)
    }
    ///Bit 3 - USB D- tx value in test
    #[inline(always)]
    #[must_use]
    pub fn tx_dm(&mut self) -> TX_DM_W<TEST_SPEC> {
        TX_DM_W::new(self, 3)
    }
}
/**USB Internal PHY test register

You can [`read`](crate::generic::Reg::read) this register and get [`test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`test::R`](R) reader structure
impl crate::Readable for TEST_SPEC {}
///`write(|w| ..)` method takes [`test::W`](W) writer structure
impl crate::Writable for TEST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TEST to value 0
impl crate::Resettable for TEST_SPEC {
    const RESET_VALUE: u32 = 0;
}
