#[doc = "Register `TX_DATA_CFG` reader"]
pub type R = crate::R<TX_DATA_CFG_SPEC>;
#[doc = "Register `TX_DATA_CFG` writer"]
pub type W = crate::W<TX_DATA_CFG_SPEC>;
#[doc = "Field `TX_BITLEN` reader - Configures expected byte number of sent data."]
pub type TX_BITLEN_R = crate::FieldReader<u32>;
#[doc = "Field `TX_BITLEN` writer - Configures expected byte number of sent data."]
pub type TX_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `TX_DATA_ORDER_INV` reader - Write 1 to invert bit order of one byte sent from TX_FIFO to IO data."]
pub type TX_DATA_ORDER_INV_R = crate::BitReader;
#[doc = "Field `TX_DATA_ORDER_INV` writer - Write 1 to invert bit order of one byte sent from TX_FIFO to IO data."]
pub type TX_DATA_ORDER_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_BUS_WID_SEL` reader - Configures the txd bus width. 0: bus width is 1. 1: bus width is 2. 2: bus width is 4. 3: bus width is 8."]
pub type TX_BUS_WID_SEL_R = crate::FieldReader;
#[doc = "Field `TX_BUS_WID_SEL` writer - Configures the txd bus width. 0: bus width is 1. 1: bus width is 2. 2: bus width is 4. 3: bus width is 8."]
pub type TX_BUS_WID_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 9:27 - Configures expected byte number of sent data."]
    #[inline(always)]
    pub fn tx_bitlen(&self) -> TX_BITLEN_R {
        TX_BITLEN_R::new((self.bits >> 9) & 0x0007_ffff)
    }
    #[doc = "Bit 28 - Write 1 to invert bit order of one byte sent from TX_FIFO to IO data."]
    #[inline(always)]
    pub fn tx_data_order_inv(&self) -> TX_DATA_ORDER_INV_R {
        TX_DATA_ORDER_INV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Configures the txd bus width. 0: bus width is 1. 1: bus width is 2. 2: bus width is 4. 3: bus width is 8."]
    #[inline(always)]
    pub fn tx_bus_wid_sel(&self) -> TX_BUS_WID_SEL_R {
        TX_BUS_WID_SEL_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_DATA_CFG")
            .field("tx_bitlen", &self.tx_bitlen())
            .field("tx_data_order_inv", &self.tx_data_order_inv())
            .field("tx_bus_wid_sel", &self.tx_bus_wid_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 9:27 - Configures expected byte number of sent data."]
    #[inline(always)]
    pub fn tx_bitlen(&mut self) -> TX_BITLEN_W<TX_DATA_CFG_SPEC> {
        TX_BITLEN_W::new(self, 9)
    }
    #[doc = "Bit 28 - Write 1 to invert bit order of one byte sent from TX_FIFO to IO data."]
    #[inline(always)]
    pub fn tx_data_order_inv(&mut self) -> TX_DATA_ORDER_INV_W<TX_DATA_CFG_SPEC> {
        TX_DATA_ORDER_INV_W::new(self, 28)
    }
    #[doc = "Bits 29:31 - Configures the txd bus width. 0: bus width is 1. 1: bus width is 2. 2: bus width is 4. 3: bus width is 8."]
    #[inline(always)]
    pub fn tx_bus_wid_sel(&mut self) -> TX_BUS_WID_SEL_W<TX_DATA_CFG_SPEC> {
        TX_BUS_WID_SEL_W::new(self, 29)
    }
}
#[doc = "Parallel TX data configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_data_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_data_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_DATA_CFG_SPEC;
impl crate::RegisterSpec for TX_DATA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_data_cfg::R`](R) reader structure"]
impl crate::Readable for TX_DATA_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_data_cfg::W`](W) writer structure"]
impl crate::Writable for TX_DATA_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_DATA_CFG to value 0x6000_0000"]
impl crate::Resettable for TX_DATA_CFG_SPEC {
    const RESET_VALUE: u32 = 0x6000_0000;
}
