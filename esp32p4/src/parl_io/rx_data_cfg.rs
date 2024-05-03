#[doc = "Register `RX_DATA_CFG` reader"]
pub type R = crate::R<RX_DATA_CFG_SPEC>;
#[doc = "Register `RX_DATA_CFG` writer"]
pub type W = crate::W<RX_DATA_CFG_SPEC>;
#[doc = "Field `RX_BITLEN` reader - Configures expected byte number of received data."]
pub type RX_BITLEN_R = crate::FieldReader<u32>;
#[doc = "Field `RX_BITLEN` writer - Configures expected byte number of received data."]
pub type RX_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `RX_DATA_ORDER_INV` reader - Set this bit to invert bit order of one byte sent from RX_FIFO to DMA."]
pub type RX_DATA_ORDER_INV_R = crate::BitReader;
#[doc = "Field `RX_DATA_ORDER_INV` writer - Set this bit to invert bit order of one byte sent from RX_FIFO to DMA."]
pub type RX_DATA_ORDER_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_BUS_WID_SEL` reader - Configures the rxd bus width. 3'd0: bus width is 1. 3'd1: bus width is 2. 3'd2: bus width is 4. 3'd3: bus width is 8."]
pub type RX_BUS_WID_SEL_R = crate::FieldReader;
#[doc = "Field `RX_BUS_WID_SEL` writer - Configures the rxd bus width. 3'd0: bus width is 1. 3'd1: bus width is 2. 3'd2: bus width is 4. 3'd3: bus width is 8."]
pub type RX_BUS_WID_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 9:27 - Configures expected byte number of received data."]
    #[inline(always)]
    pub fn rx_bitlen(&self) -> RX_BITLEN_R {
        RX_BITLEN_R::new((self.bits >> 9) & 0x0007_ffff)
    }
    #[doc = "Bit 28 - Set this bit to invert bit order of one byte sent from RX_FIFO to DMA."]
    #[inline(always)]
    pub fn rx_data_order_inv(&self) -> RX_DATA_ORDER_INV_R {
        RX_DATA_ORDER_INV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Configures the rxd bus width. 3'd0: bus width is 1. 3'd1: bus width is 2. 3'd2: bus width is 4. 3'd3: bus width is 8."]
    #[inline(always)]
    pub fn rx_bus_wid_sel(&self) -> RX_BUS_WID_SEL_R {
        RX_BUS_WID_SEL_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_DATA_CFG")
            .field("rx_bitlen", &self.rx_bitlen().bits())
            .field("rx_data_order_inv", &self.rx_data_order_inv().bit())
            .field("rx_bus_wid_sel", &self.rx_bus_wid_sel().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_DATA_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 9:27 - Configures expected byte number of received data."]
    #[inline(always)]
    #[must_use]
    pub fn rx_bitlen(&mut self) -> RX_BITLEN_W<RX_DATA_CFG_SPEC> {
        RX_BITLEN_W::new(self, 9)
    }
    #[doc = "Bit 28 - Set this bit to invert bit order of one byte sent from RX_FIFO to DMA."]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_order_inv(&mut self) -> RX_DATA_ORDER_INV_W<RX_DATA_CFG_SPEC> {
        RX_DATA_ORDER_INV_W::new(self, 28)
    }
    #[doc = "Bits 29:31 - Configures the rxd bus width. 3'd0: bus width is 1. 3'd1: bus width is 2. 3'd2: bus width is 4. 3'd3: bus width is 8."]
    #[inline(always)]
    #[must_use]
    pub fn rx_bus_wid_sel(&mut self) -> RX_BUS_WID_SEL_W<RX_DATA_CFG_SPEC> {
        RX_BUS_WID_SEL_W::new(self, 29)
    }
}
#[doc = "Parallel RX data configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_data_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_data_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_DATA_CFG_SPEC;
impl crate::RegisterSpec for RX_DATA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_data_cfg::R`](R) reader structure"]
impl crate::Readable for RX_DATA_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_data_cfg::W`](W) writer structure"]
impl crate::Writable for RX_DATA_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_DATA_CFG to value 0x6000_0000"]
impl crate::Resettable for RX_DATA_CFG_SPEC {
    const RESET_VALUE: u32 = 0x6000_0000;
}
