#[doc = "Register `FIFO_ST` reader"]
pub struct R(crate::R<FIFO_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO_ST` writer"]
pub struct W(crate::W<FIFO_ST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_ST_SPEC>;
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
impl From<crate::W<FIFO_ST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_ST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO_START_ADDR` reader - This is the offset address of the last received data, as described in I2C_NONFIFO_RX_THRES."]
pub type RXFIFO_START_ADDR_R = crate::FieldReader;
#[doc = "Field `RXFIFO_END_ADDR` reader - This is the offset address of the last received data, as described in I2C_NONFIFO_RX_THRES. This value refreshes when an I2C_RXFIFO_UDF_INT or I2C_TRANS_COMPLETE_INT interrupt is generated."]
pub type RXFIFO_END_ADDR_R = crate::FieldReader;
#[doc = "Field `TXFIFO_START_ADDR` reader - This is the offset address of the first sent data, as described in I2C_NONFIFO_TX_THRES."]
pub type TXFIFO_START_ADDR_R = crate::FieldReader;
#[doc = "Field `TXFIFO_END_ADDR` reader - This is the offset address of the last sent data, as described in I2C_NONFIFO_TX_THRES. The value refreshes when an I2C_TXFIFO_OVF_INT or I2C_TRANS_COMPLETE_INT interrupt is generated."]
pub type TXFIFO_END_ADDR_R = crate::FieldReader;
#[doc = "Field `RX_UPDATE` writer - Write 0 or 1 to I2C_RX_UPDATE to update the value of I2C_RXFIFO_END_ADDR and I2C_RXFIFO_START_ADDR."]
pub type RX_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, FIFO_ST_SPEC, O>;
#[doc = "Field `TX_UPDATE` writer - Write 0 or 1 to I2C_TX_UPDATE to update the value of I2C_TXFIFO_END_ADDR and I2C_TXFIFO_START_ADDR."]
pub type TX_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, FIFO_ST_SPEC, O>;
#[doc = "Field `SLAVE_RW_POINT` reader - The received data in I2C slave mode."]
pub type SLAVE_RW_POINT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - This is the offset address of the last received data, as described in I2C_NONFIFO_RX_THRES."]
    #[inline(always)]
    pub fn rxfifo_start_addr(&self) -> RXFIFO_START_ADDR_R {
        RXFIFO_START_ADDR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - This is the offset address of the last received data, as described in I2C_NONFIFO_RX_THRES. This value refreshes when an I2C_RXFIFO_UDF_INT or I2C_TRANS_COMPLETE_INT interrupt is generated."]
    #[inline(always)]
    pub fn rxfifo_end_addr(&self) -> RXFIFO_END_ADDR_R {
        RXFIFO_END_ADDR_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - This is the offset address of the first sent data, as described in I2C_NONFIFO_TX_THRES."]
    #[inline(always)]
    pub fn txfifo_start_addr(&self) -> TXFIFO_START_ADDR_R {
        TXFIFO_START_ADDR_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - This is the offset address of the last sent data, as described in I2C_NONFIFO_TX_THRES. The value refreshes when an I2C_TXFIFO_OVF_INT or I2C_TRANS_COMPLETE_INT interrupt is generated."]
    #[inline(always)]
    pub fn txfifo_end_addr(&self) -> TXFIFO_END_ADDR_R {
        TXFIFO_END_ADDR_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 22:29 - The received data in I2C slave mode."]
    #[inline(always)]
    pub fn slave_rw_point(&self) -> SLAVE_RW_POINT_R {
        SLAVE_RW_POINT_R::new(((self.bits >> 22) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_ST")
            .field(
                "rxfifo_start_addr",
                &format_args!("{}", self.rxfifo_start_addr().bits()),
            )
            .field(
                "rxfifo_end_addr",
                &format_args!("{}", self.rxfifo_end_addr().bits()),
            )
            .field(
                "txfifo_start_addr",
                &format_args!("{}", self.txfifo_start_addr().bits()),
            )
            .field(
                "txfifo_end_addr",
                &format_args!("{}", self.txfifo_end_addr().bits()),
            )
            .field(
                "slave_rw_point",
                &format_args!("{}", self.slave_rw_point().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FIFO_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 20 - Write 0 or 1 to I2C_RX_UPDATE to update the value of I2C_RXFIFO_END_ADDR and I2C_RXFIFO_START_ADDR."]
    #[inline(always)]
    #[must_use]
    pub fn rx_update(&mut self) -> RX_UPDATE_W<20> {
        RX_UPDATE_W::new(self)
    }
    #[doc = "Bit 21 - Write 0 or 1 to I2C_TX_UPDATE to update the value of I2C_TXFIFO_END_ADDR and I2C_TXFIFO_START_ADDR."]
    #[inline(always)]
    #[must_use]
    pub fn tx_update(&mut self) -> TX_UPDATE_W<21> {
        TX_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_st](index.html) module"]
pub struct FIFO_ST_SPEC;
impl crate::RegisterSpec for FIFO_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_st::R](R) reader structure"]
impl crate::Readable for FIFO_ST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_st::W](W) writer structure"]
impl crate::Writable for FIFO_ST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFO_ST to value 0"]
impl crate::Resettable for FIFO_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
