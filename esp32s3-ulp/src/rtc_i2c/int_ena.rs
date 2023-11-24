#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `SLAVE_TRAN_COMP_INT_ENA` reader - enable slave transit complete interrupt"]
pub type SLAVE_TRAN_COMP_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLAVE_TRAN_COMP_INT_ENA` writer - enable slave transit complete interrupt"]
pub type SLAVE_TRAN_COMP_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBITRATION_LOST_INT_ENA` reader - enable arbitration lost interrupt"]
pub type ARBITRATION_LOST_INT_ENA_R = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST_INT_ENA` writer - enable arbitration lost interrupt"]
pub type ARBITRATION_LOST_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER_TRAN_COMP_INT_ENA` reader - enable master transit complete interrupt"]
pub type MASTER_TRAN_COMP_INT_ENA_R = crate::BitReader;
#[doc = "Field `MASTER_TRAN_COMP_INT_ENA` writer - enable master transit complete interrupt"]
pub type MASTER_TRAN_COMP_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_COMPLETE_INT_ENA` reader - enable transit complete interrupt"]
pub type TRANS_COMPLETE_INT_ENA_R = crate::BitReader;
#[doc = "Field `TRANS_COMPLETE_INT_ENA` writer - enable transit complete interrupt"]
pub type TRANS_COMPLETE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_OUT_INT_ENA` reader - enable time out interrupt"]
pub type TIME_OUT_INT_ENA_R = crate::BitReader;
#[doc = "Field `TIME_OUT_INT_ENA` writer - enable time out interrupt"]
pub type TIME_OUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_ERR_INT_ENA` reader - enable eack error interrupt"]
pub type ACK_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `ACK_ERR_INT_ENA` writer - enable eack error interrupt"]
pub type ACK_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DATA_INT_ENA` reader - enable receive data interrupt"]
pub type RX_DATA_INT_ENA_R = crate::BitReader;
#[doc = "Field `RX_DATA_INT_ENA` writer - enable receive data interrupt"]
pub type RX_DATA_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DATA_INT_ENA` reader - enable transit data interrupt"]
pub type TX_DATA_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_DATA_INT_ENA` writer - enable transit data interrupt"]
pub type TX_DATA_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DETECT_START_INT_ENA` reader - enable detect start interrupt"]
pub type DETECT_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `DETECT_START_INT_ENA` writer - enable detect start interrupt"]
pub type DETECT_START_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable slave transit complete interrupt"]
    #[inline(always)]
    pub fn slave_tran_comp_int_ena(&self) -> SLAVE_TRAN_COMP_INT_ENA_R {
        SLAVE_TRAN_COMP_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable arbitration lost interrupt"]
    #[inline(always)]
    pub fn arbitration_lost_int_ena(&self) -> ARBITRATION_LOST_INT_ENA_R {
        ARBITRATION_LOST_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable master transit complete interrupt"]
    #[inline(always)]
    pub fn master_tran_comp_int_ena(&self) -> MASTER_TRAN_COMP_INT_ENA_R {
        MASTER_TRAN_COMP_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable transit complete interrupt"]
    #[inline(always)]
    pub fn trans_complete_int_ena(&self) -> TRANS_COMPLETE_INT_ENA_R {
        TRANS_COMPLETE_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable time out interrupt"]
    #[inline(always)]
    pub fn time_out_int_ena(&self) -> TIME_OUT_INT_ENA_R {
        TIME_OUT_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - enable eack error interrupt"]
    #[inline(always)]
    pub fn ack_err_int_ena(&self) -> ACK_ERR_INT_ENA_R {
        ACK_ERR_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - enable receive data interrupt"]
    #[inline(always)]
    pub fn rx_data_int_ena(&self) -> RX_DATA_INT_ENA_R {
        RX_DATA_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable transit data interrupt"]
    #[inline(always)]
    pub fn tx_data_int_ena(&self) -> TX_DATA_INT_ENA_R {
        TX_DATA_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - enable detect start interrupt"]
    #[inline(always)]
    pub fn detect_start_int_ena(&self) -> DETECT_START_INT_ENA_R {
        DETECT_START_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "slave_tran_comp_int_ena",
                &format_args!("{}", self.slave_tran_comp_int_ena().bit()),
            )
            .field(
                "arbitration_lost_int_ena",
                &format_args!("{}", self.arbitration_lost_int_ena().bit()),
            )
            .field(
                "master_tran_comp_int_ena",
                &format_args!("{}", self.master_tran_comp_int_ena().bit()),
            )
            .field(
                "trans_complete_int_ena",
                &format_args!("{}", self.trans_complete_int_ena().bit()),
            )
            .field(
                "time_out_int_ena",
                &format_args!("{}", self.time_out_int_ena().bit()),
            )
            .field(
                "ack_err_int_ena",
                &format_args!("{}", self.ack_err_int_ena().bit()),
            )
            .field(
                "rx_data_int_ena",
                &format_args!("{}", self.rx_data_int_ena().bit()),
            )
            .field(
                "tx_data_int_ena",
                &format_args!("{}", self.tx_data_int_ena().bit()),
            )
            .field(
                "detect_start_int_ena",
                &format_args!("{}", self.detect_start_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - enable slave transit complete interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slave_tran_comp_int_ena(&mut self) -> SLAVE_TRAN_COMP_INT_ENA_W<INT_ENA_SPEC> {
        SLAVE_TRAN_COMP_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - enable arbitration lost interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn arbitration_lost_int_ena(&mut self) -> ARBITRATION_LOST_INT_ENA_W<INT_ENA_SPEC> {
        ARBITRATION_LOST_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - enable master transit complete interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn master_tran_comp_int_ena(&mut self) -> MASTER_TRAN_COMP_INT_ENA_W<INT_ENA_SPEC> {
        MASTER_TRAN_COMP_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - enable transit complete interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn trans_complete_int_ena(&mut self) -> TRANS_COMPLETE_INT_ENA_W<INT_ENA_SPEC> {
        TRANS_COMPLETE_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - enable time out interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn time_out_int_ena(&mut self) -> TIME_OUT_INT_ENA_W<INT_ENA_SPEC> {
        TIME_OUT_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - enable eack error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ack_err_int_ena(&mut self) -> ACK_ERR_INT_ENA_W<INT_ENA_SPEC> {
        ACK_ERR_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - enable receive data interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_int_ena(&mut self) -> RX_DATA_INT_ENA_W<INT_ENA_SPEC> {
        RX_DATA_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - enable transit data interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_int_ena(&mut self) -> TX_DATA_INT_ENA_W<INT_ENA_SPEC> {
        TX_DATA_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - enable detect start interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn detect_start_int_ena(&mut self) -> DETECT_START_INT_ENA_W<INT_ENA_SPEC> {
        DETECT_START_INT_ENA_W::new(self, 8)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
