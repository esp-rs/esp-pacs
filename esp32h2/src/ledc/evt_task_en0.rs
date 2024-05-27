///Register `EVT_TASK_EN0` reader
pub type R = crate::R<EVT_TASK_EN0_SPEC>;
///Register `EVT_TASK_EN0` writer
pub type W = crate::W<EVT_TASK_EN0_SPEC>;
///Field `EVT_DUTY_CHNG_END_CH0_EN` reader - Ledc ch0 duty change end event enable register, write 1 to enable this event.
pub type EVT_DUTY_CHNG_END_CH0_EN_R = crate::BitReader;
///Field `EVT_DUTY_CHNG_END_CH0_EN` writer - Ledc ch0 duty change end event enable register, write 1 to enable this event.
pub type EVT_DUTY_CHNG_END_CH0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVT_DUTY_CHNG_END_CH1_EN` reader - Ledc ch1 duty change end event enable register, write 1 to enable this event.
pub type EVT_DUTY_CHNG_END_CH1_EN_R = crate::BitReader;
///Field `EVT_DUTY_CHNG_END_CH1_EN` writer - Ledc ch1 duty change end event enable register, write 1 to enable this event.
pub type EVT_DUTY_CHNG_END_CH1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVT_DUTY_CHNG_END_CH2_EN` reader - Ledc ch2 duty change end event enable register, write 1 to enable this event.
pub type EVT_DUTY_CHNG_END_CH2_EN_R = crate::BitReader;
///Field `EVT_DUTY_CHNG_END_CH2_EN` writer - Ledc ch2 duty change end event enable register, write 1 to enable this event.
pub type EVT_DUTY_CHNG_END_CH2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVT_DUTY_CHNG_END_CH3_EN` reader - Ledc ch3 duty change end event enable register, write 1 to enable this event.
pub type EVT_DUTY_CHNG_END_CH3_EN_R = crate::BitReader;
///Field `EVT_DUTY_CHNG_END_CH3_EN` writer - Ledc ch3 duty change end event enable register, write 1 to enable this event.
pub type EVT_DUTY_CHNG_END_CH3_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVT_DUTY_CHNG_END_CH4_EN` reader - Ledc ch4 duty change end event enable register, write 1 to enable this event.
pub type EVT_DUTY_CHNG_END_CH4_EN_R = crate::BitReader;
///Field `EVT_DUTY_CHNG_END_CH4_EN` writer - Ledc ch4 duty change end event enable register, write 1 to enable this event.
pub type EVT_DUTY_CHNG_END_CH4_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVT_DUTY_CHNG_END_CH5_EN` reader - Ledc ch5 duty change end event enable register, write 1 to enable this event.
pub type EVT_DUTY_CHNG_END_CH5_EN_R = crate::BitReader;
///Field `EVT_DUTY_CHNG_END_CH5_EN` writer - Ledc ch5 duty change end event enable register, write 1 to enable this event.
pub type EVT_DUTY_CHNG_END_CH5_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVT_OVF_CNT_PLS_CH0_EN` reader - Ledc ch0 overflow count pulse event enable register, write 1 to enable this event.
pub type EVT_OVF_CNT_PLS_CH0_EN_R = crate::BitReader;
///Field `EVT_OVF_CNT_PLS_CH0_EN` writer - Ledc ch0 overflow count pulse event enable register, write 1 to enable this event.
pub type EVT_OVF_CNT_PLS_CH0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVT_OVF_CNT_PLS_CH1_EN` reader - Ledc ch1 overflow count pulse event enable register, write 1 to enable this event.
pub type EVT_OVF_CNT_PLS_CH1_EN_R = crate::BitReader;
///Field `EVT_OVF_CNT_PLS_CH1_EN` writer - Ledc ch1 overflow count pulse event enable register, write 1 to enable this event.
pub type EVT_OVF_CNT_PLS_CH1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVT_OVF_CNT_PLS_CH2_EN` reader - Ledc ch2 overflow count pulse event enable register, write 1 to enable this event.
pub type EVT_OVF_CNT_PLS_CH2_EN_R = crate::BitReader;
///Field `EVT_OVF_CNT_PLS_CH2_EN` writer - Ledc ch2 overflow count pulse event enable register, write 1 to enable this event.
pub type EVT_OVF_CNT_PLS_CH2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVT_OVF_CNT_PLS_CH3_EN` reader - Ledc ch3 overflow count pulse event enable register, write 1 to enable this event.
pub type EVT_OVF_CNT_PLS_CH3_EN_R = crate::BitReader;
///Field `EVT_OVF_CNT_PLS_CH3_EN` writer - Ledc ch3 overflow count pulse event enable register, write 1 to enable this event.
pub type EVT_OVF_CNT_PLS_CH3_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVT_OVF_CNT_PLS_CH4_EN` reader - Ledc ch4 overflow count pulse event enable register, write 1 to enable this event.
pub type EVT_OVF_CNT_PLS_CH4_EN_R = crate::BitReader;
///Field `EVT_OVF_CNT_PLS_CH4_EN` writer - Ledc ch4 overflow count pulse event enable register, write 1 to enable this event.
pub type EVT_OVF_CNT_PLS_CH4_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVT_OVF_CNT_PLS_CH5_EN` reader - Ledc ch5 overflow count pulse event enable register, write 1 to enable this event.
pub type EVT_OVF_CNT_PLS_CH5_EN_R = crate::BitReader;
///Field `EVT_OVF_CNT_PLS_CH5_EN` writer - Ledc ch5 overflow count pulse event enable register, write 1 to enable this event.
pub type EVT_OVF_CNT_PLS_CH5_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVT_TIME_OVF_TIMER0_EN` reader - Ledc timer0 overflow event enable register, write 1 to enable this event.
pub type EVT_TIME_OVF_TIMER0_EN_R = crate::BitReader;
///Field `EVT_TIME_OVF_TIMER0_EN` writer - Ledc timer0 overflow event enable register, write 1 to enable this event.
pub type EVT_TIME_OVF_TIMER0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVT_TIME_OVF_TIMER1_EN` reader - Ledc timer1 overflow event enable register, write 1 to enable this event.
pub type EVT_TIME_OVF_TIMER1_EN_R = crate::BitReader;
///Field `EVT_TIME_OVF_TIMER1_EN` writer - Ledc timer1 overflow event enable register, write 1 to enable this event.
pub type EVT_TIME_OVF_TIMER1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVT_TIME_OVF_TIMER2_EN` reader - Ledc timer2 overflow event enable register, write 1 to enable this event.
pub type EVT_TIME_OVF_TIMER2_EN_R = crate::BitReader;
///Field `EVT_TIME_OVF_TIMER2_EN` writer - Ledc timer2 overflow event enable register, write 1 to enable this event.
pub type EVT_TIME_OVF_TIMER2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVT_TIME_OVF_TIMER3_EN` reader - Ledc timer3 overflow event enable register, write 1 to enable this event.
pub type EVT_TIME_OVF_TIMER3_EN_R = crate::BitReader;
///Field `EVT_TIME_OVF_TIMER3_EN` writer - Ledc timer3 overflow event enable register, write 1 to enable this event.
pub type EVT_TIME_OVF_TIMER3_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVT_TIME0_CMP_EN` reader - Ledc timer0 compare event enable register, write 1 to enable this event.
pub type EVT_TIME0_CMP_EN_R = crate::BitReader;
///Field `EVT_TIME0_CMP_EN` writer - Ledc timer0 compare event enable register, write 1 to enable this event.
pub type EVT_TIME0_CMP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVT_TIME1_CMP_EN` reader - Ledc timer1 compare event enable register, write 1 to enable this event.
pub type EVT_TIME1_CMP_EN_R = crate::BitReader;
///Field `EVT_TIME1_CMP_EN` writer - Ledc timer1 compare event enable register, write 1 to enable this event.
pub type EVT_TIME1_CMP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVT_TIME2_CMP_EN` reader - Ledc timer2 compare event enable register, write 1 to enable this event.
pub type EVT_TIME2_CMP_EN_R = crate::BitReader;
///Field `EVT_TIME2_CMP_EN` writer - Ledc timer2 compare event enable register, write 1 to enable this event.
pub type EVT_TIME2_CMP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVT_TIME3_CMP_EN` reader - Ledc timer3 compare event enable register, write 1 to enable this event.
pub type EVT_TIME3_CMP_EN_R = crate::BitReader;
///Field `EVT_TIME3_CMP_EN` writer - Ledc timer3 compare event enable register, write 1 to enable this event.
pub type EVT_TIME3_CMP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_DUTY_SCALE_UPDATE_CH0_EN` reader - Ledc ch0 duty scale update task enable register, write 1 to enable this task.
pub type TASK_DUTY_SCALE_UPDATE_CH0_EN_R = crate::BitReader;
///Field `TASK_DUTY_SCALE_UPDATE_CH0_EN` writer - Ledc ch0 duty scale update task enable register, write 1 to enable this task.
pub type TASK_DUTY_SCALE_UPDATE_CH0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_DUTY_SCALE_UPDATE_CH1_EN` reader - Ledc ch1 duty scale update task enable register, write 1 to enable this task.
pub type TASK_DUTY_SCALE_UPDATE_CH1_EN_R = crate::BitReader;
///Field `TASK_DUTY_SCALE_UPDATE_CH1_EN` writer - Ledc ch1 duty scale update task enable register, write 1 to enable this task.
pub type TASK_DUTY_SCALE_UPDATE_CH1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_DUTY_SCALE_UPDATE_CH2_EN` reader - Ledc ch2 duty scale update task enable register, write 1 to enable this task.
pub type TASK_DUTY_SCALE_UPDATE_CH2_EN_R = crate::BitReader;
///Field `TASK_DUTY_SCALE_UPDATE_CH2_EN` writer - Ledc ch2 duty scale update task enable register, write 1 to enable this task.
pub type TASK_DUTY_SCALE_UPDATE_CH2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_DUTY_SCALE_UPDATE_CH3_EN` reader - Ledc ch3 duty scale update task enable register, write 1 to enable this task.
pub type TASK_DUTY_SCALE_UPDATE_CH3_EN_R = crate::BitReader;
///Field `TASK_DUTY_SCALE_UPDATE_CH3_EN` writer - Ledc ch3 duty scale update task enable register, write 1 to enable this task.
pub type TASK_DUTY_SCALE_UPDATE_CH3_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_DUTY_SCALE_UPDATE_CH4_EN` reader - Ledc ch4 duty scale update task enable register, write 1 to enable this task.
pub type TASK_DUTY_SCALE_UPDATE_CH4_EN_R = crate::BitReader;
///Field `TASK_DUTY_SCALE_UPDATE_CH4_EN` writer - Ledc ch4 duty scale update task enable register, write 1 to enable this task.
pub type TASK_DUTY_SCALE_UPDATE_CH4_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_DUTY_SCALE_UPDATE_CH5_EN` reader - Ledc ch5 duty scale update task enable register, write 1 to enable this task.
pub type TASK_DUTY_SCALE_UPDATE_CH5_EN_R = crate::BitReader;
///Field `TASK_DUTY_SCALE_UPDATE_CH5_EN` writer - Ledc ch5 duty scale update task enable register, write 1 to enable this task.
pub type TASK_DUTY_SCALE_UPDATE_CH5_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Ledc ch0 duty change end event enable register, write 1 to enable this event.
    #[inline(always)]
    pub fn evt_duty_chng_end_ch0_en(&self) -> EVT_DUTY_CHNG_END_CH0_EN_R {
        EVT_DUTY_CHNG_END_CH0_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Ledc ch1 duty change end event enable register, write 1 to enable this event.
    #[inline(always)]
    pub fn evt_duty_chng_end_ch1_en(&self) -> EVT_DUTY_CHNG_END_CH1_EN_R {
        EVT_DUTY_CHNG_END_CH1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Ledc ch2 duty change end event enable register, write 1 to enable this event.
    #[inline(always)]
    pub fn evt_duty_chng_end_ch2_en(&self) -> EVT_DUTY_CHNG_END_CH2_EN_R {
        EVT_DUTY_CHNG_END_CH2_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Ledc ch3 duty change end event enable register, write 1 to enable this event.
    #[inline(always)]
    pub fn evt_duty_chng_end_ch3_en(&self) -> EVT_DUTY_CHNG_END_CH3_EN_R {
        EVT_DUTY_CHNG_END_CH3_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Ledc ch4 duty change end event enable register, write 1 to enable this event.
    #[inline(always)]
    pub fn evt_duty_chng_end_ch4_en(&self) -> EVT_DUTY_CHNG_END_CH4_EN_R {
        EVT_DUTY_CHNG_END_CH4_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Ledc ch5 duty change end event enable register, write 1 to enable this event.
    #[inline(always)]
    pub fn evt_duty_chng_end_ch5_en(&self) -> EVT_DUTY_CHNG_END_CH5_EN_R {
        EVT_DUTY_CHNG_END_CH5_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Ledc ch0 overflow count pulse event enable register, write 1 to enable this event.
    #[inline(always)]
    pub fn evt_ovf_cnt_pls_ch0_en(&self) -> EVT_OVF_CNT_PLS_CH0_EN_R {
        EVT_OVF_CNT_PLS_CH0_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Ledc ch1 overflow count pulse event enable register, write 1 to enable this event.
    #[inline(always)]
    pub fn evt_ovf_cnt_pls_ch1_en(&self) -> EVT_OVF_CNT_PLS_CH1_EN_R {
        EVT_OVF_CNT_PLS_CH1_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Ledc ch2 overflow count pulse event enable register, write 1 to enable this event.
    #[inline(always)]
    pub fn evt_ovf_cnt_pls_ch2_en(&self) -> EVT_OVF_CNT_PLS_CH2_EN_R {
        EVT_OVF_CNT_PLS_CH2_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Ledc ch3 overflow count pulse event enable register, write 1 to enable this event.
    #[inline(always)]
    pub fn evt_ovf_cnt_pls_ch3_en(&self) -> EVT_OVF_CNT_PLS_CH3_EN_R {
        EVT_OVF_CNT_PLS_CH3_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Ledc ch4 overflow count pulse event enable register, write 1 to enable this event.
    #[inline(always)]
    pub fn evt_ovf_cnt_pls_ch4_en(&self) -> EVT_OVF_CNT_PLS_CH4_EN_R {
        EVT_OVF_CNT_PLS_CH4_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Ledc ch5 overflow count pulse event enable register, write 1 to enable this event.
    #[inline(always)]
    pub fn evt_ovf_cnt_pls_ch5_en(&self) -> EVT_OVF_CNT_PLS_CH5_EN_R {
        EVT_OVF_CNT_PLS_CH5_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - Ledc timer0 overflow event enable register, write 1 to enable this event.
    #[inline(always)]
    pub fn evt_time_ovf_timer0_en(&self) -> EVT_TIME_OVF_TIMER0_EN_R {
        EVT_TIME_OVF_TIMER0_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Ledc timer1 overflow event enable register, write 1 to enable this event.
    #[inline(always)]
    pub fn evt_time_ovf_timer1_en(&self) -> EVT_TIME_OVF_TIMER1_EN_R {
        EVT_TIME_OVF_TIMER1_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Ledc timer2 overflow event enable register, write 1 to enable this event.
    #[inline(always)]
    pub fn evt_time_ovf_timer2_en(&self) -> EVT_TIME_OVF_TIMER2_EN_R {
        EVT_TIME_OVF_TIMER2_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Ledc timer3 overflow event enable register, write 1 to enable this event.
    #[inline(always)]
    pub fn evt_time_ovf_timer3_en(&self) -> EVT_TIME_OVF_TIMER3_EN_R {
        EVT_TIME_OVF_TIMER3_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Ledc timer0 compare event enable register, write 1 to enable this event.
    #[inline(always)]
    pub fn evt_time0_cmp_en(&self) -> EVT_TIME0_CMP_EN_R {
        EVT_TIME0_CMP_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Ledc timer1 compare event enable register, write 1 to enable this event.
    #[inline(always)]
    pub fn evt_time1_cmp_en(&self) -> EVT_TIME1_CMP_EN_R {
        EVT_TIME1_CMP_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Ledc timer2 compare event enable register, write 1 to enable this event.
    #[inline(always)]
    pub fn evt_time2_cmp_en(&self) -> EVT_TIME2_CMP_EN_R {
        EVT_TIME2_CMP_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Ledc timer3 compare event enable register, write 1 to enable this event.
    #[inline(always)]
    pub fn evt_time3_cmp_en(&self) -> EVT_TIME3_CMP_EN_R {
        EVT_TIME3_CMP_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Ledc ch0 duty scale update task enable register, write 1 to enable this task.
    #[inline(always)]
    pub fn task_duty_scale_update_ch0_en(&self) -> TASK_DUTY_SCALE_UPDATE_CH0_EN_R {
        TASK_DUTY_SCALE_UPDATE_CH0_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Ledc ch1 duty scale update task enable register, write 1 to enable this task.
    #[inline(always)]
    pub fn task_duty_scale_update_ch1_en(&self) -> TASK_DUTY_SCALE_UPDATE_CH1_EN_R {
        TASK_DUTY_SCALE_UPDATE_CH1_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Ledc ch2 duty scale update task enable register, write 1 to enable this task.
    #[inline(always)]
    pub fn task_duty_scale_update_ch2_en(&self) -> TASK_DUTY_SCALE_UPDATE_CH2_EN_R {
        TASK_DUTY_SCALE_UPDATE_CH2_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Ledc ch3 duty scale update task enable register, write 1 to enable this task.
    #[inline(always)]
    pub fn task_duty_scale_update_ch3_en(&self) -> TASK_DUTY_SCALE_UPDATE_CH3_EN_R {
        TASK_DUTY_SCALE_UPDATE_CH3_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Ledc ch4 duty scale update task enable register, write 1 to enable this task.
    #[inline(always)]
    pub fn task_duty_scale_update_ch4_en(&self) -> TASK_DUTY_SCALE_UPDATE_CH4_EN_R {
        TASK_DUTY_SCALE_UPDATE_CH4_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Ledc ch5 duty scale update task enable register, write 1 to enable this task.
    #[inline(always)]
    pub fn task_duty_scale_update_ch5_en(&self) -> TASK_DUTY_SCALE_UPDATE_CH5_EN_R {
        TASK_DUTY_SCALE_UPDATE_CH5_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVT_TASK_EN0")
            .field("evt_duty_chng_end_ch0_en", &self.evt_duty_chng_end_ch0_en())
            .field("evt_duty_chng_end_ch1_en", &self.evt_duty_chng_end_ch1_en())
            .field("evt_duty_chng_end_ch2_en", &self.evt_duty_chng_end_ch2_en())
            .field("evt_duty_chng_end_ch3_en", &self.evt_duty_chng_end_ch3_en())
            .field("evt_duty_chng_end_ch4_en", &self.evt_duty_chng_end_ch4_en())
            .field("evt_duty_chng_end_ch5_en", &self.evt_duty_chng_end_ch5_en())
            .field("evt_ovf_cnt_pls_ch0_en", &self.evt_ovf_cnt_pls_ch0_en())
            .field("evt_ovf_cnt_pls_ch1_en", &self.evt_ovf_cnt_pls_ch1_en())
            .field("evt_ovf_cnt_pls_ch2_en", &self.evt_ovf_cnt_pls_ch2_en())
            .field("evt_ovf_cnt_pls_ch3_en", &self.evt_ovf_cnt_pls_ch3_en())
            .field("evt_ovf_cnt_pls_ch4_en", &self.evt_ovf_cnt_pls_ch4_en())
            .field("evt_ovf_cnt_pls_ch5_en", &self.evt_ovf_cnt_pls_ch5_en())
            .field("evt_time_ovf_timer0_en", &self.evt_time_ovf_timer0_en())
            .field("evt_time_ovf_timer1_en", &self.evt_time_ovf_timer1_en())
            .field("evt_time_ovf_timer2_en", &self.evt_time_ovf_timer2_en())
            .field("evt_time_ovf_timer3_en", &self.evt_time_ovf_timer3_en())
            .field("evt_time0_cmp_en", &self.evt_time0_cmp_en())
            .field("evt_time1_cmp_en", &self.evt_time1_cmp_en())
            .field("evt_time2_cmp_en", &self.evt_time2_cmp_en())
            .field("evt_time3_cmp_en", &self.evt_time3_cmp_en())
            .field(
                "task_duty_scale_update_ch0_en",
                &self.task_duty_scale_update_ch0_en(),
            )
            .field(
                "task_duty_scale_update_ch1_en",
                &self.task_duty_scale_update_ch1_en(),
            )
            .field(
                "task_duty_scale_update_ch2_en",
                &self.task_duty_scale_update_ch2_en(),
            )
            .field(
                "task_duty_scale_update_ch3_en",
                &self.task_duty_scale_update_ch3_en(),
            )
            .field(
                "task_duty_scale_update_ch4_en",
                &self.task_duty_scale_update_ch4_en(),
            )
            .field(
                "task_duty_scale_update_ch5_en",
                &self.task_duty_scale_update_ch5_en(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - Ledc ch0 duty change end event enable register, write 1 to enable this event.
    #[inline(always)]
    #[must_use]
    pub fn evt_duty_chng_end_ch0_en(&mut self) -> EVT_DUTY_CHNG_END_CH0_EN_W<EVT_TASK_EN0_SPEC> {
        EVT_DUTY_CHNG_END_CH0_EN_W::new(self, 0)
    }
    ///Bit 1 - Ledc ch1 duty change end event enable register, write 1 to enable this event.
    #[inline(always)]
    #[must_use]
    pub fn evt_duty_chng_end_ch1_en(&mut self) -> EVT_DUTY_CHNG_END_CH1_EN_W<EVT_TASK_EN0_SPEC> {
        EVT_DUTY_CHNG_END_CH1_EN_W::new(self, 1)
    }
    ///Bit 2 - Ledc ch2 duty change end event enable register, write 1 to enable this event.
    #[inline(always)]
    #[must_use]
    pub fn evt_duty_chng_end_ch2_en(&mut self) -> EVT_DUTY_CHNG_END_CH2_EN_W<EVT_TASK_EN0_SPEC> {
        EVT_DUTY_CHNG_END_CH2_EN_W::new(self, 2)
    }
    ///Bit 3 - Ledc ch3 duty change end event enable register, write 1 to enable this event.
    #[inline(always)]
    #[must_use]
    pub fn evt_duty_chng_end_ch3_en(&mut self) -> EVT_DUTY_CHNG_END_CH3_EN_W<EVT_TASK_EN0_SPEC> {
        EVT_DUTY_CHNG_END_CH3_EN_W::new(self, 3)
    }
    ///Bit 4 - Ledc ch4 duty change end event enable register, write 1 to enable this event.
    #[inline(always)]
    #[must_use]
    pub fn evt_duty_chng_end_ch4_en(&mut self) -> EVT_DUTY_CHNG_END_CH4_EN_W<EVT_TASK_EN0_SPEC> {
        EVT_DUTY_CHNG_END_CH4_EN_W::new(self, 4)
    }
    ///Bit 5 - Ledc ch5 duty change end event enable register, write 1 to enable this event.
    #[inline(always)]
    #[must_use]
    pub fn evt_duty_chng_end_ch5_en(&mut self) -> EVT_DUTY_CHNG_END_CH5_EN_W<EVT_TASK_EN0_SPEC> {
        EVT_DUTY_CHNG_END_CH5_EN_W::new(self, 5)
    }
    ///Bit 8 - Ledc ch0 overflow count pulse event enable register, write 1 to enable this event.
    #[inline(always)]
    #[must_use]
    pub fn evt_ovf_cnt_pls_ch0_en(&mut self) -> EVT_OVF_CNT_PLS_CH0_EN_W<EVT_TASK_EN0_SPEC> {
        EVT_OVF_CNT_PLS_CH0_EN_W::new(self, 8)
    }
    ///Bit 9 - Ledc ch1 overflow count pulse event enable register, write 1 to enable this event.
    #[inline(always)]
    #[must_use]
    pub fn evt_ovf_cnt_pls_ch1_en(&mut self) -> EVT_OVF_CNT_PLS_CH1_EN_W<EVT_TASK_EN0_SPEC> {
        EVT_OVF_CNT_PLS_CH1_EN_W::new(self, 9)
    }
    ///Bit 10 - Ledc ch2 overflow count pulse event enable register, write 1 to enable this event.
    #[inline(always)]
    #[must_use]
    pub fn evt_ovf_cnt_pls_ch2_en(&mut self) -> EVT_OVF_CNT_PLS_CH2_EN_W<EVT_TASK_EN0_SPEC> {
        EVT_OVF_CNT_PLS_CH2_EN_W::new(self, 10)
    }
    ///Bit 11 - Ledc ch3 overflow count pulse event enable register, write 1 to enable this event.
    #[inline(always)]
    #[must_use]
    pub fn evt_ovf_cnt_pls_ch3_en(&mut self) -> EVT_OVF_CNT_PLS_CH3_EN_W<EVT_TASK_EN0_SPEC> {
        EVT_OVF_CNT_PLS_CH3_EN_W::new(self, 11)
    }
    ///Bit 12 - Ledc ch4 overflow count pulse event enable register, write 1 to enable this event.
    #[inline(always)]
    #[must_use]
    pub fn evt_ovf_cnt_pls_ch4_en(&mut self) -> EVT_OVF_CNT_PLS_CH4_EN_W<EVT_TASK_EN0_SPEC> {
        EVT_OVF_CNT_PLS_CH4_EN_W::new(self, 12)
    }
    ///Bit 13 - Ledc ch5 overflow count pulse event enable register, write 1 to enable this event.
    #[inline(always)]
    #[must_use]
    pub fn evt_ovf_cnt_pls_ch5_en(&mut self) -> EVT_OVF_CNT_PLS_CH5_EN_W<EVT_TASK_EN0_SPEC> {
        EVT_OVF_CNT_PLS_CH5_EN_W::new(self, 13)
    }
    ///Bit 16 - Ledc timer0 overflow event enable register, write 1 to enable this event.
    #[inline(always)]
    #[must_use]
    pub fn evt_time_ovf_timer0_en(&mut self) -> EVT_TIME_OVF_TIMER0_EN_W<EVT_TASK_EN0_SPEC> {
        EVT_TIME_OVF_TIMER0_EN_W::new(self, 16)
    }
    ///Bit 17 - Ledc timer1 overflow event enable register, write 1 to enable this event.
    #[inline(always)]
    #[must_use]
    pub fn evt_time_ovf_timer1_en(&mut self) -> EVT_TIME_OVF_TIMER1_EN_W<EVT_TASK_EN0_SPEC> {
        EVT_TIME_OVF_TIMER1_EN_W::new(self, 17)
    }
    ///Bit 18 - Ledc timer2 overflow event enable register, write 1 to enable this event.
    #[inline(always)]
    #[must_use]
    pub fn evt_time_ovf_timer2_en(&mut self) -> EVT_TIME_OVF_TIMER2_EN_W<EVT_TASK_EN0_SPEC> {
        EVT_TIME_OVF_TIMER2_EN_W::new(self, 18)
    }
    ///Bit 19 - Ledc timer3 overflow event enable register, write 1 to enable this event.
    #[inline(always)]
    #[must_use]
    pub fn evt_time_ovf_timer3_en(&mut self) -> EVT_TIME_OVF_TIMER3_EN_W<EVT_TASK_EN0_SPEC> {
        EVT_TIME_OVF_TIMER3_EN_W::new(self, 19)
    }
    ///Bit 20 - Ledc timer0 compare event enable register, write 1 to enable this event.
    #[inline(always)]
    #[must_use]
    pub fn evt_time0_cmp_en(&mut self) -> EVT_TIME0_CMP_EN_W<EVT_TASK_EN0_SPEC> {
        EVT_TIME0_CMP_EN_W::new(self, 20)
    }
    ///Bit 21 - Ledc timer1 compare event enable register, write 1 to enable this event.
    #[inline(always)]
    #[must_use]
    pub fn evt_time1_cmp_en(&mut self) -> EVT_TIME1_CMP_EN_W<EVT_TASK_EN0_SPEC> {
        EVT_TIME1_CMP_EN_W::new(self, 21)
    }
    ///Bit 22 - Ledc timer2 compare event enable register, write 1 to enable this event.
    #[inline(always)]
    #[must_use]
    pub fn evt_time2_cmp_en(&mut self) -> EVT_TIME2_CMP_EN_W<EVT_TASK_EN0_SPEC> {
        EVT_TIME2_CMP_EN_W::new(self, 22)
    }
    ///Bit 23 - Ledc timer3 compare event enable register, write 1 to enable this event.
    #[inline(always)]
    #[must_use]
    pub fn evt_time3_cmp_en(&mut self) -> EVT_TIME3_CMP_EN_W<EVT_TASK_EN0_SPEC> {
        EVT_TIME3_CMP_EN_W::new(self, 23)
    }
    ///Bit 24 - Ledc ch0 duty scale update task enable register, write 1 to enable this task.
    #[inline(always)]
    #[must_use]
    pub fn task_duty_scale_update_ch0_en(
        &mut self,
    ) -> TASK_DUTY_SCALE_UPDATE_CH0_EN_W<EVT_TASK_EN0_SPEC> {
        TASK_DUTY_SCALE_UPDATE_CH0_EN_W::new(self, 24)
    }
    ///Bit 25 - Ledc ch1 duty scale update task enable register, write 1 to enable this task.
    #[inline(always)]
    #[must_use]
    pub fn task_duty_scale_update_ch1_en(
        &mut self,
    ) -> TASK_DUTY_SCALE_UPDATE_CH1_EN_W<EVT_TASK_EN0_SPEC> {
        TASK_DUTY_SCALE_UPDATE_CH1_EN_W::new(self, 25)
    }
    ///Bit 26 - Ledc ch2 duty scale update task enable register, write 1 to enable this task.
    #[inline(always)]
    #[must_use]
    pub fn task_duty_scale_update_ch2_en(
        &mut self,
    ) -> TASK_DUTY_SCALE_UPDATE_CH2_EN_W<EVT_TASK_EN0_SPEC> {
        TASK_DUTY_SCALE_UPDATE_CH2_EN_W::new(self, 26)
    }
    ///Bit 27 - Ledc ch3 duty scale update task enable register, write 1 to enable this task.
    #[inline(always)]
    #[must_use]
    pub fn task_duty_scale_update_ch3_en(
        &mut self,
    ) -> TASK_DUTY_SCALE_UPDATE_CH3_EN_W<EVT_TASK_EN0_SPEC> {
        TASK_DUTY_SCALE_UPDATE_CH3_EN_W::new(self, 27)
    }
    ///Bit 28 - Ledc ch4 duty scale update task enable register, write 1 to enable this task.
    #[inline(always)]
    #[must_use]
    pub fn task_duty_scale_update_ch4_en(
        &mut self,
    ) -> TASK_DUTY_SCALE_UPDATE_CH4_EN_W<EVT_TASK_EN0_SPEC> {
        TASK_DUTY_SCALE_UPDATE_CH4_EN_W::new(self, 28)
    }
    ///Bit 29 - Ledc ch5 duty scale update task enable register, write 1 to enable this task.
    #[inline(always)]
    #[must_use]
    pub fn task_duty_scale_update_ch5_en(
        &mut self,
    ) -> TASK_DUTY_SCALE_UPDATE_CH5_EN_W<EVT_TASK_EN0_SPEC> {
        TASK_DUTY_SCALE_UPDATE_CH5_EN_W::new(self, 29)
    }
}
/**Ledc event task enable bit register0.

You can [`read`](crate::generic::Reg::read) this register and get [`evt_task_en0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_task_en0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EVT_TASK_EN0_SPEC;
impl crate::RegisterSpec for EVT_TASK_EN0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`evt_task_en0::R`](R) reader structure
impl crate::Readable for EVT_TASK_EN0_SPEC {}
///`write(|w| ..)` method takes [`evt_task_en0::W`](W) writer structure
impl crate::Writable for EVT_TASK_EN0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EVT_TASK_EN0 to value 0
impl crate::Resettable for EVT_TASK_EN0_SPEC {
    const RESET_VALUE: u32 = 0;
}
