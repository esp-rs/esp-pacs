#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `SLAVE_TRAN_COMP` writer - RTC_I2C_SLAVE_TRAN_COMP_INT interrupt clear bit"]
pub type SLAVE_TRAN_COMP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ARBITRATION_LOST` writer - RTC_I2C_ARBITRATION_LOST_INT interrupt clear bit"]
pub type ARBITRATION_LOST_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MASTER_TRAN_COMP` writer - RTC_I2C_MASTER_TRAN_COMP_INT interrupt clear bit"]
pub type MASTER_TRAN_COMP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TRANS_COMPLETE` writer - RTC_I2C_TRANS_COMPLETE_INT interrupt clear bit"]
pub type TRANS_COMPLETE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TIME_OUT` writer - RTC_I2C_TIME_OUT_INT interrupt clear bit"]
pub type TIME_OUT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ACK_ERR` writer - RTC_I2C_ACK_ERR_INT interrupt clear bit"]
pub type ACK_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RX_DATA` writer - RTC_I2C_RX_DATA_INT interrupt clear bit"]
pub type RX_DATA_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TX_DATA` writer - RTC_I2C_TX_DATA_INT interrupt clear bit"]
pub type TX_DATA_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DETECT_START` writer - RTC_I2C_DETECT_START_INT interrupt clear bit"]
pub type DETECT_START_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - RTC_I2C_SLAVE_TRAN_COMP_INT interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn slave_tran_comp(&mut self) -> SLAVE_TRAN_COMP_W<INT_CLR_SPEC> {
        SLAVE_TRAN_COMP_W::new(self, 0)
    }
    #[doc = "Bit 1 - RTC_I2C_ARBITRATION_LOST_INT interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn arbitration_lost(&mut self) -> ARBITRATION_LOST_W<INT_CLR_SPEC> {
        ARBITRATION_LOST_W::new(self, 1)
    }
    #[doc = "Bit 2 - RTC_I2C_MASTER_TRAN_COMP_INT interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn master_tran_comp(&mut self) -> MASTER_TRAN_COMP_W<INT_CLR_SPEC> {
        MASTER_TRAN_COMP_W::new(self, 2)
    }
    #[doc = "Bit 3 - RTC_I2C_TRANS_COMPLETE_INT interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn trans_complete(&mut self) -> TRANS_COMPLETE_W<INT_CLR_SPEC> {
        TRANS_COMPLETE_W::new(self, 3)
    }
    #[doc = "Bit 4 - RTC_I2C_TIME_OUT_INT interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn time_out(&mut self) -> TIME_OUT_W<INT_CLR_SPEC> {
        TIME_OUT_W::new(self, 4)
    }
    #[doc = "Bit 5 - RTC_I2C_ACK_ERR_INT interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn ack_err(&mut self) -> ACK_ERR_W<INT_CLR_SPEC> {
        ACK_ERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - RTC_I2C_RX_DATA_INT interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rx_data(&mut self) -> RX_DATA_W<INT_CLR_SPEC> {
        RX_DATA_W::new(self, 6)
    }
    #[doc = "Bit 7 - RTC_I2C_TX_DATA_INT interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn tx_data(&mut self) -> TX_DATA_W<INT_CLR_SPEC> {
        TX_DATA_W::new(self, 7)
    }
    #[doc = "Bit 8 - RTC_I2C_DETECT_START_INT interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn detect_start(&mut self) -> DETECT_START_W<INT_CLR_SPEC> {
        DETECT_START_W::new(self, 8)
    }
}
#[doc = "Clear RTC I2C interrupt\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01ff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
