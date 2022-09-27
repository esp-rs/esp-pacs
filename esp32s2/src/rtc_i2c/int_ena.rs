#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLAVE_TRAN_COMP_INT_ENA` reader - RTC_I2C_SLAVE_TRAN_COMP_INT interrupt enable bit"]
pub type SLAVE_TRAN_COMP_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SLAVE_TRAN_COMP_INT_ENA` writer - RTC_I2C_SLAVE_TRAN_COMP_INT interrupt enable bit"]
pub type SLAVE_TRAN_COMP_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `ARBITRATION_LOST_INT_ENA` reader - RTC_I2C_ARBITRATION_LOST_INT interrupt enable bit"]
pub type ARBITRATION_LOST_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `ARBITRATION_LOST_INT_ENA` writer - RTC_I2C_ARBITRATION_LOST_INT interrupt enable bit"]
pub type ARBITRATION_LOST_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `MASTER_TRAN_COMP_INT_ENA` reader - RTC_I2C_MASTER_TRAN_COMP_INT interrupt enable bit"]
pub type MASTER_TRAN_COMP_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `MASTER_TRAN_COMP_INT_ENA` writer - RTC_I2C_MASTER_TRAN_COMP_INT interrupt enable bit"]
pub type MASTER_TRAN_COMP_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `TRANS_COMPLETE_INT_ENA` reader - RTC_I2C_TRANS_COMPLETE_INT interrupt enable bit"]
pub type TRANS_COMPLETE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TRANS_COMPLETE_INT_ENA` writer - RTC_I2C_TRANS_COMPLETE_INT interrupt enable bit"]
pub type TRANS_COMPLETE_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `TIME_OUT_INT_ENA` reader - RTC_I2C_TIME_OUT_INT interrupt enable bit"]
pub type TIME_OUT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TIME_OUT_INT_ENA` writer - RTC_I2C_TIME_OUT_INT interrupt enable bit"]
pub type TIME_OUT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `ACK_ERR_INT_ENA` reader - RTC_I2C_ACK_ERR_INT interrupt enable bit"]
pub type ACK_ERR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `ACK_ERR_INT_ENA` writer - RTC_I2C_ACK_ERR_INT interrupt enable bit"]
pub type ACK_ERR_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `RX_DATA_INT_ENA` reader - RTC_I2C_RX_DATA_INT interrupt enable bit"]
pub type RX_DATA_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RX_DATA_INT_ENA` writer - RTC_I2C_RX_DATA_INT interrupt enable bit"]
pub type RX_DATA_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `TX_DATA_INT_ENA` reader - RTC_I2C_TX_DATA_INT interrupt enable bit"]
pub type TX_DATA_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TX_DATA_INT_ENA` writer - RTC_I2C_TX_DATA_INT interrupt enable bit"]
pub type TX_DATA_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `DETECT_START_INT_ENA` reader - RTC_I2C_DETECT_START_INT interrupt enable bit"]
pub type DETECT_START_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `DETECT_START_INT_ENA` writer - RTC_I2C_DETECT_START_INT interrupt enable bit"]
pub type DETECT_START_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RTC_I2C_SLAVE_TRAN_COMP_INT interrupt enable bit"]
    #[inline(always)]
    pub fn slave_tran_comp_int_ena(&self) -> SLAVE_TRAN_COMP_INT_ENA_R {
        SLAVE_TRAN_COMP_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC_I2C_ARBITRATION_LOST_INT interrupt enable bit"]
    #[inline(always)]
    pub fn arbitration_lost_int_ena(&self) -> ARBITRATION_LOST_INT_ENA_R {
        ARBITRATION_LOST_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC_I2C_MASTER_TRAN_COMP_INT interrupt enable bit"]
    #[inline(always)]
    pub fn master_tran_comp_int_ena(&self) -> MASTER_TRAN_COMP_INT_ENA_R {
        MASTER_TRAN_COMP_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC_I2C_TRANS_COMPLETE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn trans_complete_int_ena(&self) -> TRANS_COMPLETE_INT_ENA_R {
        TRANS_COMPLETE_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC_I2C_TIME_OUT_INT interrupt enable bit"]
    #[inline(always)]
    pub fn time_out_int_ena(&self) -> TIME_OUT_INT_ENA_R {
        TIME_OUT_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC_I2C_ACK_ERR_INT interrupt enable bit"]
    #[inline(always)]
    pub fn ack_err_int_ena(&self) -> ACK_ERR_INT_ENA_R {
        ACK_ERR_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTC_I2C_RX_DATA_INT interrupt enable bit"]
    #[inline(always)]
    pub fn rx_data_int_ena(&self) -> RX_DATA_INT_ENA_R {
        RX_DATA_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RTC_I2C_TX_DATA_INT interrupt enable bit"]
    #[inline(always)]
    pub fn tx_data_int_ena(&self) -> TX_DATA_INT_ENA_R {
        TX_DATA_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTC_I2C_DETECT_START_INT interrupt enable bit"]
    #[inline(always)]
    pub fn detect_start_int_ena(&self) -> DETECT_START_INT_ENA_R {
        DETECT_START_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC_I2C_SLAVE_TRAN_COMP_INT interrupt enable bit"]
    #[inline(always)]
    pub fn slave_tran_comp_int_ena(&mut self) -> SLAVE_TRAN_COMP_INT_ENA_W<0> {
        SLAVE_TRAN_COMP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - RTC_I2C_ARBITRATION_LOST_INT interrupt enable bit"]
    #[inline(always)]
    pub fn arbitration_lost_int_ena(&mut self) -> ARBITRATION_LOST_INT_ENA_W<1> {
        ARBITRATION_LOST_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - RTC_I2C_MASTER_TRAN_COMP_INT interrupt enable bit"]
    #[inline(always)]
    pub fn master_tran_comp_int_ena(&mut self) -> MASTER_TRAN_COMP_INT_ENA_W<2> {
        MASTER_TRAN_COMP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - RTC_I2C_TRANS_COMPLETE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn trans_complete_int_ena(&mut self) -> TRANS_COMPLETE_INT_ENA_W<3> {
        TRANS_COMPLETE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - RTC_I2C_TIME_OUT_INT interrupt enable bit"]
    #[inline(always)]
    pub fn time_out_int_ena(&mut self) -> TIME_OUT_INT_ENA_W<4> {
        TIME_OUT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - RTC_I2C_ACK_ERR_INT interrupt enable bit"]
    #[inline(always)]
    pub fn ack_err_int_ena(&mut self) -> ACK_ERR_INT_ENA_W<5> {
        ACK_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - RTC_I2C_RX_DATA_INT interrupt enable bit"]
    #[inline(always)]
    pub fn rx_data_int_ena(&mut self) -> RX_DATA_INT_ENA_W<6> {
        RX_DATA_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - RTC_I2C_TX_DATA_INT interrupt enable bit"]
    #[inline(always)]
    pub fn tx_data_int_ena(&mut self) -> TX_DATA_INT_ENA_W<7> {
        TX_DATA_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - RTC_I2C_DETECT_START_INT interrupt enable bit"]
    #[inline(always)]
    pub fn detect_start_int_ena(&mut self) -> DETECT_START_INT_ENA_W<8> {
        DETECT_START_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable RTC I2C interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
