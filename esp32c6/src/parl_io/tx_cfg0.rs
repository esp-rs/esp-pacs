#[doc = "Register `TX_CFG0` reader"]
pub struct R(crate::R<TX_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CFG0` writer"]
pub struct W(crate::W<TX_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CFG0_SPEC>;
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
impl From<crate::W<TX_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BYTELEN` reader - Configures tx sending data byte length."]
pub type TX_BYTELEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TX_BYTELEN` writer - Configures tx sending data byte length."]
pub type TX_BYTELEN_W<'a, const O: u8> = crate::FieldWriter<'a, TX_CFG0_SPEC, 16, O, u16, u16>;
#[doc = "Field `TX_GATING_EN` reader - Write 1 to enable output tx clock gating."]
pub type TX_GATING_EN_R = crate::BitReader;
#[doc = "Field `TX_GATING_EN` writer - Write 1 to enable output tx clock gating."]
pub type TX_GATING_EN_W<'a, const O: u8> = crate::BitWriter<'a, TX_CFG0_SPEC, O>;
#[doc = "Field `TX_START` reader - Write 1 to start tx global data output."]
pub type TX_START_R = crate::BitReader;
#[doc = "Field `TX_START` writer - Write 1 to start tx global data output."]
pub type TX_START_W<'a, const O: u8> = crate::BitWriter<'a, TX_CFG0_SPEC, O>;
#[doc = "Field `TX_HW_VALID_EN` reader - Write 1 to enable tx hardware data valid signal."]
pub type TX_HW_VALID_EN_R = crate::BitReader;
#[doc = "Field `TX_HW_VALID_EN` writer - Write 1 to enable tx hardware data valid signal."]
pub type TX_HW_VALID_EN_W<'a, const O: u8> = crate::BitWriter<'a, TX_CFG0_SPEC, O>;
#[doc = "Field `TX_SMP_EDGE_SEL` reader - Write 0 to enable sampling data on the rising edge of tx clock. Write 0 to enable sampling data on the falling edge of tx clock."]
pub type TX_SMP_EDGE_SEL_R = crate::BitReader;
#[doc = "Field `TX_SMP_EDGE_SEL` writer - Write 0 to enable sampling data on the rising edge of tx clock. Write 0 to enable sampling data on the falling edge of tx clock."]
pub type TX_SMP_EDGE_SEL_W<'a, const O: u8> = crate::BitWriter<'a, TX_CFG0_SPEC, O>;
#[doc = "Field `TX_BIT_UNPACK_ORDER` reader - Write 0 to unpack bits from 1byte from MSB when data bus width is 4/2/1 bits. Write 0 to unpack bits from 1byte from LSB when data bus width is 4/2/1 bits."]
pub type TX_BIT_UNPACK_ORDER_R = crate::BitReader;
#[doc = "Field `TX_BIT_UNPACK_ORDER` writer - Write 0 to unpack bits from 1byte from MSB when data bus width is 4/2/1 bits. Write 0 to unpack bits from 1byte from LSB when data bus width is 4/2/1 bits."]
pub type TX_BIT_UNPACK_ORDER_W<'a, const O: u8> = crate::BitWriter<'a, TX_CFG0_SPEC, O>;
#[doc = "Field `TX_BUS_WID_SEL` reader - Tx data bus width selection. 100: bus width is 1 bit 011: bus width is 2 bits 010: bus width is 4 bits 001: bus width is 8 bits 000: bus width is 16 bits"]
pub type TX_BUS_WID_SEL_R = crate::FieldReader;
#[doc = "Field `TX_BUS_WID_SEL` writer - Tx data bus width selection. 100: bus width is 1 bit 011: bus width is 2 bits 010: bus width is 4 bits 001: bus width is 8 bits 000: bus width is 16 bits"]
pub type TX_BUS_WID_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, TX_CFG0_SPEC, 3, O>;
#[doc = "Field `TX_FIFO_SRST` reader - Write 1 to enable soft reset of async fifo in tx module."]
pub type TX_FIFO_SRST_R = crate::BitReader;
#[doc = "Field `TX_FIFO_SRST` writer - Write 1 to enable soft reset of async fifo in tx module."]
pub type TX_FIFO_SRST_W<'a, const O: u8> = crate::BitWriter<'a, TX_CFG0_SPEC, O>;
impl R {
    #[doc = "Bits 2:17 - Configures tx sending data byte length."]
    #[inline(always)]
    pub fn tx_bytelen(&self) -> TX_BYTELEN_R {
        TX_BYTELEN_R::new(((self.bits >> 2) & 0xffff) as u16)
    }
    #[doc = "Bit 18 - Write 1 to enable output tx clock gating."]
    #[inline(always)]
    pub fn tx_gating_en(&self) -> TX_GATING_EN_R {
        TX_GATING_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write 1 to start tx global data output."]
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write 1 to enable tx hardware data valid signal."]
    #[inline(always)]
    pub fn tx_hw_valid_en(&self) -> TX_HW_VALID_EN_R {
        TX_HW_VALID_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 25 - Write 0 to enable sampling data on the rising edge of tx clock. Write 0 to enable sampling data on the falling edge of tx clock."]
    #[inline(always)]
    pub fn tx_smp_edge_sel(&self) -> TX_SMP_EDGE_SEL_R {
        TX_SMP_EDGE_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Write 0 to unpack bits from 1byte from MSB when data bus width is 4/2/1 bits. Write 0 to unpack bits from 1byte from LSB when data bus width is 4/2/1 bits."]
    #[inline(always)]
    pub fn tx_bit_unpack_order(&self) -> TX_BIT_UNPACK_ORDER_R {
        TX_BIT_UNPACK_ORDER_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:29 - Tx data bus width selection. 100: bus width is 1 bit 011: bus width is 2 bits 010: bus width is 4 bits 001: bus width is 8 bits 000: bus width is 16 bits"]
    #[inline(always)]
    pub fn tx_bus_wid_sel(&self) -> TX_BUS_WID_SEL_R {
        TX_BUS_WID_SEL_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bit 30 - Write 1 to enable soft reset of async fifo in tx module."]
    #[inline(always)]
    pub fn tx_fifo_srst(&self) -> TX_FIFO_SRST_R {
        TX_FIFO_SRST_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CFG0")
            .field("tx_bytelen", &format_args!("{}", self.tx_bytelen().bits()))
            .field(
                "tx_gating_en",
                &format_args!("{}", self.tx_gating_en().bit()),
            )
            .field("tx_start", &format_args!("{}", self.tx_start().bit()))
            .field(
                "tx_hw_valid_en",
                &format_args!("{}", self.tx_hw_valid_en().bit()),
            )
            .field(
                "tx_smp_edge_sel",
                &format_args!("{}", self.tx_smp_edge_sel().bit()),
            )
            .field(
                "tx_bit_unpack_order",
                &format_args!("{}", self.tx_bit_unpack_order().bit()),
            )
            .field(
                "tx_bus_wid_sel",
                &format_args!("{}", self.tx_bus_wid_sel().bits()),
            )
            .field(
                "tx_fifo_srst",
                &format_args!("{}", self.tx_fifo_srst().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_CFG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 2:17 - Configures tx sending data byte length."]
    #[inline(always)]
    #[must_use]
    pub fn tx_bytelen(&mut self) -> TX_BYTELEN_W<2> {
        TX_BYTELEN_W::new(self)
    }
    #[doc = "Bit 18 - Write 1 to enable output tx clock gating."]
    #[inline(always)]
    #[must_use]
    pub fn tx_gating_en(&mut self) -> TX_GATING_EN_W<18> {
        TX_GATING_EN_W::new(self)
    }
    #[doc = "Bit 19 - Write 1 to start tx global data output."]
    #[inline(always)]
    #[must_use]
    pub fn tx_start(&mut self) -> TX_START_W<19> {
        TX_START_W::new(self)
    }
    #[doc = "Bit 20 - Write 1 to enable tx hardware data valid signal."]
    #[inline(always)]
    #[must_use]
    pub fn tx_hw_valid_en(&mut self) -> TX_HW_VALID_EN_W<20> {
        TX_HW_VALID_EN_W::new(self)
    }
    #[doc = "Bit 25 - Write 0 to enable sampling data on the rising edge of tx clock. Write 0 to enable sampling data on the falling edge of tx clock."]
    #[inline(always)]
    #[must_use]
    pub fn tx_smp_edge_sel(&mut self) -> TX_SMP_EDGE_SEL_W<25> {
        TX_SMP_EDGE_SEL_W::new(self)
    }
    #[doc = "Bit 26 - Write 0 to unpack bits from 1byte from MSB when data bus width is 4/2/1 bits. Write 0 to unpack bits from 1byte from LSB when data bus width is 4/2/1 bits."]
    #[inline(always)]
    #[must_use]
    pub fn tx_bit_unpack_order(&mut self) -> TX_BIT_UNPACK_ORDER_W<26> {
        TX_BIT_UNPACK_ORDER_W::new(self)
    }
    #[doc = "Bits 27:29 - Tx data bus width selection. 100: bus width is 1 bit 011: bus width is 2 bits 010: bus width is 4 bits 001: bus width is 8 bits 000: bus width is 16 bits"]
    #[inline(always)]
    #[must_use]
    pub fn tx_bus_wid_sel(&mut self) -> TX_BUS_WID_SEL_W<27> {
        TX_BUS_WID_SEL_W::new(self)
    }
    #[doc = "Bit 30 - Write 1 to enable soft reset of async fifo in tx module."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_srst(&mut self) -> TX_FIFO_SRST_W<30> {
        TX_FIFO_SRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parallel TX module configuration register0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_cfg0](index.html) module"]
pub struct TX_CFG0_SPEC;
impl crate::RegisterSpec for TX_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_cfg0::R](R) reader structure"]
impl crate::Readable for TX_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_cfg0::W](W) writer structure"]
impl crate::Writable for TX_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_CFG0 to value 0"]
impl crate::Resettable for TX_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
