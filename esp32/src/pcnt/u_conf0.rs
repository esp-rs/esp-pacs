#[doc = "Register `U%s_CONF0` reader"]
pub struct R(crate::R<U_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<U_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<U_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U%s_CONF0` writer"]
pub struct W(crate::W<U_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U_CONF0_SPEC>;
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
impl From<crate::W<U_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<U_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTER_THRES` reader - This register is used to filter pluse whose width is smaller than this value for unit0."]
pub type FILTER_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `FILTER_THRES` writer - This register is used to filter pluse whose width is smaller than this value for unit0."]
pub type FILTER_THRES_W<'a, const O: u8> = crate::FieldWriter<'a, U_CONF0_SPEC, 10, O, u16>;
#[doc = "Field `FILTER_EN` reader - This is the enable bit for filtering input signals for unit0."]
pub type FILTER_EN_R = crate::BitReader;
#[doc = "Field `FILTER_EN` writer - This is the enable bit for filtering input signals for unit0."]
pub type FILTER_EN_W<'a, const O: u8> = crate::BitWriter<'a, U_CONF0_SPEC, O>;
#[doc = "Field `THR_ZERO_EN` reader - This is the enable bit for comparing unit0's count with 0 value."]
pub type THR_ZERO_EN_R = crate::BitReader;
#[doc = "Field `THR_ZERO_EN` writer - This is the enable bit for comparing unit0's count with 0 value."]
pub type THR_ZERO_EN_W<'a, const O: u8> = crate::BitWriter<'a, U_CONF0_SPEC, O>;
#[doc = "Field `THR_H_LIM_EN` reader - This is the enable bit for comparing unit0's count with thr_h_lim value."]
pub type THR_H_LIM_EN_R = crate::BitReader;
#[doc = "Field `THR_H_LIM_EN` writer - This is the enable bit for comparing unit0's count with thr_h_lim value."]
pub type THR_H_LIM_EN_W<'a, const O: u8> = crate::BitWriter<'a, U_CONF0_SPEC, O>;
#[doc = "Field `THR_L_LIM_EN` reader - This is the enable bit for comparing unit0's count with thr_l_lim value."]
pub type THR_L_LIM_EN_R = crate::BitReader;
#[doc = "Field `THR_L_LIM_EN` writer - This is the enable bit for comparing unit0's count with thr_l_lim value."]
pub type THR_L_LIM_EN_W<'a, const O: u8> = crate::BitWriter<'a, U_CONF0_SPEC, O>;
#[doc = "Field `THR_THRES0_EN` reader - This is the enable bit for comparing unit0's count with thres0 value."]
pub type THR_THRES0_EN_R = crate::BitReader;
#[doc = "Field `THR_THRES0_EN` writer - This is the enable bit for comparing unit0's count with thres0 value."]
pub type THR_THRES0_EN_W<'a, const O: u8> = crate::BitWriter<'a, U_CONF0_SPEC, O>;
#[doc = "Field `THR_THRES1_EN` reader - This is the enable bit for comparing unit0's count with thres1 value ."]
pub type THR_THRES1_EN_R = crate::BitReader;
#[doc = "Field `THR_THRES1_EN` writer - This is the enable bit for comparing unit0's count with thres1 value ."]
pub type THR_THRES1_EN_W<'a, const O: u8> = crate::BitWriter<'a, U_CONF0_SPEC, O>;
#[doc = "Field `CH0_NEG_MODE` reader - This register is used to control the mode of channel0's input negedge signal for unit0. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
pub type CH0_NEG_MODE_R = crate::FieldReader;
#[doc = "Field `CH0_NEG_MODE` writer - This register is used to control the mode of channel0's input negedge signal for unit0. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
pub type CH0_NEG_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, U_CONF0_SPEC, 2, O>;
#[doc = "Field `CH0_POS_MODE` reader - This register is used to control the mode of channel0's input posedge signal for unit0. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
pub type CH0_POS_MODE_R = crate::FieldReader;
#[doc = "Field `CH0_POS_MODE` writer - This register is used to control the mode of channel0's input posedge signal for unit0. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
pub type CH0_POS_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, U_CONF0_SPEC, 2, O>;
#[doc = "Field `CH0_HCTRL_MODE` reader - This register is used to control the mode of channel0's high control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH0_HCTRL_MODE_R = crate::FieldReader;
#[doc = "Field `CH0_HCTRL_MODE` writer - This register is used to control the mode of channel0's high control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH0_HCTRL_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, U_CONF0_SPEC, 2, O>;
#[doc = "Field `CH0_LCTRL_MODE` reader - This register is used to control the mode of channel0's low control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH0_LCTRL_MODE_R = crate::FieldReader;
#[doc = "Field `CH0_LCTRL_MODE` writer - This register is used to control the mode of channel0's low control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH0_LCTRL_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, U_CONF0_SPEC, 2, O>;
#[doc = "Field `CH1_NEG_MODE` reader - This register is used to control the mode of channel1's input negedge signal for unit0. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
pub type CH1_NEG_MODE_R = crate::FieldReader;
#[doc = "Field `CH1_NEG_MODE` writer - This register is used to control the mode of channel1's input negedge signal for unit0. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
pub type CH1_NEG_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, U_CONF0_SPEC, 2, O>;
#[doc = "Field `CH1_POS_MODE` reader - This register is used to control the mode of channel1's input posedge signal for unit0. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
pub type CH1_POS_MODE_R = crate::FieldReader;
#[doc = "Field `CH1_POS_MODE` writer - This register is used to control the mode of channel1's input posedge signal for unit0. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
pub type CH1_POS_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, U_CONF0_SPEC, 2, O>;
#[doc = "Field `CH1_HCTRL_MODE` reader - This register is used to control the mode of channel1's high control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH1_HCTRL_MODE_R = crate::FieldReader;
#[doc = "Field `CH1_HCTRL_MODE` writer - This register is used to control the mode of channel1's high control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH1_HCTRL_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, U_CONF0_SPEC, 2, O>;
#[doc = "Field `CH1_LCTRL_MODE` reader - This register is used to control the mode of channel1's low control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH1_LCTRL_MODE_R = crate::FieldReader;
#[doc = "Field `CH1_LCTRL_MODE` writer - This register is used to control the mode of channel1's low control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH1_LCTRL_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, U_CONF0_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:9 - This register is used to filter pluse whose width is smaller than this value for unit0."]
    #[inline(always)]
    pub fn filter_thres(&self) -> FILTER_THRES_R {
        FILTER_THRES_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - This is the enable bit for filtering input signals for unit0."]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This is the enable bit for comparing unit0's count with 0 value."]
    #[inline(always)]
    pub fn thr_zero_en(&self) -> THR_ZERO_EN_R {
        THR_ZERO_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This is the enable bit for comparing unit0's count with thr_h_lim value."]
    #[inline(always)]
    pub fn thr_h_lim_en(&self) -> THR_H_LIM_EN_R {
        THR_H_LIM_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This is the enable bit for comparing unit0's count with thr_l_lim value."]
    #[inline(always)]
    pub fn thr_l_lim_en(&self) -> THR_L_LIM_EN_R {
        THR_L_LIM_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This is the enable bit for comparing unit0's count with thres0 value."]
    #[inline(always)]
    pub fn thr_thres0_en(&self) -> THR_THRES0_EN_R {
        THR_THRES0_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This is the enable bit for comparing unit0's count with thres1 value ."]
    #[inline(always)]
    pub fn thr_thres1_en(&self) -> THR_THRES1_EN_R {
        THR_THRES1_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - This register is used to control the mode of channel0's input negedge signal for unit0. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn ch0_neg_mode(&self) -> CH0_NEG_MODE_R {
        CH0_NEG_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - This register is used to control the mode of channel0's input posedge signal for unit0. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn ch0_pos_mode(&self) -> CH0_POS_MODE_R {
        CH0_POS_MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - This register is used to control the mode of channel0's high control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn ch0_hctrl_mode(&self) -> CH0_HCTRL_MODE_R {
        CH0_HCTRL_MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - This register is used to control the mode of channel0's low control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn ch0_lctrl_mode(&self) -> CH0_LCTRL_MODE_R {
        CH0_LCTRL_MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - This register is used to control the mode of channel1's input negedge signal for unit0. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn ch1_neg_mode(&self) -> CH1_NEG_MODE_R {
        CH1_NEG_MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - This register is used to control the mode of channel1's input posedge signal for unit0. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn ch1_pos_mode(&self) -> CH1_POS_MODE_R {
        CH1_POS_MODE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - This register is used to control the mode of channel1's high control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn ch1_hctrl_mode(&self) -> CH1_HCTRL_MODE_R {
        CH1_HCTRL_MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - This register is used to control the mode of channel1's low control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn ch1_lctrl_mode(&self) -> CH1_LCTRL_MODE_R {
        CH1_LCTRL_MODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("U_CONF0")
            .field(
                "filter_thres",
                &format_args!("{}", self.filter_thres().bits()),
            )
            .field("filter_en", &format_args!("{}", self.filter_en().bit()))
            .field("thr_zero_en", &format_args!("{}", self.thr_zero_en().bit()))
            .field(
                "thr_h_lim_en",
                &format_args!("{}", self.thr_h_lim_en().bit()),
            )
            .field(
                "thr_l_lim_en",
                &format_args!("{}", self.thr_l_lim_en().bit()),
            )
            .field(
                "thr_thres0_en",
                &format_args!("{}", self.thr_thres0_en().bit()),
            )
            .field(
                "thr_thres1_en",
                &format_args!("{}", self.thr_thres1_en().bit()),
            )
            .field(
                "ch0_neg_mode",
                &format_args!("{}", self.ch0_neg_mode().bits()),
            )
            .field(
                "ch0_pos_mode",
                &format_args!("{}", self.ch0_pos_mode().bits()),
            )
            .field(
                "ch0_hctrl_mode",
                &format_args!("{}", self.ch0_hctrl_mode().bits()),
            )
            .field(
                "ch0_lctrl_mode",
                &format_args!("{}", self.ch0_lctrl_mode().bits()),
            )
            .field(
                "ch1_neg_mode",
                &format_args!("{}", self.ch1_neg_mode().bits()),
            )
            .field(
                "ch1_pos_mode",
                &format_args!("{}", self.ch1_pos_mode().bits()),
            )
            .field(
                "ch1_hctrl_mode",
                &format_args!("{}", self.ch1_hctrl_mode().bits()),
            )
            .field(
                "ch1_lctrl_mode",
                &format_args!("{}", self.ch1_lctrl_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<U_CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - This register is used to filter pluse whose width is smaller than this value for unit0."]
    #[inline(always)]
    #[must_use]
    pub fn filter_thres(&mut self) -> FILTER_THRES_W<0> {
        FILTER_THRES_W::new(self)
    }
    #[doc = "Bit 10 - This is the enable bit for filtering input signals for unit0."]
    #[inline(always)]
    #[must_use]
    pub fn filter_en(&mut self) -> FILTER_EN_W<10> {
        FILTER_EN_W::new(self)
    }
    #[doc = "Bit 11 - This is the enable bit for comparing unit0's count with 0 value."]
    #[inline(always)]
    #[must_use]
    pub fn thr_zero_en(&mut self) -> THR_ZERO_EN_W<11> {
        THR_ZERO_EN_W::new(self)
    }
    #[doc = "Bit 12 - This is the enable bit for comparing unit0's count with thr_h_lim value."]
    #[inline(always)]
    #[must_use]
    pub fn thr_h_lim_en(&mut self) -> THR_H_LIM_EN_W<12> {
        THR_H_LIM_EN_W::new(self)
    }
    #[doc = "Bit 13 - This is the enable bit for comparing unit0's count with thr_l_lim value."]
    #[inline(always)]
    #[must_use]
    pub fn thr_l_lim_en(&mut self) -> THR_L_LIM_EN_W<13> {
        THR_L_LIM_EN_W::new(self)
    }
    #[doc = "Bit 14 - This is the enable bit for comparing unit0's count with thres0 value."]
    #[inline(always)]
    #[must_use]
    pub fn thr_thres0_en(&mut self) -> THR_THRES0_EN_W<14> {
        THR_THRES0_EN_W::new(self)
    }
    #[doc = "Bit 15 - This is the enable bit for comparing unit0's count with thres1 value ."]
    #[inline(always)]
    #[must_use]
    pub fn thr_thres1_en(&mut self) -> THR_THRES1_EN_W<15> {
        THR_THRES1_EN_W::new(self)
    }
    #[doc = "Bits 16:17 - This register is used to control the mode of channel0's input negedge signal for unit0. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_neg_mode(&mut self) -> CH0_NEG_MODE_W<16> {
        CH0_NEG_MODE_W::new(self)
    }
    #[doc = "Bits 18:19 - This register is used to control the mode of channel0's input posedge signal for unit0. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_pos_mode(&mut self) -> CH0_POS_MODE_W<18> {
        CH0_POS_MODE_W::new(self)
    }
    #[doc = "Bits 20:21 - This register is used to control the mode of channel0's high control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_hctrl_mode(&mut self) -> CH0_HCTRL_MODE_W<20> {
        CH0_HCTRL_MODE_W::new(self)
    }
    #[doc = "Bits 22:23 - This register is used to control the mode of channel0's low control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_lctrl_mode(&mut self) -> CH0_LCTRL_MODE_W<22> {
        CH0_LCTRL_MODE_W::new(self)
    }
    #[doc = "Bits 24:25 - This register is used to control the mode of channel1's input negedge signal for unit0. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_neg_mode(&mut self) -> CH1_NEG_MODE_W<24> {
        CH1_NEG_MODE_W::new(self)
    }
    #[doc = "Bits 26:27 - This register is used to control the mode of channel1's input posedge signal for unit0. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_pos_mode(&mut self) -> CH1_POS_MODE_W<26> {
        CH1_POS_MODE_W::new(self)
    }
    #[doc = "Bits 28:29 - This register is used to control the mode of channel1's high control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_hctrl_mode(&mut self) -> CH1_HCTRL_MODE_W<28> {
        CH1_HCTRL_MODE_W::new(self)
    }
    #[doc = "Bits 30:31 - This register is used to control the mode of channel1's low control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_lctrl_mode(&mut self) -> CH1_LCTRL_MODE_W<30> {
        CH1_LCTRL_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u_conf0](index.html) module"]
pub struct U_CONF0_SPEC;
impl crate::RegisterSpec for U_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u_conf0::R](R) reader structure"]
impl crate::Readable for U_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u_conf0::W](W) writer structure"]
impl crate::Writable for U_CONF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets U%s_CONF0 to value 0x3c10"]
impl crate::Resettable for U_CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x3c10;
}
