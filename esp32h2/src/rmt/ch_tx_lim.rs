#[doc = "Register `CH%s_TX_LIM` reader"]
pub struct R(crate::R<CH_TX_LIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_TX_LIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_TX_LIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_TX_LIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%s_TX_LIM` writer"]
pub struct W(crate::W<CH_TX_LIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_TX_LIM_SPEC>;
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
impl From<crate::W<CH_TX_LIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_TX_LIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_LIM_CH0` reader - This register is used to configure the maximum entries that CHANNEL%s can send out."]
pub type TX_LIM_CH0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TX_LIM_CH0` writer - This register is used to configure the maximum entries that CHANNEL%s can send out."]
pub type TX_LIM_CH0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH_TX_LIM_SPEC, u16, u16, 9, O>;
#[doc = "Field `TX_LOOP_NUM_CH0` reader - This register is used to configure the maximum loop count when tx_conti_mode is valid."]
pub type TX_LOOP_NUM_CH0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TX_LOOP_NUM_CH0` writer - This register is used to configure the maximum loop count when tx_conti_mode is valid."]
pub type TX_LOOP_NUM_CH0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH_TX_LIM_SPEC, u16, u16, 10, O>;
#[doc = "Field `TX_LOOP_CNT_EN_CH0` reader - This register is the enabled bit for loop count."]
pub type TX_LOOP_CNT_EN_CH0_R = crate::BitReader<bool>;
#[doc = "Field `TX_LOOP_CNT_EN_CH0` writer - This register is the enabled bit for loop count."]
pub type TX_LOOP_CNT_EN_CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_TX_LIM_SPEC, bool, O>;
#[doc = "Field `LOOP_COUNT_RESET_CH0` writer - This register is used to reset the loop count when tx_conti_mode is valid."]
pub type LOOP_COUNT_RESET_CH0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CH_TX_LIM_SPEC, bool, O>;
#[doc = "Field `LOOP_STOP_EN_CH0` reader - This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s."]
pub type LOOP_STOP_EN_CH0_R = crate::BitReader<bool>;
#[doc = "Field `LOOP_STOP_EN_CH0` writer - This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s."]
pub type LOOP_STOP_EN_CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_TX_LIM_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:8 - This register is used to configure the maximum entries that CHANNEL%s can send out."]
    #[inline(always)]
    pub fn tx_lim_ch0(&self) -> TX_LIM_CH0_R {
        TX_LIM_CH0_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:18 - This register is used to configure the maximum loop count when tx_conti_mode is valid."]
    #[inline(always)]
    pub fn tx_loop_num_ch0(&self) -> TX_LOOP_NUM_CH0_R {
        TX_LOOP_NUM_CH0_R::new(((self.bits >> 9) & 0x03ff) as u16)
    }
    #[doc = "Bit 19 - This register is the enabled bit for loop count."]
    #[inline(always)]
    pub fn tx_loop_cnt_en_ch0(&self) -> TX_LOOP_CNT_EN_CH0_R {
        TX_LOOP_CNT_EN_CH0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s."]
    #[inline(always)]
    pub fn loop_stop_en_ch0(&self) -> LOOP_STOP_EN_CH0_R {
        LOOP_STOP_EN_CH0_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - This register is used to configure the maximum entries that CHANNEL%s can send out."]
    #[inline(always)]
    #[must_use]
    pub fn tx_lim_ch0(&mut self) -> TX_LIM_CH0_W<0> {
        TX_LIM_CH0_W::new(self)
    }
    #[doc = "Bits 9:18 - This register is used to configure the maximum loop count when tx_conti_mode is valid."]
    #[inline(always)]
    #[must_use]
    pub fn tx_loop_num_ch0(&mut self) -> TX_LOOP_NUM_CH0_W<9> {
        TX_LOOP_NUM_CH0_W::new(self)
    }
    #[doc = "Bit 19 - This register is the enabled bit for loop count."]
    #[inline(always)]
    #[must_use]
    pub fn tx_loop_cnt_en_ch0(&mut self) -> TX_LOOP_CNT_EN_CH0_W<19> {
        TX_LOOP_CNT_EN_CH0_W::new(self)
    }
    #[doc = "Bit 20 - This register is used to reset the loop count when tx_conti_mode is valid."]
    #[inline(always)]
    #[must_use]
    pub fn loop_count_reset_ch0(&mut self) -> LOOP_COUNT_RESET_CH0_W<20> {
        LOOP_COUNT_RESET_CH0_W::new(self)
    }
    #[doc = "Bit 21 - This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s."]
    #[inline(always)]
    #[must_use]
    pub fn loop_stop_en_ch0(&mut self) -> LOOP_STOP_EN_CH0_W<21> {
        LOOP_STOP_EN_CH0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel %s Tx event configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_tx_lim](index.html) module"]
pub struct CH_TX_LIM_SPEC;
impl crate::RegisterSpec for CH_TX_LIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_tx_lim::R](R) reader structure"]
impl crate::Readable for CH_TX_LIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_tx_lim::W](W) writer structure"]
impl crate::Writable for CH_TX_LIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%s_TX_LIM to value 0x80"]
impl crate::Resettable for CH_TX_LIM_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
