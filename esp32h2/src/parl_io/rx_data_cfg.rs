#[doc = "Register `RX_DATA_CFG` reader"]
pub struct R(crate::R<RX_DATA_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_DATA_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_DATA_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_DATA_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_DATA_CFG` writer"]
pub struct W(crate::W<RX_DATA_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_DATA_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RX_DATA_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_DATA_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_BITLEN` reader - Configures expected byte number of received data."]
pub type RX_BITLEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RX_BITLEN` writer - Configures expected byte number of received data."]
pub type RX_BITLEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RX_DATA_CFG_SPEC, u32, u32, 19, O>;
#[doc = "Field `RX_DATA_ORDER_INV` reader - Set this bit to invert bit order of one byte sent from RX_FIFO to DMA."]
pub type RX_DATA_ORDER_INV_R = crate::BitReader<bool>;
#[doc = "Field `RX_DATA_ORDER_INV` writer - Set this bit to invert bit order of one byte sent from RX_FIFO to DMA."]
pub type RX_DATA_ORDER_INV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RX_DATA_CFG_SPEC, bool, O>;
#[doc = "Field `RX_BUS_WID_SEL` reader - Configures the rxd bus width. 3'd0: bus width is 1. 3'd1: bus width is 2. 3'd2: bus width is 4. 3'd3: bus width is 8."]
pub type RX_BUS_WID_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_BUS_WID_SEL` writer - Configures the rxd bus width. 3'd0: bus width is 1. 3'd1: bus width is 2. 3'd2: bus width is 4. 3'd3: bus width is 8."]
pub type RX_BUS_WID_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RX_DATA_CFG_SPEC, u8, u8, 3, O>;
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
impl W {
    #[doc = "Bits 9:27 - Configures expected byte number of received data."]
    #[inline(always)]
    #[must_use]
    pub fn rx_bitlen(&mut self) -> RX_BITLEN_W<9> {
        RX_BITLEN_W::new(self)
    }
    #[doc = "Bit 28 - Set this bit to invert bit order of one byte sent from RX_FIFO to DMA."]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_order_inv(&mut self) -> RX_DATA_ORDER_INV_W<28> {
        RX_DATA_ORDER_INV_W::new(self)
    }
    #[doc = "Bits 29:31 - Configures the rxd bus width. 3'd0: bus width is 1. 3'd1: bus width is 2. 3'd2: bus width is 4. 3'd3: bus width is 8."]
    #[inline(always)]
    #[must_use]
    pub fn rx_bus_wid_sel(&mut self) -> RX_BUS_WID_SEL_W<29> {
        RX_BUS_WID_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parallel RX data configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_data_cfg](index.html) module"]
pub struct RX_DATA_CFG_SPEC;
impl crate::RegisterSpec for RX_DATA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_data_cfg::R](R) reader structure"]
impl crate::Readable for RX_DATA_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_data_cfg::W](W) writer structure"]
impl crate::Writable for RX_DATA_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_DATA_CFG to value 0x6000_0000"]
impl crate::Resettable for RX_DATA_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x6000_0000;
}
