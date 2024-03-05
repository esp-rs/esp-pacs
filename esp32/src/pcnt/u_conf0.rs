#[doc = "Register `U%s_CONF0` reader"]
pub type R = crate::R<U_CONF0_SPEC>;
#[doc = "Register `U%s_CONF0` writer"]
pub type W = crate::W<U_CONF0_SPEC>;
#[doc = "Field `FILTER_THRES` reader - This register is used to filter pluse whose width is smaller than this value for unit0."]
pub type FILTER_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `FILTER_THRES` writer - This register is used to filter pluse whose width is smaller than this value for unit0."]
pub type FILTER_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `FILTER_EN` reader - This is the enable bit for filtering input signals for unit0."]
pub type FILTER_EN_R = crate::BitReader;
#[doc = "Field `FILTER_EN` writer - This is the enable bit for filtering input signals for unit0."]
pub type FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR_ZERO_EN` reader - This is the enable bit for comparing unit0's count with 0 value."]
pub type THR_ZERO_EN_R = crate::BitReader;
#[doc = "Field `THR_ZERO_EN` writer - This is the enable bit for comparing unit0's count with 0 value."]
pub type THR_ZERO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR_H_LIM_EN` reader - This is the enable bit for comparing unit0's count with thr_h_lim value."]
pub type THR_H_LIM_EN_R = crate::BitReader;
#[doc = "Field `THR_H_LIM_EN` writer - This is the enable bit for comparing unit0's count with thr_h_lim value."]
pub type THR_H_LIM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR_L_LIM_EN` reader - This is the enable bit for comparing unit0's count with thr_l_lim value."]
pub type THR_L_LIM_EN_R = crate::BitReader;
#[doc = "Field `THR_L_LIM_EN` writer - This is the enable bit for comparing unit0's count with thr_l_lim value."]
pub type THR_L_LIM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR_THRES0_EN` reader - This is the enable bit for comparing unit0's count with thres0 value."]
pub type THR_THRES0_EN_R = crate::BitReader;
#[doc = "Field `THR_THRES0_EN` writer - This is the enable bit for comparing unit0's count with thres0 value."]
pub type THR_THRES0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR_THRES1_EN` reader - This is the enable bit for comparing unit0's count with thres1 value ."]
pub type THR_THRES1_EN_R = crate::BitReader;
#[doc = "Field `THR_THRES1_EN` writer - This is the enable bit for comparing unit0's count with thres1 value ."]
pub type THR_THRES1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0_NEG_MODE` reader - This register is used to control the mode of channel0's input negedge signal for unit0. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
pub type CH0_NEG_MODE_R = crate::FieldReader;
#[doc = "Field `CH0_NEG_MODE` writer - This register is used to control the mode of channel0's input negedge signal for unit0. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
pub type CH0_NEG_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH0_POS_MODE` reader - This register is used to control the mode of channel0's input posedge signal for unit0. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
pub type CH0_POS_MODE_R = crate::FieldReader;
#[doc = "Field `CH0_POS_MODE` writer - This register is used to control the mode of channel0's input posedge signal for unit0. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
pub type CH0_POS_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH0_HCTRL_MODE` reader - This register is used to control the mode of channel0's high control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH0_HCTRL_MODE_R = crate::FieldReader;
#[doc = "Field `CH0_HCTRL_MODE` writer - This register is used to control the mode of channel0's high control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH0_HCTRL_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH0_LCTRL_MODE` reader - This register is used to control the mode of channel0's low control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH0_LCTRL_MODE_R = crate::FieldReader;
#[doc = "Field `CH0_LCTRL_MODE` writer - This register is used to control the mode of channel0's low control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH0_LCTRL_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1_NEG_MODE` reader - This register is used to control the mode of channel1's input negedge signal for unit0. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
pub type CH1_NEG_MODE_R = crate::FieldReader;
#[doc = "Field `CH1_NEG_MODE` writer - This register is used to control the mode of channel1's input negedge signal for unit0. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
pub type CH1_NEG_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1_POS_MODE` reader - This register is used to control the mode of channel1's input posedge signal for unit0. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
pub type CH1_POS_MODE_R = crate::FieldReader;
#[doc = "Field `CH1_POS_MODE` writer - This register is used to control the mode of channel1's input posedge signal for unit0. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
pub type CH1_POS_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1_HCTRL_MODE` reader - This register is used to control the mode of channel1's high control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH1_HCTRL_MODE_R = crate::FieldReader;
#[doc = "Field `CH1_HCTRL_MODE` writer - This register is used to control the mode of channel1's high control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH1_HCTRL_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1_LCTRL_MODE` reader - This register is used to control the mode of channel1's low control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH1_LCTRL_MODE_R = crate::FieldReader;
#[doc = "Field `CH1_LCTRL_MODE` writer - This register is used to control the mode of channel1's low control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub type CH1_LCTRL_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:9 - This register is used to filter pluse whose width is smaller than this value for unit0."]
    #[inline(always)]
    #[must_use]
    pub fn filter_thres(&mut self) -> FILTER_THRES_W<U_CONF0_SPEC> {
        FILTER_THRES_W::new(self, 0)
    }
    #[doc = "Bit 10 - This is the enable bit for filtering input signals for unit0."]
    #[inline(always)]
    #[must_use]
    pub fn filter_en(&mut self) -> FILTER_EN_W<U_CONF0_SPEC> {
        FILTER_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - This is the enable bit for comparing unit0's count with 0 value."]
    #[inline(always)]
    #[must_use]
    pub fn thr_zero_en(&mut self) -> THR_ZERO_EN_W<U_CONF0_SPEC> {
        THR_ZERO_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - This is the enable bit for comparing unit0's count with thr_h_lim value."]
    #[inline(always)]
    #[must_use]
    pub fn thr_h_lim_en(&mut self) -> THR_H_LIM_EN_W<U_CONF0_SPEC> {
        THR_H_LIM_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - This is the enable bit for comparing unit0's count with thr_l_lim value."]
    #[inline(always)]
    #[must_use]
    pub fn thr_l_lim_en(&mut self) -> THR_L_LIM_EN_W<U_CONF0_SPEC> {
        THR_L_LIM_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - This is the enable bit for comparing unit0's count with thres0 value."]
    #[inline(always)]
    #[must_use]
    pub fn thr_thres0_en(&mut self) -> THR_THRES0_EN_W<U_CONF0_SPEC> {
        THR_THRES0_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - This is the enable bit for comparing unit0's count with thres1 value ."]
    #[inline(always)]
    #[must_use]
    pub fn thr_thres1_en(&mut self) -> THR_THRES1_EN_W<U_CONF0_SPEC> {
        THR_THRES1_EN_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - This register is used to control the mode of channel0's input negedge signal for unit0. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_neg_mode(&mut self) -> CH0_NEG_MODE_W<U_CONF0_SPEC> {
        CH0_NEG_MODE_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - This register is used to control the mode of channel0's input posedge signal for unit0. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_pos_mode(&mut self) -> CH0_POS_MODE_W<U_CONF0_SPEC> {
        CH0_POS_MODE_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - This register is used to control the mode of channel0's high control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_hctrl_mode(&mut self) -> CH0_HCTRL_MODE_W<U_CONF0_SPEC> {
        CH0_HCTRL_MODE_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - This register is used to control the mode of channel0's low control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_lctrl_mode(&mut self) -> CH0_LCTRL_MODE_W<U_CONF0_SPEC> {
        CH0_LCTRL_MODE_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - This register is used to control the mode of channel1's input negedge signal for unit0. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_neg_mode(&mut self) -> CH1_NEG_MODE_W<U_CONF0_SPEC> {
        CH1_NEG_MODE_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - This register is used to control the mode of channel1's input posedge signal for unit0. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_pos_mode(&mut self) -> CH1_POS_MODE_W<U_CONF0_SPEC> {
        CH1_POS_MODE_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - This register is used to control the mode of channel1's high control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_hctrl_mode(&mut self) -> CH1_HCTRL_MODE_W<U_CONF0_SPEC> {
        CH1_HCTRL_MODE_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - This register is used to control the mode of channel1's low control signal for unit0. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_lctrl_mode(&mut self) -> CH1_LCTRL_MODE_W<U_CONF0_SPEC> {
        CH1_LCTRL_MODE_W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`u_conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`u_conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U_CONF0_SPEC;
impl crate::RegisterSpec for U_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`u_conf0::R`](R) reader structure"]
impl crate::Readable for U_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`u_conf0::W`](W) writer structure"]
impl crate::Writable for U_CONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets U%s_CONF0 to value 0x3c10"]
impl crate::Resettable for U_CONF0_SPEC {
    const RESET_VALUE: u32 = 0x3c10;
}
