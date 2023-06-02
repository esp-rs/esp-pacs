#[doc = "Register `EVT_TASK_EN0` reader"]
pub struct R(crate::R<EVT_TASK_EN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVT_TASK_EN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVT_TASK_EN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVT_TASK_EN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVT_TASK_EN0` writer"]
pub struct W(crate::W<EVT_TASK_EN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVT_TASK_EN0_SPEC>;
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
impl From<crate::W<EVT_TASK_EN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVT_TASK_EN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVT_DUTY_CHNG_END_CH0_EN` reader - Ledc ch0 duty change end event enable register, write 1 to enable this event."]
pub type EVT_DUTY_CHNG_END_CH0_EN_R = crate::BitReader;
#[doc = "Field `EVT_DUTY_CHNG_END_CH0_EN` writer - Ledc ch0 duty change end event enable register, write 1 to enable this event."]
pub type EVT_DUTY_CHNG_END_CH0_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `EVT_DUTY_CHNG_END_CH1_EN` reader - Ledc ch1 duty change end event enable register, write 1 to enable this event."]
pub type EVT_DUTY_CHNG_END_CH1_EN_R = crate::BitReader;
#[doc = "Field `EVT_DUTY_CHNG_END_CH1_EN` writer - Ledc ch1 duty change end event enable register, write 1 to enable this event."]
pub type EVT_DUTY_CHNG_END_CH1_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `EVT_DUTY_CHNG_END_CH2_EN` reader - Ledc ch2 duty change end event enable register, write 1 to enable this event."]
pub type EVT_DUTY_CHNG_END_CH2_EN_R = crate::BitReader;
#[doc = "Field `EVT_DUTY_CHNG_END_CH2_EN` writer - Ledc ch2 duty change end event enable register, write 1 to enable this event."]
pub type EVT_DUTY_CHNG_END_CH2_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `EVT_DUTY_CHNG_END_CH3_EN` reader - Ledc ch3 duty change end event enable register, write 1 to enable this event."]
pub type EVT_DUTY_CHNG_END_CH3_EN_R = crate::BitReader;
#[doc = "Field `EVT_DUTY_CHNG_END_CH3_EN` writer - Ledc ch3 duty change end event enable register, write 1 to enable this event."]
pub type EVT_DUTY_CHNG_END_CH3_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `EVT_DUTY_CHNG_END_CH4_EN` reader - Ledc ch4 duty change end event enable register, write 1 to enable this event."]
pub type EVT_DUTY_CHNG_END_CH4_EN_R = crate::BitReader;
#[doc = "Field `EVT_DUTY_CHNG_END_CH4_EN` writer - Ledc ch4 duty change end event enable register, write 1 to enable this event."]
pub type EVT_DUTY_CHNG_END_CH4_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `EVT_DUTY_CHNG_END_CH5_EN` reader - Ledc ch5 duty change end event enable register, write 1 to enable this event."]
pub type EVT_DUTY_CHNG_END_CH5_EN_R = crate::BitReader;
#[doc = "Field `EVT_DUTY_CHNG_END_CH5_EN` writer - Ledc ch5 duty change end event enable register, write 1 to enable this event."]
pub type EVT_DUTY_CHNG_END_CH5_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `EVT_OVF_CNT_PLS_CH0_EN` reader - Ledc ch0 overflow count pulse event enable register, write 1 to enable this event."]
pub type EVT_OVF_CNT_PLS_CH0_EN_R = crate::BitReader;
#[doc = "Field `EVT_OVF_CNT_PLS_CH0_EN` writer - Ledc ch0 overflow count pulse event enable register, write 1 to enable this event."]
pub type EVT_OVF_CNT_PLS_CH0_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `EVT_OVF_CNT_PLS_CH1_EN` reader - Ledc ch1 overflow count pulse event enable register, write 1 to enable this event."]
pub type EVT_OVF_CNT_PLS_CH1_EN_R = crate::BitReader;
#[doc = "Field `EVT_OVF_CNT_PLS_CH1_EN` writer - Ledc ch1 overflow count pulse event enable register, write 1 to enable this event."]
pub type EVT_OVF_CNT_PLS_CH1_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `EVT_OVF_CNT_PLS_CH2_EN` reader - Ledc ch2 overflow count pulse event enable register, write 1 to enable this event."]
pub type EVT_OVF_CNT_PLS_CH2_EN_R = crate::BitReader;
#[doc = "Field `EVT_OVF_CNT_PLS_CH2_EN` writer - Ledc ch2 overflow count pulse event enable register, write 1 to enable this event."]
pub type EVT_OVF_CNT_PLS_CH2_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `EVT_OVF_CNT_PLS_CH3_EN` reader - Ledc ch3 overflow count pulse event enable register, write 1 to enable this event."]
pub type EVT_OVF_CNT_PLS_CH3_EN_R = crate::BitReader;
#[doc = "Field `EVT_OVF_CNT_PLS_CH3_EN` writer - Ledc ch3 overflow count pulse event enable register, write 1 to enable this event."]
pub type EVT_OVF_CNT_PLS_CH3_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `EVT_OVF_CNT_PLS_CH4_EN` reader - Ledc ch4 overflow count pulse event enable register, write 1 to enable this event."]
pub type EVT_OVF_CNT_PLS_CH4_EN_R = crate::BitReader;
#[doc = "Field `EVT_OVF_CNT_PLS_CH4_EN` writer - Ledc ch4 overflow count pulse event enable register, write 1 to enable this event."]
pub type EVT_OVF_CNT_PLS_CH4_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `EVT_OVF_CNT_PLS_CH5_EN` reader - Ledc ch5 overflow count pulse event enable register, write 1 to enable this event."]
pub type EVT_OVF_CNT_PLS_CH5_EN_R = crate::BitReader;
#[doc = "Field `EVT_OVF_CNT_PLS_CH5_EN` writer - Ledc ch5 overflow count pulse event enable register, write 1 to enable this event."]
pub type EVT_OVF_CNT_PLS_CH5_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `EVT_TIME_OVF_TIMER0_EN` reader - Ledc timer0 overflow event enable register, write 1 to enable this event."]
pub type EVT_TIME_OVF_TIMER0_EN_R = crate::BitReader;
#[doc = "Field `EVT_TIME_OVF_TIMER0_EN` writer - Ledc timer0 overflow event enable register, write 1 to enable this event."]
pub type EVT_TIME_OVF_TIMER0_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `EVT_TIME_OVF_TIMER1_EN` reader - Ledc timer1 overflow event enable register, write 1 to enable this event."]
pub type EVT_TIME_OVF_TIMER1_EN_R = crate::BitReader;
#[doc = "Field `EVT_TIME_OVF_TIMER1_EN` writer - Ledc timer1 overflow event enable register, write 1 to enable this event."]
pub type EVT_TIME_OVF_TIMER1_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `EVT_TIME_OVF_TIMER2_EN` reader - Ledc timer2 overflow event enable register, write 1 to enable this event."]
pub type EVT_TIME_OVF_TIMER2_EN_R = crate::BitReader;
#[doc = "Field `EVT_TIME_OVF_TIMER2_EN` writer - Ledc timer2 overflow event enable register, write 1 to enable this event."]
pub type EVT_TIME_OVF_TIMER2_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `EVT_TIME_OVF_TIMER3_EN` reader - Ledc timer3 overflow event enable register, write 1 to enable this event."]
pub type EVT_TIME_OVF_TIMER3_EN_R = crate::BitReader;
#[doc = "Field `EVT_TIME_OVF_TIMER3_EN` writer - Ledc timer3 overflow event enable register, write 1 to enable this event."]
pub type EVT_TIME_OVF_TIMER3_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `EVT_TIME0_CMP_EN` reader - Ledc timer0 compare event enable register, write 1 to enable this event."]
pub type EVT_TIME0_CMP_EN_R = crate::BitReader;
#[doc = "Field `EVT_TIME0_CMP_EN` writer - Ledc timer0 compare event enable register, write 1 to enable this event."]
pub type EVT_TIME0_CMP_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `EVT_TIME1_CMP_EN` reader - Ledc timer1 compare event enable register, write 1 to enable this event."]
pub type EVT_TIME1_CMP_EN_R = crate::BitReader;
#[doc = "Field `EVT_TIME1_CMP_EN` writer - Ledc timer1 compare event enable register, write 1 to enable this event."]
pub type EVT_TIME1_CMP_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `EVT_TIME2_CMP_EN` reader - Ledc timer2 compare event enable register, write 1 to enable this event."]
pub type EVT_TIME2_CMP_EN_R = crate::BitReader;
#[doc = "Field `EVT_TIME2_CMP_EN` writer - Ledc timer2 compare event enable register, write 1 to enable this event."]
pub type EVT_TIME2_CMP_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `EVT_TIME3_CMP_EN` reader - Ledc timer3 compare event enable register, write 1 to enable this event."]
pub type EVT_TIME3_CMP_EN_R = crate::BitReader;
#[doc = "Field `EVT_TIME3_CMP_EN` writer - Ledc timer3 compare event enable register, write 1 to enable this event."]
pub type EVT_TIME3_CMP_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `TASK_DUTY_SCALE_UPDATE_CH0_EN` reader - Ledc ch0 duty scale update task enable register, write 1 to enable this task."]
pub type TASK_DUTY_SCALE_UPDATE_CH0_EN_R = crate::BitReader;
#[doc = "Field `TASK_DUTY_SCALE_UPDATE_CH0_EN` writer - Ledc ch0 duty scale update task enable register, write 1 to enable this task."]
pub type TASK_DUTY_SCALE_UPDATE_CH0_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `TASK_DUTY_SCALE_UPDATE_CH1_EN` reader - Ledc ch1 duty scale update task enable register, write 1 to enable this task."]
pub type TASK_DUTY_SCALE_UPDATE_CH1_EN_R = crate::BitReader;
#[doc = "Field `TASK_DUTY_SCALE_UPDATE_CH1_EN` writer - Ledc ch1 duty scale update task enable register, write 1 to enable this task."]
pub type TASK_DUTY_SCALE_UPDATE_CH1_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `TASK_DUTY_SCALE_UPDATE_CH2_EN` reader - Ledc ch2 duty scale update task enable register, write 1 to enable this task."]
pub type TASK_DUTY_SCALE_UPDATE_CH2_EN_R = crate::BitReader;
#[doc = "Field `TASK_DUTY_SCALE_UPDATE_CH2_EN` writer - Ledc ch2 duty scale update task enable register, write 1 to enable this task."]
pub type TASK_DUTY_SCALE_UPDATE_CH2_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `TASK_DUTY_SCALE_UPDATE_CH3_EN` reader - Ledc ch3 duty scale update task enable register, write 1 to enable this task."]
pub type TASK_DUTY_SCALE_UPDATE_CH3_EN_R = crate::BitReader;
#[doc = "Field `TASK_DUTY_SCALE_UPDATE_CH3_EN` writer - Ledc ch3 duty scale update task enable register, write 1 to enable this task."]
pub type TASK_DUTY_SCALE_UPDATE_CH3_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `TASK_DUTY_SCALE_UPDATE_CH4_EN` reader - Ledc ch4 duty scale update task enable register, write 1 to enable this task."]
pub type TASK_DUTY_SCALE_UPDATE_CH4_EN_R = crate::BitReader;
#[doc = "Field `TASK_DUTY_SCALE_UPDATE_CH4_EN` writer - Ledc ch4 duty scale update task enable register, write 1 to enable this task."]
pub type TASK_DUTY_SCALE_UPDATE_CH4_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
#[doc = "Field `TASK_DUTY_SCALE_UPDATE_CH5_EN` reader - Ledc ch5 duty scale update task enable register, write 1 to enable this task."]
pub type TASK_DUTY_SCALE_UPDATE_CH5_EN_R = crate::BitReader;
#[doc = "Field `TASK_DUTY_SCALE_UPDATE_CH5_EN` writer - Ledc ch5 duty scale update task enable register, write 1 to enable this task."]
pub type TASK_DUTY_SCALE_UPDATE_CH5_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, EVT_TASK_EN0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Ledc ch0 duty change end event enable register, write 1 to enable this event."]
    #[inline(always)]
    pub fn evt_duty_chng_end_ch0_en(&self) -> EVT_DUTY_CHNG_END_CH0_EN_R {
        EVT_DUTY_CHNG_END_CH0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ledc ch1 duty change end event enable register, write 1 to enable this event."]
    #[inline(always)]
    pub fn evt_duty_chng_end_ch1_en(&self) -> EVT_DUTY_CHNG_END_CH1_EN_R {
        EVT_DUTY_CHNG_END_CH1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Ledc ch2 duty change end event enable register, write 1 to enable this event."]
    #[inline(always)]
    pub fn evt_duty_chng_end_ch2_en(&self) -> EVT_DUTY_CHNG_END_CH2_EN_R {
        EVT_DUTY_CHNG_END_CH2_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Ledc ch3 duty change end event enable register, write 1 to enable this event."]
    #[inline(always)]
    pub fn evt_duty_chng_end_ch3_en(&self) -> EVT_DUTY_CHNG_END_CH3_EN_R {
        EVT_DUTY_CHNG_END_CH3_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Ledc ch4 duty change end event enable register, write 1 to enable this event."]
    #[inline(always)]
    pub fn evt_duty_chng_end_ch4_en(&self) -> EVT_DUTY_CHNG_END_CH4_EN_R {
        EVT_DUTY_CHNG_END_CH4_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Ledc ch5 duty change end event enable register, write 1 to enable this event."]
    #[inline(always)]
    pub fn evt_duty_chng_end_ch5_en(&self) -> EVT_DUTY_CHNG_END_CH5_EN_R {
        EVT_DUTY_CHNG_END_CH5_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Ledc ch0 overflow count pulse event enable register, write 1 to enable this event."]
    #[inline(always)]
    pub fn evt_ovf_cnt_pls_ch0_en(&self) -> EVT_OVF_CNT_PLS_CH0_EN_R {
        EVT_OVF_CNT_PLS_CH0_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Ledc ch1 overflow count pulse event enable register, write 1 to enable this event."]
    #[inline(always)]
    pub fn evt_ovf_cnt_pls_ch1_en(&self) -> EVT_OVF_CNT_PLS_CH1_EN_R {
        EVT_OVF_CNT_PLS_CH1_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Ledc ch2 overflow count pulse event enable register, write 1 to enable this event."]
    #[inline(always)]
    pub fn evt_ovf_cnt_pls_ch2_en(&self) -> EVT_OVF_CNT_PLS_CH2_EN_R {
        EVT_OVF_CNT_PLS_CH2_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Ledc ch3 overflow count pulse event enable register, write 1 to enable this event."]
    #[inline(always)]
    pub fn evt_ovf_cnt_pls_ch3_en(&self) -> EVT_OVF_CNT_PLS_CH3_EN_R {
        EVT_OVF_CNT_PLS_CH3_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Ledc ch4 overflow count pulse event enable register, write 1 to enable this event."]
    #[inline(always)]
    pub fn evt_ovf_cnt_pls_ch4_en(&self) -> EVT_OVF_CNT_PLS_CH4_EN_R {
        EVT_OVF_CNT_PLS_CH4_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Ledc ch5 overflow count pulse event enable register, write 1 to enable this event."]
    #[inline(always)]
    pub fn evt_ovf_cnt_pls_ch5_en(&self) -> EVT_OVF_CNT_PLS_CH5_EN_R {
        EVT_OVF_CNT_PLS_CH5_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Ledc timer0 overflow event enable register, write 1 to enable this event."]
    #[inline(always)]
    pub fn evt_time_ovf_timer0_en(&self) -> EVT_TIME_OVF_TIMER0_EN_R {
        EVT_TIME_OVF_TIMER0_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Ledc timer1 overflow event enable register, write 1 to enable this event."]
    #[inline(always)]
    pub fn evt_time_ovf_timer1_en(&self) -> EVT_TIME_OVF_TIMER1_EN_R {
        EVT_TIME_OVF_TIMER1_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Ledc timer2 overflow event enable register, write 1 to enable this event."]
    #[inline(always)]
    pub fn evt_time_ovf_timer2_en(&self) -> EVT_TIME_OVF_TIMER2_EN_R {
        EVT_TIME_OVF_TIMER2_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Ledc timer3 overflow event enable register, write 1 to enable this event."]
    #[inline(always)]
    pub fn evt_time_ovf_timer3_en(&self) -> EVT_TIME_OVF_TIMER3_EN_R {
        EVT_TIME_OVF_TIMER3_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Ledc timer0 compare event enable register, write 1 to enable this event."]
    #[inline(always)]
    pub fn evt_time0_cmp_en(&self) -> EVT_TIME0_CMP_EN_R {
        EVT_TIME0_CMP_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Ledc timer1 compare event enable register, write 1 to enable this event."]
    #[inline(always)]
    pub fn evt_time1_cmp_en(&self) -> EVT_TIME1_CMP_EN_R {
        EVT_TIME1_CMP_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Ledc timer2 compare event enable register, write 1 to enable this event."]
    #[inline(always)]
    pub fn evt_time2_cmp_en(&self) -> EVT_TIME2_CMP_EN_R {
        EVT_TIME2_CMP_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Ledc timer3 compare event enable register, write 1 to enable this event."]
    #[inline(always)]
    pub fn evt_time3_cmp_en(&self) -> EVT_TIME3_CMP_EN_R {
        EVT_TIME3_CMP_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Ledc ch0 duty scale update task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_duty_scale_update_ch0_en(&self) -> TASK_DUTY_SCALE_UPDATE_CH0_EN_R {
        TASK_DUTY_SCALE_UPDATE_CH0_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Ledc ch1 duty scale update task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_duty_scale_update_ch1_en(&self) -> TASK_DUTY_SCALE_UPDATE_CH1_EN_R {
        TASK_DUTY_SCALE_UPDATE_CH1_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Ledc ch2 duty scale update task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_duty_scale_update_ch2_en(&self) -> TASK_DUTY_SCALE_UPDATE_CH2_EN_R {
        TASK_DUTY_SCALE_UPDATE_CH2_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Ledc ch3 duty scale update task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_duty_scale_update_ch3_en(&self) -> TASK_DUTY_SCALE_UPDATE_CH3_EN_R {
        TASK_DUTY_SCALE_UPDATE_CH3_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Ledc ch4 duty scale update task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_duty_scale_update_ch4_en(&self) -> TASK_DUTY_SCALE_UPDATE_CH4_EN_R {
        TASK_DUTY_SCALE_UPDATE_CH4_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Ledc ch5 duty scale update task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_duty_scale_update_ch5_en(&self) -> TASK_DUTY_SCALE_UPDATE_CH5_EN_R {
        TASK_DUTY_SCALE_UPDATE_CH5_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVT_TASK_EN0")
            .field(
                "evt_duty_chng_end_ch0_en",
                &format_args!("{}", self.evt_duty_chng_end_ch0_en().bit()),
            )
            .field(
                "evt_duty_chng_end_ch1_en",
                &format_args!("{}", self.evt_duty_chng_end_ch1_en().bit()),
            )
            .field(
                "evt_duty_chng_end_ch2_en",
                &format_args!("{}", self.evt_duty_chng_end_ch2_en().bit()),
            )
            .field(
                "evt_duty_chng_end_ch3_en",
                &format_args!("{}", self.evt_duty_chng_end_ch3_en().bit()),
            )
            .field(
                "evt_duty_chng_end_ch4_en",
                &format_args!("{}", self.evt_duty_chng_end_ch4_en().bit()),
            )
            .field(
                "evt_duty_chng_end_ch5_en",
                &format_args!("{}", self.evt_duty_chng_end_ch5_en().bit()),
            )
            .field(
                "evt_ovf_cnt_pls_ch0_en",
                &format_args!("{}", self.evt_ovf_cnt_pls_ch0_en().bit()),
            )
            .field(
                "evt_ovf_cnt_pls_ch1_en",
                &format_args!("{}", self.evt_ovf_cnt_pls_ch1_en().bit()),
            )
            .field(
                "evt_ovf_cnt_pls_ch2_en",
                &format_args!("{}", self.evt_ovf_cnt_pls_ch2_en().bit()),
            )
            .field(
                "evt_ovf_cnt_pls_ch3_en",
                &format_args!("{}", self.evt_ovf_cnt_pls_ch3_en().bit()),
            )
            .field(
                "evt_ovf_cnt_pls_ch4_en",
                &format_args!("{}", self.evt_ovf_cnt_pls_ch4_en().bit()),
            )
            .field(
                "evt_ovf_cnt_pls_ch5_en",
                &format_args!("{}", self.evt_ovf_cnt_pls_ch5_en().bit()),
            )
            .field(
                "evt_time_ovf_timer0_en",
                &format_args!("{}", self.evt_time_ovf_timer0_en().bit()),
            )
            .field(
                "evt_time_ovf_timer1_en",
                &format_args!("{}", self.evt_time_ovf_timer1_en().bit()),
            )
            .field(
                "evt_time_ovf_timer2_en",
                &format_args!("{}", self.evt_time_ovf_timer2_en().bit()),
            )
            .field(
                "evt_time_ovf_timer3_en",
                &format_args!("{}", self.evt_time_ovf_timer3_en().bit()),
            )
            .field(
                "evt_time0_cmp_en",
                &format_args!("{}", self.evt_time0_cmp_en().bit()),
            )
            .field(
                "evt_time1_cmp_en",
                &format_args!("{}", self.evt_time1_cmp_en().bit()),
            )
            .field(
                "evt_time2_cmp_en",
                &format_args!("{}", self.evt_time2_cmp_en().bit()),
            )
            .field(
                "evt_time3_cmp_en",
                &format_args!("{}", self.evt_time3_cmp_en().bit()),
            )
            .field(
                "task_duty_scale_update_ch0_en",
                &format_args!("{}", self.task_duty_scale_update_ch0_en().bit()),
            )
            .field(
                "task_duty_scale_update_ch1_en",
                &format_args!("{}", self.task_duty_scale_update_ch1_en().bit()),
            )
            .field(
                "task_duty_scale_update_ch2_en",
                &format_args!("{}", self.task_duty_scale_update_ch2_en().bit()),
            )
            .field(
                "task_duty_scale_update_ch3_en",
                &format_args!("{}", self.task_duty_scale_update_ch3_en().bit()),
            )
            .field(
                "task_duty_scale_update_ch4_en",
                &format_args!("{}", self.task_duty_scale_update_ch4_en().bit()),
            )
            .field(
                "task_duty_scale_update_ch5_en",
                &format_args!("{}", self.task_duty_scale_update_ch5_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EVT_TASK_EN0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Ledc ch0 duty change end event enable register, write 1 to enable this event."]
    #[inline(always)]
    #[must_use]
    pub fn evt_duty_chng_end_ch0_en(&mut self) -> EVT_DUTY_CHNG_END_CH0_EN_W<0> {
        EVT_DUTY_CHNG_END_CH0_EN_W::new(self)
    }
    #[doc = "Bit 1 - Ledc ch1 duty change end event enable register, write 1 to enable this event."]
    #[inline(always)]
    #[must_use]
    pub fn evt_duty_chng_end_ch1_en(&mut self) -> EVT_DUTY_CHNG_END_CH1_EN_W<1> {
        EVT_DUTY_CHNG_END_CH1_EN_W::new(self)
    }
    #[doc = "Bit 2 - Ledc ch2 duty change end event enable register, write 1 to enable this event."]
    #[inline(always)]
    #[must_use]
    pub fn evt_duty_chng_end_ch2_en(&mut self) -> EVT_DUTY_CHNG_END_CH2_EN_W<2> {
        EVT_DUTY_CHNG_END_CH2_EN_W::new(self)
    }
    #[doc = "Bit 3 - Ledc ch3 duty change end event enable register, write 1 to enable this event."]
    #[inline(always)]
    #[must_use]
    pub fn evt_duty_chng_end_ch3_en(&mut self) -> EVT_DUTY_CHNG_END_CH3_EN_W<3> {
        EVT_DUTY_CHNG_END_CH3_EN_W::new(self)
    }
    #[doc = "Bit 4 - Ledc ch4 duty change end event enable register, write 1 to enable this event."]
    #[inline(always)]
    #[must_use]
    pub fn evt_duty_chng_end_ch4_en(&mut self) -> EVT_DUTY_CHNG_END_CH4_EN_W<4> {
        EVT_DUTY_CHNG_END_CH4_EN_W::new(self)
    }
    #[doc = "Bit 5 - Ledc ch5 duty change end event enable register, write 1 to enable this event."]
    #[inline(always)]
    #[must_use]
    pub fn evt_duty_chng_end_ch5_en(&mut self) -> EVT_DUTY_CHNG_END_CH5_EN_W<5> {
        EVT_DUTY_CHNG_END_CH5_EN_W::new(self)
    }
    #[doc = "Bit 8 - Ledc ch0 overflow count pulse event enable register, write 1 to enable this event."]
    #[inline(always)]
    #[must_use]
    pub fn evt_ovf_cnt_pls_ch0_en(&mut self) -> EVT_OVF_CNT_PLS_CH0_EN_W<8> {
        EVT_OVF_CNT_PLS_CH0_EN_W::new(self)
    }
    #[doc = "Bit 9 - Ledc ch1 overflow count pulse event enable register, write 1 to enable this event."]
    #[inline(always)]
    #[must_use]
    pub fn evt_ovf_cnt_pls_ch1_en(&mut self) -> EVT_OVF_CNT_PLS_CH1_EN_W<9> {
        EVT_OVF_CNT_PLS_CH1_EN_W::new(self)
    }
    #[doc = "Bit 10 - Ledc ch2 overflow count pulse event enable register, write 1 to enable this event."]
    #[inline(always)]
    #[must_use]
    pub fn evt_ovf_cnt_pls_ch2_en(&mut self) -> EVT_OVF_CNT_PLS_CH2_EN_W<10> {
        EVT_OVF_CNT_PLS_CH2_EN_W::new(self)
    }
    #[doc = "Bit 11 - Ledc ch3 overflow count pulse event enable register, write 1 to enable this event."]
    #[inline(always)]
    #[must_use]
    pub fn evt_ovf_cnt_pls_ch3_en(&mut self) -> EVT_OVF_CNT_PLS_CH3_EN_W<11> {
        EVT_OVF_CNT_PLS_CH3_EN_W::new(self)
    }
    #[doc = "Bit 12 - Ledc ch4 overflow count pulse event enable register, write 1 to enable this event."]
    #[inline(always)]
    #[must_use]
    pub fn evt_ovf_cnt_pls_ch4_en(&mut self) -> EVT_OVF_CNT_PLS_CH4_EN_W<12> {
        EVT_OVF_CNT_PLS_CH4_EN_W::new(self)
    }
    #[doc = "Bit 13 - Ledc ch5 overflow count pulse event enable register, write 1 to enable this event."]
    #[inline(always)]
    #[must_use]
    pub fn evt_ovf_cnt_pls_ch5_en(&mut self) -> EVT_OVF_CNT_PLS_CH5_EN_W<13> {
        EVT_OVF_CNT_PLS_CH5_EN_W::new(self)
    }
    #[doc = "Bit 16 - Ledc timer0 overflow event enable register, write 1 to enable this event."]
    #[inline(always)]
    #[must_use]
    pub fn evt_time_ovf_timer0_en(&mut self) -> EVT_TIME_OVF_TIMER0_EN_W<16> {
        EVT_TIME_OVF_TIMER0_EN_W::new(self)
    }
    #[doc = "Bit 17 - Ledc timer1 overflow event enable register, write 1 to enable this event."]
    #[inline(always)]
    #[must_use]
    pub fn evt_time_ovf_timer1_en(&mut self) -> EVT_TIME_OVF_TIMER1_EN_W<17> {
        EVT_TIME_OVF_TIMER1_EN_W::new(self)
    }
    #[doc = "Bit 18 - Ledc timer2 overflow event enable register, write 1 to enable this event."]
    #[inline(always)]
    #[must_use]
    pub fn evt_time_ovf_timer2_en(&mut self) -> EVT_TIME_OVF_TIMER2_EN_W<18> {
        EVT_TIME_OVF_TIMER2_EN_W::new(self)
    }
    #[doc = "Bit 19 - Ledc timer3 overflow event enable register, write 1 to enable this event."]
    #[inline(always)]
    #[must_use]
    pub fn evt_time_ovf_timer3_en(&mut self) -> EVT_TIME_OVF_TIMER3_EN_W<19> {
        EVT_TIME_OVF_TIMER3_EN_W::new(self)
    }
    #[doc = "Bit 20 - Ledc timer0 compare event enable register, write 1 to enable this event."]
    #[inline(always)]
    #[must_use]
    pub fn evt_time0_cmp_en(&mut self) -> EVT_TIME0_CMP_EN_W<20> {
        EVT_TIME0_CMP_EN_W::new(self)
    }
    #[doc = "Bit 21 - Ledc timer1 compare event enable register, write 1 to enable this event."]
    #[inline(always)]
    #[must_use]
    pub fn evt_time1_cmp_en(&mut self) -> EVT_TIME1_CMP_EN_W<21> {
        EVT_TIME1_CMP_EN_W::new(self)
    }
    #[doc = "Bit 22 - Ledc timer2 compare event enable register, write 1 to enable this event."]
    #[inline(always)]
    #[must_use]
    pub fn evt_time2_cmp_en(&mut self) -> EVT_TIME2_CMP_EN_W<22> {
        EVT_TIME2_CMP_EN_W::new(self)
    }
    #[doc = "Bit 23 - Ledc timer3 compare event enable register, write 1 to enable this event."]
    #[inline(always)]
    #[must_use]
    pub fn evt_time3_cmp_en(&mut self) -> EVT_TIME3_CMP_EN_W<23> {
        EVT_TIME3_CMP_EN_W::new(self)
    }
    #[doc = "Bit 24 - Ledc ch0 duty scale update task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_duty_scale_update_ch0_en(&mut self) -> TASK_DUTY_SCALE_UPDATE_CH0_EN_W<24> {
        TASK_DUTY_SCALE_UPDATE_CH0_EN_W::new(self)
    }
    #[doc = "Bit 25 - Ledc ch1 duty scale update task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_duty_scale_update_ch1_en(&mut self) -> TASK_DUTY_SCALE_UPDATE_CH1_EN_W<25> {
        TASK_DUTY_SCALE_UPDATE_CH1_EN_W::new(self)
    }
    #[doc = "Bit 26 - Ledc ch2 duty scale update task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_duty_scale_update_ch2_en(&mut self) -> TASK_DUTY_SCALE_UPDATE_CH2_EN_W<26> {
        TASK_DUTY_SCALE_UPDATE_CH2_EN_W::new(self)
    }
    #[doc = "Bit 27 - Ledc ch3 duty scale update task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_duty_scale_update_ch3_en(&mut self) -> TASK_DUTY_SCALE_UPDATE_CH3_EN_W<27> {
        TASK_DUTY_SCALE_UPDATE_CH3_EN_W::new(self)
    }
    #[doc = "Bit 28 - Ledc ch4 duty scale update task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_duty_scale_update_ch4_en(&mut self) -> TASK_DUTY_SCALE_UPDATE_CH4_EN_W<28> {
        TASK_DUTY_SCALE_UPDATE_CH4_EN_W::new(self)
    }
    #[doc = "Bit 29 - Ledc ch5 duty scale update task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_duty_scale_update_ch5_en(&mut self) -> TASK_DUTY_SCALE_UPDATE_CH5_EN_W<29> {
        TASK_DUTY_SCALE_UPDATE_CH5_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ledc event task enable bit register0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evt_task_en0](index.html) module"]
pub struct EVT_TASK_EN0_SPEC;
impl crate::RegisterSpec for EVT_TASK_EN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evt_task_en0::R](R) reader structure"]
impl crate::Readable for EVT_TASK_EN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evt_task_en0::W](W) writer structure"]
impl crate::Writable for EVT_TASK_EN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVT_TASK_EN0 to value 0"]
impl crate::Resettable for EVT_TASK_EN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
