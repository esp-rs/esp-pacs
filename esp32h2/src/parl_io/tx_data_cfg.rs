#[doc = "Register `TX_DATA_CFG` reader"]
pub struct R(crate::R<TX_DATA_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_DATA_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_DATA_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_DATA_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_DATA_CFG` writer"]
pub struct W(crate::W<TX_DATA_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_DATA_CFG_SPEC>;
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
impl From<crate::W<TX_DATA_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_DATA_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BITLEN` reader - Configures expected byte number of sent data."]
pub type TX_BITLEN_R = crate::FieldReader<u32>;
#[doc = "Field `TX_BITLEN` writer - Configures expected byte number of sent data."]
pub type TX_BITLEN_W<'a, const O: u8> = crate::FieldWriter<'a, TX_DATA_CFG_SPEC, 19, O, u32>;
#[doc = "Field `TX_DATA_ORDER_INV` reader - Set this bit to invert bit order of one byte sent from TX_FIFO to IO data."]
pub type TX_DATA_ORDER_INV_R = crate::BitReader;
#[doc = "Field `TX_DATA_ORDER_INV` writer - Set this bit to invert bit order of one byte sent from TX_FIFO to IO data."]
pub type TX_DATA_ORDER_INV_W<'a, const O: u8> = crate::BitWriter<'a, TX_DATA_CFG_SPEC, O>;
#[doc = "Field `TX_BUS_WID_SEL` reader - Configures the txd bus width. 3'd0: bus width is 1. 3'd1: bus width is 2. 3'd2: bus width is 4. 3'd3: bus width is 8."]
pub type TX_BUS_WID_SEL_R = crate::FieldReader;
#[doc = "Field `TX_BUS_WID_SEL` writer - Configures the txd bus width. 3'd0: bus width is 1. 3'd1: bus width is 2. 3'd2: bus width is 4. 3'd3: bus width is 8."]
pub type TX_BUS_WID_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, TX_DATA_CFG_SPEC, 3, O>;
impl R {
    #[doc = "Bits 9:27 - Configures expected byte number of sent data."]
    #[inline(always)]
    pub fn tx_bitlen(&self) -> TX_BITLEN_R {
        TX_BITLEN_R::new((self.bits >> 9) & 0x0007_ffff)
    }
    #[doc = "Bit 28 - Set this bit to invert bit order of one byte sent from TX_FIFO to IO data."]
    #[inline(always)]
    pub fn tx_data_order_inv(&self) -> TX_DATA_ORDER_INV_R {
        TX_DATA_ORDER_INV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Configures the txd bus width. 3'd0: bus width is 1. 3'd1: bus width is 2. 3'd2: bus width is 4. 3'd3: bus width is 8."]
    #[inline(always)]
    pub fn tx_bus_wid_sel(&self) -> TX_BUS_WID_SEL_R {
        TX_BUS_WID_SEL_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_DATA_CFG")
            .field("tx_bitlen", &format_args!("{}", self.tx_bitlen().bits()))
            .field(
                "tx_data_order_inv",
                &format_args!("{}", self.tx_data_order_inv().bit()),
            )
            .field(
                "tx_bus_wid_sel",
                &format_args!("{}", self.tx_bus_wid_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_DATA_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 9:27 - Configures expected byte number of sent data."]
    #[inline(always)]
    #[must_use]
    pub fn tx_bitlen(&mut self) -> TX_BITLEN_W<9> {
        TX_BITLEN_W::new(self)
    }
    #[doc = "Bit 28 - Set this bit to invert bit order of one byte sent from TX_FIFO to IO data."]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_order_inv(&mut self) -> TX_DATA_ORDER_INV_W<28> {
        TX_DATA_ORDER_INV_W::new(self)
    }
    #[doc = "Bits 29:31 - Configures the txd bus width. 3'd0: bus width is 1. 3'd1: bus width is 2. 3'd2: bus width is 4. 3'd3: bus width is 8."]
    #[inline(always)]
    #[must_use]
    pub fn tx_bus_wid_sel(&mut self) -> TX_BUS_WID_SEL_W<29> {
        TX_BUS_WID_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parallel TX data configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_data_cfg](index.html) module"]
pub struct TX_DATA_CFG_SPEC;
impl crate::RegisterSpec for TX_DATA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_data_cfg::R](R) reader structure"]
impl crate::Readable for TX_DATA_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_data_cfg::W](W) writer structure"]
impl crate::Writable for TX_DATA_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_DATA_CFG to value 0x6000_0000"]
impl crate::Resettable for TX_DATA_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x6000_0000;
}
