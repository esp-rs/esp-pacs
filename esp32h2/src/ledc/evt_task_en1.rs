#[doc = "Register `EVT_TASK_EN1` reader"]
pub struct R(crate::R<EVT_TASK_EN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVT_TASK_EN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVT_TASK_EN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVT_TASK_EN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVT_TASK_EN1` writer"]
pub struct W(crate::W<EVT_TASK_EN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVT_TASK_EN1_SPEC>;
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
impl From<crate::W<EVT_TASK_EN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVT_TASK_EN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TASK_TIMER0_RES_UPDATE_EN` reader - Ledc timer0 res update task enable register, write 1 to enable this task."]
pub type TASK_TIMER0_RES_UPDATE_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER0_RES_UPDATE_EN` writer - Ledc timer0 res update task enable register, write 1 to enable this task."]
pub type TASK_TIMER0_RES_UPDATE_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_TIMER1_RES_UPDATE_EN` reader - Ledc timer1 res update task enable register, write 1 to enable this task."]
pub type TASK_TIMER1_RES_UPDATE_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER1_RES_UPDATE_EN` writer - Ledc timer1 res update task enable register, write 1 to enable this task."]
pub type TASK_TIMER1_RES_UPDATE_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_TIMER2_RES_UPDATE_EN` reader - Ledc timer2 res update task enable register, write 1 to enable this task."]
pub type TASK_TIMER2_RES_UPDATE_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER2_RES_UPDATE_EN` writer - Ledc timer2 res update task enable register, write 1 to enable this task."]
pub type TASK_TIMER2_RES_UPDATE_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_TIMER3_RES_UPDATE_EN` reader - Ledc timer3 res update task enable register, write 1 to enable this task."]
pub type TASK_TIMER3_RES_UPDATE_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER3_RES_UPDATE_EN` writer - Ledc timer3 res update task enable register, write 1 to enable this task."]
pub type TASK_TIMER3_RES_UPDATE_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_TIMER0_CAP_EN` reader - Ledc timer0 capture task enable register, write 1 to enable this task."]
pub type TASK_TIMER0_CAP_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER0_CAP_EN` writer - Ledc timer0 capture task enable register, write 1 to enable this task."]
pub type TASK_TIMER0_CAP_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_TIMER1_CAP_EN` reader - Ledc timer1 capture task enable register, write 1 to enable this task."]
pub type TASK_TIMER1_CAP_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER1_CAP_EN` writer - Ledc timer1 capture task enable register, write 1 to enable this task."]
pub type TASK_TIMER1_CAP_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_TIMER2_CAP_EN` reader - Ledc timer2 capture task enable register, write 1 to enable this task."]
pub type TASK_TIMER2_CAP_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER2_CAP_EN` writer - Ledc timer2 capture task enable register, write 1 to enable this task."]
pub type TASK_TIMER2_CAP_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_TIMER3_CAP_EN` reader - Ledc timer3 capture task enable register, write 1 to enable this task."]
pub type TASK_TIMER3_CAP_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER3_CAP_EN` writer - Ledc timer3 capture task enable register, write 1 to enable this task."]
pub type TASK_TIMER3_CAP_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_SIG_OUT_DIS_CH0_EN` reader - Ledc ch0 signal out disable task enable register, write 1 to enable this task."]
pub type TASK_SIG_OUT_DIS_CH0_EN_R = crate::BitReader;
#[doc = "Field `TASK_SIG_OUT_DIS_CH0_EN` writer - Ledc ch0 signal out disable task enable register, write 1 to enable this task."]
pub type TASK_SIG_OUT_DIS_CH0_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_SIG_OUT_DIS_CH1_EN` reader - Ledc ch1 signal out disable task enable register, write 1 to enable this task."]
pub type TASK_SIG_OUT_DIS_CH1_EN_R = crate::BitReader;
#[doc = "Field `TASK_SIG_OUT_DIS_CH1_EN` writer - Ledc ch1 signal out disable task enable register, write 1 to enable this task."]
pub type TASK_SIG_OUT_DIS_CH1_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_SIG_OUT_DIS_CH2_EN` reader - Ledc ch2 signal out disable task enable register, write 1 to enable this task."]
pub type TASK_SIG_OUT_DIS_CH2_EN_R = crate::BitReader;
#[doc = "Field `TASK_SIG_OUT_DIS_CH2_EN` writer - Ledc ch2 signal out disable task enable register, write 1 to enable this task."]
pub type TASK_SIG_OUT_DIS_CH2_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_SIG_OUT_DIS_CH3_EN` reader - Ledc ch3 signal out disable task enable register, write 1 to enable this task."]
pub type TASK_SIG_OUT_DIS_CH3_EN_R = crate::BitReader;
#[doc = "Field `TASK_SIG_OUT_DIS_CH3_EN` writer - Ledc ch3 signal out disable task enable register, write 1 to enable this task."]
pub type TASK_SIG_OUT_DIS_CH3_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_SIG_OUT_DIS_CH4_EN` reader - Ledc ch4 signal out disable task enable register, write 1 to enable this task."]
pub type TASK_SIG_OUT_DIS_CH4_EN_R = crate::BitReader;
#[doc = "Field `TASK_SIG_OUT_DIS_CH4_EN` writer - Ledc ch4 signal out disable task enable register, write 1 to enable this task."]
pub type TASK_SIG_OUT_DIS_CH4_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_SIG_OUT_DIS_CH5_EN` reader - Ledc ch5 signal out disable task enable register, write 1 to enable this task."]
pub type TASK_SIG_OUT_DIS_CH5_EN_R = crate::BitReader;
#[doc = "Field `TASK_SIG_OUT_DIS_CH5_EN` writer - Ledc ch5 signal out disable task enable register, write 1 to enable this task."]
pub type TASK_SIG_OUT_DIS_CH5_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_OVF_CNT_RST_CH0_EN` reader - Ledc ch0 overflow count reset task enable register, write 1 to enable this task."]
pub type TASK_OVF_CNT_RST_CH0_EN_R = crate::BitReader;
#[doc = "Field `TASK_OVF_CNT_RST_CH0_EN` writer - Ledc ch0 overflow count reset task enable register, write 1 to enable this task."]
pub type TASK_OVF_CNT_RST_CH0_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_OVF_CNT_RST_CH1_EN` reader - Ledc ch1 overflow count reset task enable register, write 1 to enable this task."]
pub type TASK_OVF_CNT_RST_CH1_EN_R = crate::BitReader;
#[doc = "Field `TASK_OVF_CNT_RST_CH1_EN` writer - Ledc ch1 overflow count reset task enable register, write 1 to enable this task."]
pub type TASK_OVF_CNT_RST_CH1_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_OVF_CNT_RST_CH2_EN` reader - Ledc ch2 overflow count reset task enable register, write 1 to enable this task."]
pub type TASK_OVF_CNT_RST_CH2_EN_R = crate::BitReader;
#[doc = "Field `TASK_OVF_CNT_RST_CH2_EN` writer - Ledc ch2 overflow count reset task enable register, write 1 to enable this task."]
pub type TASK_OVF_CNT_RST_CH2_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_OVF_CNT_RST_CH3_EN` reader - Ledc ch3 overflow count reset task enable register, write 1 to enable this task."]
pub type TASK_OVF_CNT_RST_CH3_EN_R = crate::BitReader;
#[doc = "Field `TASK_OVF_CNT_RST_CH3_EN` writer - Ledc ch3 overflow count reset task enable register, write 1 to enable this task."]
pub type TASK_OVF_CNT_RST_CH3_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_OVF_CNT_RST_CH4_EN` reader - Ledc ch4 overflow count reset task enable register, write 1 to enable this task."]
pub type TASK_OVF_CNT_RST_CH4_EN_R = crate::BitReader;
#[doc = "Field `TASK_OVF_CNT_RST_CH4_EN` writer - Ledc ch4 overflow count reset task enable register, write 1 to enable this task."]
pub type TASK_OVF_CNT_RST_CH4_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_OVF_CNT_RST_CH5_EN` reader - Ledc ch5 overflow count reset task enable register, write 1 to enable this task."]
pub type TASK_OVF_CNT_RST_CH5_EN_R = crate::BitReader;
#[doc = "Field `TASK_OVF_CNT_RST_CH5_EN` writer - Ledc ch5 overflow count reset task enable register, write 1 to enable this task."]
pub type TASK_OVF_CNT_RST_CH5_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_TIMER0_RST_EN` reader - Ledc timer0 reset task enable register, write 1 to enable this task."]
pub type TASK_TIMER0_RST_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER0_RST_EN` writer - Ledc timer0 reset task enable register, write 1 to enable this task."]
pub type TASK_TIMER0_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_TIMER1_RST_EN` reader - Ledc timer1 reset task enable register, write 1 to enable this task."]
pub type TASK_TIMER1_RST_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER1_RST_EN` writer - Ledc timer1 reset task enable register, write 1 to enable this task."]
pub type TASK_TIMER1_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_TIMER2_RST_EN` reader - Ledc timer2 reset task enable register, write 1 to enable this task."]
pub type TASK_TIMER2_RST_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER2_RST_EN` writer - Ledc timer2 reset task enable register, write 1 to enable this task."]
pub type TASK_TIMER2_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_TIMER3_RST_EN` reader - Ledc timer3 reset task enable register, write 1 to enable this task."]
pub type TASK_TIMER3_RST_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER3_RST_EN` writer - Ledc timer3 reset task enable register, write 1 to enable this task."]
pub type TASK_TIMER3_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_TIMER0_PAUSE_RESUME_EN` reader - Ledc timer0 pause resume task enable register, write 1 to enable this task."]
pub type TASK_TIMER0_PAUSE_RESUME_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER0_PAUSE_RESUME_EN` writer - Ledc timer0 pause resume task enable register, write 1 to enable this task."]
pub type TASK_TIMER0_PAUSE_RESUME_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_TIMER1_PAUSE_RESUME_EN` reader - Ledc timer1 pause resume task enable register, write 1 to enable this task."]
pub type TASK_TIMER1_PAUSE_RESUME_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER1_PAUSE_RESUME_EN` writer - Ledc timer1 pause resume task enable register, write 1 to enable this task."]
pub type TASK_TIMER1_PAUSE_RESUME_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_TIMER2_PAUSE_RESUME_EN` reader - Ledc timer2 pause resume task enable register, write 1 to enable this task."]
pub type TASK_TIMER2_PAUSE_RESUME_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER2_PAUSE_RESUME_EN` writer - Ledc timer2 pause resume task enable register, write 1 to enable this task."]
pub type TASK_TIMER2_PAUSE_RESUME_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
#[doc = "Field `TASK_TIMER3_PAUSE_RESUME_EN` reader - Ledc timer3 pause resume task enable register, write 1 to enable this task."]
pub type TASK_TIMER3_PAUSE_RESUME_EN_R = crate::BitReader;
#[doc = "Field `TASK_TIMER3_PAUSE_RESUME_EN` writer - Ledc timer3 pause resume task enable register, write 1 to enable this task."]
pub type TASK_TIMER3_PAUSE_RESUME_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, EVT_TASK_EN1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Ledc timer0 res update task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_timer0_res_update_en(&self) -> TASK_TIMER0_RES_UPDATE_EN_R {
        TASK_TIMER0_RES_UPDATE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ledc timer1 res update task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_timer1_res_update_en(&self) -> TASK_TIMER1_RES_UPDATE_EN_R {
        TASK_TIMER1_RES_UPDATE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Ledc timer2 res update task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_timer2_res_update_en(&self) -> TASK_TIMER2_RES_UPDATE_EN_R {
        TASK_TIMER2_RES_UPDATE_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Ledc timer3 res update task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_timer3_res_update_en(&self) -> TASK_TIMER3_RES_UPDATE_EN_R {
        TASK_TIMER3_RES_UPDATE_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Ledc timer0 capture task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_timer0_cap_en(&self) -> TASK_TIMER0_CAP_EN_R {
        TASK_TIMER0_CAP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Ledc timer1 capture task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_timer1_cap_en(&self) -> TASK_TIMER1_CAP_EN_R {
        TASK_TIMER1_CAP_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Ledc timer2 capture task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_timer2_cap_en(&self) -> TASK_TIMER2_CAP_EN_R {
        TASK_TIMER2_CAP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Ledc timer3 capture task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_timer3_cap_en(&self) -> TASK_TIMER3_CAP_EN_R {
        TASK_TIMER3_CAP_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Ledc ch0 signal out disable task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_sig_out_dis_ch0_en(&self) -> TASK_SIG_OUT_DIS_CH0_EN_R {
        TASK_SIG_OUT_DIS_CH0_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Ledc ch1 signal out disable task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_sig_out_dis_ch1_en(&self) -> TASK_SIG_OUT_DIS_CH1_EN_R {
        TASK_SIG_OUT_DIS_CH1_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Ledc ch2 signal out disable task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_sig_out_dis_ch2_en(&self) -> TASK_SIG_OUT_DIS_CH2_EN_R {
        TASK_SIG_OUT_DIS_CH2_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Ledc ch3 signal out disable task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_sig_out_dis_ch3_en(&self) -> TASK_SIG_OUT_DIS_CH3_EN_R {
        TASK_SIG_OUT_DIS_CH3_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Ledc ch4 signal out disable task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_sig_out_dis_ch4_en(&self) -> TASK_SIG_OUT_DIS_CH4_EN_R {
        TASK_SIG_OUT_DIS_CH4_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Ledc ch5 signal out disable task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_sig_out_dis_ch5_en(&self) -> TASK_SIG_OUT_DIS_CH5_EN_R {
        TASK_SIG_OUT_DIS_CH5_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Ledc ch0 overflow count reset task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch0_en(&self) -> TASK_OVF_CNT_RST_CH0_EN_R {
        TASK_OVF_CNT_RST_CH0_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Ledc ch1 overflow count reset task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch1_en(&self) -> TASK_OVF_CNT_RST_CH1_EN_R {
        TASK_OVF_CNT_RST_CH1_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Ledc ch2 overflow count reset task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch2_en(&self) -> TASK_OVF_CNT_RST_CH2_EN_R {
        TASK_OVF_CNT_RST_CH2_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Ledc ch3 overflow count reset task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch3_en(&self) -> TASK_OVF_CNT_RST_CH3_EN_R {
        TASK_OVF_CNT_RST_CH3_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Ledc ch4 overflow count reset task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch4_en(&self) -> TASK_OVF_CNT_RST_CH4_EN_R {
        TASK_OVF_CNT_RST_CH4_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Ledc ch5 overflow count reset task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch5_en(&self) -> TASK_OVF_CNT_RST_CH5_EN_R {
        TASK_OVF_CNT_RST_CH5_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Ledc timer0 reset task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_timer0_rst_en(&self) -> TASK_TIMER0_RST_EN_R {
        TASK_TIMER0_RST_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Ledc timer1 reset task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_timer1_rst_en(&self) -> TASK_TIMER1_RST_EN_R {
        TASK_TIMER1_RST_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Ledc timer2 reset task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_timer2_rst_en(&self) -> TASK_TIMER2_RST_EN_R {
        TASK_TIMER2_RST_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Ledc timer3 reset task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_timer3_rst_en(&self) -> TASK_TIMER3_RST_EN_R {
        TASK_TIMER3_RST_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Ledc timer0 pause resume task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_timer0_pause_resume_en(&self) -> TASK_TIMER0_PAUSE_RESUME_EN_R {
        TASK_TIMER0_PAUSE_RESUME_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Ledc timer1 pause resume task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_timer1_pause_resume_en(&self) -> TASK_TIMER1_PAUSE_RESUME_EN_R {
        TASK_TIMER1_PAUSE_RESUME_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Ledc timer2 pause resume task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_timer2_pause_resume_en(&self) -> TASK_TIMER2_PAUSE_RESUME_EN_R {
        TASK_TIMER2_PAUSE_RESUME_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Ledc timer3 pause resume task enable register, write 1 to enable this task."]
    #[inline(always)]
    pub fn task_timer3_pause_resume_en(&self) -> TASK_TIMER3_PAUSE_RESUME_EN_R {
        TASK_TIMER3_PAUSE_RESUME_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVT_TASK_EN1")
            .field(
                "task_timer0_res_update_en",
                &format_args!("{}", self.task_timer0_res_update_en().bit()),
            )
            .field(
                "task_timer1_res_update_en",
                &format_args!("{}", self.task_timer1_res_update_en().bit()),
            )
            .field(
                "task_timer2_res_update_en",
                &format_args!("{}", self.task_timer2_res_update_en().bit()),
            )
            .field(
                "task_timer3_res_update_en",
                &format_args!("{}", self.task_timer3_res_update_en().bit()),
            )
            .field(
                "task_timer0_cap_en",
                &format_args!("{}", self.task_timer0_cap_en().bit()),
            )
            .field(
                "task_timer1_cap_en",
                &format_args!("{}", self.task_timer1_cap_en().bit()),
            )
            .field(
                "task_timer2_cap_en",
                &format_args!("{}", self.task_timer2_cap_en().bit()),
            )
            .field(
                "task_timer3_cap_en",
                &format_args!("{}", self.task_timer3_cap_en().bit()),
            )
            .field(
                "task_sig_out_dis_ch0_en",
                &format_args!("{}", self.task_sig_out_dis_ch0_en().bit()),
            )
            .field(
                "task_sig_out_dis_ch1_en",
                &format_args!("{}", self.task_sig_out_dis_ch1_en().bit()),
            )
            .field(
                "task_sig_out_dis_ch2_en",
                &format_args!("{}", self.task_sig_out_dis_ch2_en().bit()),
            )
            .field(
                "task_sig_out_dis_ch3_en",
                &format_args!("{}", self.task_sig_out_dis_ch3_en().bit()),
            )
            .field(
                "task_sig_out_dis_ch4_en",
                &format_args!("{}", self.task_sig_out_dis_ch4_en().bit()),
            )
            .field(
                "task_sig_out_dis_ch5_en",
                &format_args!("{}", self.task_sig_out_dis_ch5_en().bit()),
            )
            .field(
                "task_ovf_cnt_rst_ch0_en",
                &format_args!("{}", self.task_ovf_cnt_rst_ch0_en().bit()),
            )
            .field(
                "task_ovf_cnt_rst_ch1_en",
                &format_args!("{}", self.task_ovf_cnt_rst_ch1_en().bit()),
            )
            .field(
                "task_ovf_cnt_rst_ch2_en",
                &format_args!("{}", self.task_ovf_cnt_rst_ch2_en().bit()),
            )
            .field(
                "task_ovf_cnt_rst_ch3_en",
                &format_args!("{}", self.task_ovf_cnt_rst_ch3_en().bit()),
            )
            .field(
                "task_ovf_cnt_rst_ch4_en",
                &format_args!("{}", self.task_ovf_cnt_rst_ch4_en().bit()),
            )
            .field(
                "task_ovf_cnt_rst_ch5_en",
                &format_args!("{}", self.task_ovf_cnt_rst_ch5_en().bit()),
            )
            .field(
                "task_timer0_rst_en",
                &format_args!("{}", self.task_timer0_rst_en().bit()),
            )
            .field(
                "task_timer1_rst_en",
                &format_args!("{}", self.task_timer1_rst_en().bit()),
            )
            .field(
                "task_timer2_rst_en",
                &format_args!("{}", self.task_timer2_rst_en().bit()),
            )
            .field(
                "task_timer3_rst_en",
                &format_args!("{}", self.task_timer3_rst_en().bit()),
            )
            .field(
                "task_timer0_pause_resume_en",
                &format_args!("{}", self.task_timer0_pause_resume_en().bit()),
            )
            .field(
                "task_timer1_pause_resume_en",
                &format_args!("{}", self.task_timer1_pause_resume_en().bit()),
            )
            .field(
                "task_timer2_pause_resume_en",
                &format_args!("{}", self.task_timer2_pause_resume_en().bit()),
            )
            .field(
                "task_timer3_pause_resume_en",
                &format_args!("{}", self.task_timer3_pause_resume_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EVT_TASK_EN1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Ledc timer0 res update task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_timer0_res_update_en(&mut self) -> TASK_TIMER0_RES_UPDATE_EN_W<0> {
        TASK_TIMER0_RES_UPDATE_EN_W::new(self)
    }
    #[doc = "Bit 1 - Ledc timer1 res update task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_timer1_res_update_en(&mut self) -> TASK_TIMER1_RES_UPDATE_EN_W<1> {
        TASK_TIMER1_RES_UPDATE_EN_W::new(self)
    }
    #[doc = "Bit 2 - Ledc timer2 res update task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_timer2_res_update_en(&mut self) -> TASK_TIMER2_RES_UPDATE_EN_W<2> {
        TASK_TIMER2_RES_UPDATE_EN_W::new(self)
    }
    #[doc = "Bit 3 - Ledc timer3 res update task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_timer3_res_update_en(&mut self) -> TASK_TIMER3_RES_UPDATE_EN_W<3> {
        TASK_TIMER3_RES_UPDATE_EN_W::new(self)
    }
    #[doc = "Bit 4 - Ledc timer0 capture task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_timer0_cap_en(&mut self) -> TASK_TIMER0_CAP_EN_W<4> {
        TASK_TIMER0_CAP_EN_W::new(self)
    }
    #[doc = "Bit 5 - Ledc timer1 capture task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_timer1_cap_en(&mut self) -> TASK_TIMER1_CAP_EN_W<5> {
        TASK_TIMER1_CAP_EN_W::new(self)
    }
    #[doc = "Bit 6 - Ledc timer2 capture task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_timer2_cap_en(&mut self) -> TASK_TIMER2_CAP_EN_W<6> {
        TASK_TIMER2_CAP_EN_W::new(self)
    }
    #[doc = "Bit 7 - Ledc timer3 capture task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_timer3_cap_en(&mut self) -> TASK_TIMER3_CAP_EN_W<7> {
        TASK_TIMER3_CAP_EN_W::new(self)
    }
    #[doc = "Bit 8 - Ledc ch0 signal out disable task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_sig_out_dis_ch0_en(&mut self) -> TASK_SIG_OUT_DIS_CH0_EN_W<8> {
        TASK_SIG_OUT_DIS_CH0_EN_W::new(self)
    }
    #[doc = "Bit 9 - Ledc ch1 signal out disable task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_sig_out_dis_ch1_en(&mut self) -> TASK_SIG_OUT_DIS_CH1_EN_W<9> {
        TASK_SIG_OUT_DIS_CH1_EN_W::new(self)
    }
    #[doc = "Bit 10 - Ledc ch2 signal out disable task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_sig_out_dis_ch2_en(&mut self) -> TASK_SIG_OUT_DIS_CH2_EN_W<10> {
        TASK_SIG_OUT_DIS_CH2_EN_W::new(self)
    }
    #[doc = "Bit 11 - Ledc ch3 signal out disable task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_sig_out_dis_ch3_en(&mut self) -> TASK_SIG_OUT_DIS_CH3_EN_W<11> {
        TASK_SIG_OUT_DIS_CH3_EN_W::new(self)
    }
    #[doc = "Bit 12 - Ledc ch4 signal out disable task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_sig_out_dis_ch4_en(&mut self) -> TASK_SIG_OUT_DIS_CH4_EN_W<12> {
        TASK_SIG_OUT_DIS_CH4_EN_W::new(self)
    }
    #[doc = "Bit 13 - Ledc ch5 signal out disable task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_sig_out_dis_ch5_en(&mut self) -> TASK_SIG_OUT_DIS_CH5_EN_W<13> {
        TASK_SIG_OUT_DIS_CH5_EN_W::new(self)
    }
    #[doc = "Bit 16 - Ledc ch0 overflow count reset task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_ovf_cnt_rst_ch0_en(&mut self) -> TASK_OVF_CNT_RST_CH0_EN_W<16> {
        TASK_OVF_CNT_RST_CH0_EN_W::new(self)
    }
    #[doc = "Bit 17 - Ledc ch1 overflow count reset task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_ovf_cnt_rst_ch1_en(&mut self) -> TASK_OVF_CNT_RST_CH1_EN_W<17> {
        TASK_OVF_CNT_RST_CH1_EN_W::new(self)
    }
    #[doc = "Bit 18 - Ledc ch2 overflow count reset task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_ovf_cnt_rst_ch2_en(&mut self) -> TASK_OVF_CNT_RST_CH2_EN_W<18> {
        TASK_OVF_CNT_RST_CH2_EN_W::new(self)
    }
    #[doc = "Bit 19 - Ledc ch3 overflow count reset task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_ovf_cnt_rst_ch3_en(&mut self) -> TASK_OVF_CNT_RST_CH3_EN_W<19> {
        TASK_OVF_CNT_RST_CH3_EN_W::new(self)
    }
    #[doc = "Bit 20 - Ledc ch4 overflow count reset task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_ovf_cnt_rst_ch4_en(&mut self) -> TASK_OVF_CNT_RST_CH4_EN_W<20> {
        TASK_OVF_CNT_RST_CH4_EN_W::new(self)
    }
    #[doc = "Bit 21 - Ledc ch5 overflow count reset task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_ovf_cnt_rst_ch5_en(&mut self) -> TASK_OVF_CNT_RST_CH5_EN_W<21> {
        TASK_OVF_CNT_RST_CH5_EN_W::new(self)
    }
    #[doc = "Bit 24 - Ledc timer0 reset task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_timer0_rst_en(&mut self) -> TASK_TIMER0_RST_EN_W<24> {
        TASK_TIMER0_RST_EN_W::new(self)
    }
    #[doc = "Bit 25 - Ledc timer1 reset task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_timer1_rst_en(&mut self) -> TASK_TIMER1_RST_EN_W<25> {
        TASK_TIMER1_RST_EN_W::new(self)
    }
    #[doc = "Bit 26 - Ledc timer2 reset task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_timer2_rst_en(&mut self) -> TASK_TIMER2_RST_EN_W<26> {
        TASK_TIMER2_RST_EN_W::new(self)
    }
    #[doc = "Bit 27 - Ledc timer3 reset task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_timer3_rst_en(&mut self) -> TASK_TIMER3_RST_EN_W<27> {
        TASK_TIMER3_RST_EN_W::new(self)
    }
    #[doc = "Bit 28 - Ledc timer0 pause resume task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_timer0_pause_resume_en(&mut self) -> TASK_TIMER0_PAUSE_RESUME_EN_W<28> {
        TASK_TIMER0_PAUSE_RESUME_EN_W::new(self)
    }
    #[doc = "Bit 29 - Ledc timer1 pause resume task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_timer1_pause_resume_en(&mut self) -> TASK_TIMER1_PAUSE_RESUME_EN_W<29> {
        TASK_TIMER1_PAUSE_RESUME_EN_W::new(self)
    }
    #[doc = "Bit 30 - Ledc timer2 pause resume task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_timer2_pause_resume_en(&mut self) -> TASK_TIMER2_PAUSE_RESUME_EN_W<30> {
        TASK_TIMER2_PAUSE_RESUME_EN_W::new(self)
    }
    #[doc = "Bit 31 - Ledc timer3 pause resume task enable register, write 1 to enable this task."]
    #[inline(always)]
    #[must_use]
    pub fn task_timer3_pause_resume_en(&mut self) -> TASK_TIMER3_PAUSE_RESUME_EN_W<31> {
        TASK_TIMER3_PAUSE_RESUME_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ledc event task enable bit register1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evt_task_en1](index.html) module"]
pub struct EVT_TASK_EN1_SPEC;
impl crate::RegisterSpec for EVT_TASK_EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evt_task_en1::R](R) reader structure"]
impl crate::Readable for EVT_TASK_EN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evt_task_en1::W](W) writer structure"]
impl crate::Writable for EVT_TASK_EN1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVT_TASK_EN1 to value 0"]
impl crate::Resettable for EVT_TASK_EN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
