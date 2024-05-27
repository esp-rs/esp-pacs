///Register `FIFO_ST` reader
pub type R = crate::R<FIFO_ST_SPEC>;
///Register `FIFO_ST` writer
pub type W = crate::W<FIFO_ST_SPEC>;
///Field `RXFIFO_START_ADDR` reader - This is the offset address of the last received data, as described in I2C_NONFIFO_RX_THRES.
pub type RXFIFO_START_ADDR_R = crate::FieldReader;
///Field `RXFIFO_END_ADDR` reader - This is the offset address of the last received data, as described in I2C_NONFIFO_RX_THRES. This value refreshes when an I2C_RXFIFO_UDF_INT or I2C_TRANS_COMPLETE_INT interrupt is generated.
pub type RXFIFO_END_ADDR_R = crate::FieldReader;
///Field `TXFIFO_START_ADDR` reader - This is the offset address of the first sent data, as described in I2C_NONFIFO_TX_THRES.
pub type TXFIFO_START_ADDR_R = crate::FieldReader;
///Field `TXFIFO_END_ADDR` reader - This is the offset address of the last sent data, as described in I2C_NONFIFO_TX_THRES. The value refreshes when an I2C_TXFIFO_OVF_INT or I2C_TRANS_COMPLETE_INT interrupt is generated.
pub type TXFIFO_END_ADDR_R = crate::FieldReader;
///Field `RX_UPDATE` writer - Write 0 or 1 to I2C_RX_UPDATE to update the value of I2C_RXFIFO_END_ADDR and I2C_RXFIFO_START_ADDR.
pub type RX_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_UPDATE` writer - Write 0 or 1 to I2C_TX_UPDATE to update the value of I2C_TXFIFO_END_ADDR and I2C_TXFIFO_START_ADDR.
pub type TX_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLAVE_RW_POINT` reader - The received data in I2C slave mode.
pub type SLAVE_RW_POINT_R = crate::FieldReader;
impl R {
    ///Bits 0:4 - This is the offset address of the last received data, as described in I2C_NONFIFO_RX_THRES.
    #[inline(always)]
    pub fn rxfifo_start_addr(&self) -> RXFIFO_START_ADDR_R {
        RXFIFO_START_ADDR_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - This is the offset address of the last received data, as described in I2C_NONFIFO_RX_THRES. This value refreshes when an I2C_RXFIFO_UDF_INT or I2C_TRANS_COMPLETE_INT interrupt is generated.
    #[inline(always)]
    pub fn rxfifo_end_addr(&self) -> RXFIFO_END_ADDR_R {
        RXFIFO_END_ADDR_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - This is the offset address of the first sent data, as described in I2C_NONFIFO_TX_THRES.
    #[inline(always)]
    pub fn txfifo_start_addr(&self) -> TXFIFO_START_ADDR_R {
        TXFIFO_START_ADDR_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 15:19 - This is the offset address of the last sent data, as described in I2C_NONFIFO_TX_THRES. The value refreshes when an I2C_TXFIFO_OVF_INT or I2C_TRANS_COMPLETE_INT interrupt is generated.
    #[inline(always)]
    pub fn txfifo_end_addr(&self) -> TXFIFO_END_ADDR_R {
        TXFIFO_END_ADDR_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 22:29 - The received data in I2C slave mode.
    #[inline(always)]
    pub fn slave_rw_point(&self) -> SLAVE_RW_POINT_R {
        SLAVE_RW_POINT_R::new(((self.bits >> 22) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_ST")
            .field("rxfifo_start_addr", &self.rxfifo_start_addr())
            .field("rxfifo_end_addr", &self.rxfifo_end_addr())
            .field("txfifo_start_addr", &self.txfifo_start_addr())
            .field("txfifo_end_addr", &self.txfifo_end_addr())
            .field("slave_rw_point", &self.slave_rw_point())
            .finish()
    }
}
impl W {
    ///Bit 20 - Write 0 or 1 to I2C_RX_UPDATE to update the value of I2C_RXFIFO_END_ADDR and I2C_RXFIFO_START_ADDR.
    #[inline(always)]
    #[must_use]
    pub fn rx_update(&mut self) -> RX_UPDATE_W<FIFO_ST_SPEC> {
        RX_UPDATE_W::new(self, 20)
    }
    ///Bit 21 - Write 0 or 1 to I2C_TX_UPDATE to update the value of I2C_TXFIFO_END_ADDR and I2C_TXFIFO_START_ADDR.
    #[inline(always)]
    #[must_use]
    pub fn tx_update(&mut self) -> TX_UPDATE_W<FIFO_ST_SPEC> {
        TX_UPDATE_W::new(self, 21)
    }
}
/**FIFO status register

You can [`read`](crate::generic::Reg::read) this register and get [`fifo_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FIFO_ST_SPEC;
impl crate::RegisterSpec for FIFO_ST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`fifo_st::R`](R) reader structure
impl crate::Readable for FIFO_ST_SPEC {}
///`write(|w| ..)` method takes [`fifo_st::W`](W) writer structure
impl crate::Writable for FIFO_ST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FIFO_ST to value 0
impl crate::Resettable for FIFO_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
