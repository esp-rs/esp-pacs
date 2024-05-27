///Register `EVT_TASK_EN1` reader
pub type R = crate::R<EVT_TASK_EN1_SPEC>;
///Register `EVT_TASK_EN1` writer
pub type W = crate::W<EVT_TASK_EN1_SPEC>;
///Field `TASK_TIMER0_RES_UPDATE_EN` reader - Configures whether or not to enable ledc_timer0_res_update task.\\0: Disable\\1: Enable
pub type TASK_TIMER0_RES_UPDATE_EN_R = crate::BitReader;
///Field `TASK_TIMER0_RES_UPDATE_EN` writer - Configures whether or not to enable ledc_timer0_res_update task.\\0: Disable\\1: Enable
pub type TASK_TIMER0_RES_UPDATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_TIMER1_RES_UPDATE_EN` reader - Configures whether or not to enable ledc_timer1_res_update task.\\0: Disable\\1: Enable
pub type TASK_TIMER1_RES_UPDATE_EN_R = crate::BitReader;
///Field `TASK_TIMER1_RES_UPDATE_EN` writer - Configures whether or not to enable ledc_timer1_res_update task.\\0: Disable\\1: Enable
pub type TASK_TIMER1_RES_UPDATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_TIMER2_RES_UPDATE_EN` reader - Configures whether or not to enable ledc_timer2_res_update task.\\0: Disable\\1: Enable
pub type TASK_TIMER2_RES_UPDATE_EN_R = crate::BitReader;
///Field `TASK_TIMER2_RES_UPDATE_EN` writer - Configures whether or not to enable ledc_timer2_res_update task.\\0: Disable\\1: Enable
pub type TASK_TIMER2_RES_UPDATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_TIMER3_RES_UPDATE_EN` reader - Configures whether or not to enable ledc_timer3_res_update task.\\0: Disable\\1: Enable
pub type TASK_TIMER3_RES_UPDATE_EN_R = crate::BitReader;
///Field `TASK_TIMER3_RES_UPDATE_EN` writer - Configures whether or not to enable ledc_timer3_res_update task.\\0: Disable\\1: Enable
pub type TASK_TIMER3_RES_UPDATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_TIMER0_CAP_EN` reader - Configures whether or not to enable ledc_timer0_cap task.\\0: Disable\\1: Enable
pub type TASK_TIMER0_CAP_EN_R = crate::BitReader;
///Field `TASK_TIMER0_CAP_EN` writer - Configures whether or not to enable ledc_timer0_cap task.\\0: Disable\\1: Enable
pub type TASK_TIMER0_CAP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_TIMER1_CAP_EN` reader - Configures whether or not to enable ledc_timer1_cap task.\\0: Disable\\1: Enable
pub type TASK_TIMER1_CAP_EN_R = crate::BitReader;
///Field `TASK_TIMER1_CAP_EN` writer - Configures whether or not to enable ledc_timer1_cap task.\\0: Disable\\1: Enable
pub type TASK_TIMER1_CAP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_TIMER2_CAP_EN` reader - Configures whether or not to enable ledc_timer2_cap task.\\0: Disable\\1: Enable
pub type TASK_TIMER2_CAP_EN_R = crate::BitReader;
///Field `TASK_TIMER2_CAP_EN` writer - Configures whether or not to enable ledc_timer2_cap task.\\0: Disable\\1: Enable
pub type TASK_TIMER2_CAP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_TIMER3_CAP_EN` reader - Configures whether or not to enable ledc_timer3_cap task.\\0: Disable\\1: Enable
pub type TASK_TIMER3_CAP_EN_R = crate::BitReader;
///Field `TASK_TIMER3_CAP_EN` writer - Configures whether or not to enable ledc_timer3_cap task.\\0: Disable\\1: Enable
pub type TASK_TIMER3_CAP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_SIG_OUT_DIS_CH0_EN` reader - Configures whether or not to enable ledc_ch0_sig_out_dis task.\\0: Disable\\1: Enable
pub type TASK_SIG_OUT_DIS_CH0_EN_R = crate::BitReader;
///Field `TASK_SIG_OUT_DIS_CH0_EN` writer - Configures whether or not to enable ledc_ch0_sig_out_dis task.\\0: Disable\\1: Enable
pub type TASK_SIG_OUT_DIS_CH0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_SIG_OUT_DIS_CH1_EN` reader - Configures whether or not to enable ledc_ch1_sig_out_dis task.\\0: Disable\\1: Enable
pub type TASK_SIG_OUT_DIS_CH1_EN_R = crate::BitReader;
///Field `TASK_SIG_OUT_DIS_CH1_EN` writer - Configures whether or not to enable ledc_ch1_sig_out_dis task.\\0: Disable\\1: Enable
pub type TASK_SIG_OUT_DIS_CH1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_SIG_OUT_DIS_CH2_EN` reader - Configures whether or not to enable ledc_ch2_sig_out_dis task.\\0: Disable\\1: Enable
pub type TASK_SIG_OUT_DIS_CH2_EN_R = crate::BitReader;
///Field `TASK_SIG_OUT_DIS_CH2_EN` writer - Configures whether or not to enable ledc_ch2_sig_out_dis task.\\0: Disable\\1: Enable
pub type TASK_SIG_OUT_DIS_CH2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_SIG_OUT_DIS_CH3_EN` reader - Configures whether or not to enable ledc_ch3_sig_out_dis task.\\0: Disable\\1: Enable
pub type TASK_SIG_OUT_DIS_CH3_EN_R = crate::BitReader;
///Field `TASK_SIG_OUT_DIS_CH3_EN` writer - Configures whether or not to enable ledc_ch3_sig_out_dis task.\\0: Disable\\1: Enable
pub type TASK_SIG_OUT_DIS_CH3_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_SIG_OUT_DIS_CH4_EN` reader - Configures whether or not to enable ledc_ch4_sig_out_dis task.\\0: Disable\\1: Enable
pub type TASK_SIG_OUT_DIS_CH4_EN_R = crate::BitReader;
///Field `TASK_SIG_OUT_DIS_CH4_EN` writer - Configures whether or not to enable ledc_ch4_sig_out_dis task.\\0: Disable\\1: Enable
pub type TASK_SIG_OUT_DIS_CH4_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_SIG_OUT_DIS_CH5_EN` reader - Configures whether or not to enable ledc_ch5_sig_out_dis task.\\0: Disable\\1: Enable
pub type TASK_SIG_OUT_DIS_CH5_EN_R = crate::BitReader;
///Field `TASK_SIG_OUT_DIS_CH5_EN` writer - Configures whether or not to enable ledc_ch5_sig_out_dis task.\\0: Disable\\1: Enable
pub type TASK_SIG_OUT_DIS_CH5_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_SIG_OUT_DIS_CH6_EN` reader - Configures whether or not to enable ledc_ch6_sig_out_dis task.\\0: Disable\\1: Enable
pub type TASK_SIG_OUT_DIS_CH6_EN_R = crate::BitReader;
///Field `TASK_SIG_OUT_DIS_CH6_EN` writer - Configures whether or not to enable ledc_ch6_sig_out_dis task.\\0: Disable\\1: Enable
pub type TASK_SIG_OUT_DIS_CH6_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_SIG_OUT_DIS_CH7_EN` reader - Configures whether or not to enable ledc_ch7_sig_out_dis task.\\0: Disable\\1: Enable
pub type TASK_SIG_OUT_DIS_CH7_EN_R = crate::BitReader;
///Field `TASK_SIG_OUT_DIS_CH7_EN` writer - Configures whether or not to enable ledc_ch7_sig_out_dis task.\\0: Disable\\1: Enable
pub type TASK_SIG_OUT_DIS_CH7_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_OVF_CNT_RST_CH0_EN` reader - Configures whether or not to enable ledc_ch0_ovf_cnt_rst task.\\0: Disable\\1: Enable
pub type TASK_OVF_CNT_RST_CH0_EN_R = crate::BitReader;
///Field `TASK_OVF_CNT_RST_CH0_EN` writer - Configures whether or not to enable ledc_ch0_ovf_cnt_rst task.\\0: Disable\\1: Enable
pub type TASK_OVF_CNT_RST_CH0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_OVF_CNT_RST_CH1_EN` reader - Configures whether or not to enable ledc_ch1_ovf_cnt_rst task.\\0: Disable\\1: Enable
pub type TASK_OVF_CNT_RST_CH1_EN_R = crate::BitReader;
///Field `TASK_OVF_CNT_RST_CH1_EN` writer - Configures whether or not to enable ledc_ch1_ovf_cnt_rst task.\\0: Disable\\1: Enable
pub type TASK_OVF_CNT_RST_CH1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_OVF_CNT_RST_CH2_EN` reader - Configures whether or not to enable ledc_ch2_ovf_cnt_rst task.\\0: Disable\\1: Enable
pub type TASK_OVF_CNT_RST_CH2_EN_R = crate::BitReader;
///Field `TASK_OVF_CNT_RST_CH2_EN` writer - Configures whether or not to enable ledc_ch2_ovf_cnt_rst task.\\0: Disable\\1: Enable
pub type TASK_OVF_CNT_RST_CH2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_OVF_CNT_RST_CH3_EN` reader - Configures whether or not to enable ledc_ch3_ovf_cnt_rst task.\\0: Disable\\1: Enable
pub type TASK_OVF_CNT_RST_CH3_EN_R = crate::BitReader;
///Field `TASK_OVF_CNT_RST_CH3_EN` writer - Configures whether or not to enable ledc_ch3_ovf_cnt_rst task.\\0: Disable\\1: Enable
pub type TASK_OVF_CNT_RST_CH3_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_OVF_CNT_RST_CH4_EN` reader - Configures whether or not to enable ledc_ch4_ovf_cnt_rst task.\\0: Disable\\1: Enable
pub type TASK_OVF_CNT_RST_CH4_EN_R = crate::BitReader;
///Field `TASK_OVF_CNT_RST_CH4_EN` writer - Configures whether or not to enable ledc_ch4_ovf_cnt_rst task.\\0: Disable\\1: Enable
pub type TASK_OVF_CNT_RST_CH4_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_OVF_CNT_RST_CH5_EN` reader - Configures whether or not to enable ledc_ch5_ovf_cnt_rst task.\\0: Disable\\1: Enable
pub type TASK_OVF_CNT_RST_CH5_EN_R = crate::BitReader;
///Field `TASK_OVF_CNT_RST_CH5_EN` writer - Configures whether or not to enable ledc_ch5_ovf_cnt_rst task.\\0: Disable\\1: Enable
pub type TASK_OVF_CNT_RST_CH5_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_OVF_CNT_RST_CH6_EN` reader - Configures whether or not to enable ledc_ch6_ovf_cnt_rst task.\\0: Disable\\1: Enable
pub type TASK_OVF_CNT_RST_CH6_EN_R = crate::BitReader;
///Field `TASK_OVF_CNT_RST_CH6_EN` writer - Configures whether or not to enable ledc_ch6_ovf_cnt_rst task.\\0: Disable\\1: Enable
pub type TASK_OVF_CNT_RST_CH6_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_OVF_CNT_RST_CH7_EN` reader - Configures whether or not to enable ledc_ch7_ovf_cnt_rst task.\\0: Disable\\1: Enable
pub type TASK_OVF_CNT_RST_CH7_EN_R = crate::BitReader;
///Field `TASK_OVF_CNT_RST_CH7_EN` writer - Configures whether or not to enable ledc_ch7_ovf_cnt_rst task.\\0: Disable\\1: Enable
pub type TASK_OVF_CNT_RST_CH7_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_TIMER0_RST_EN` reader - Configures whether or not to enable ledc_timer0_rst task.\\0: Disable\\1: Enable
pub type TASK_TIMER0_RST_EN_R = crate::BitReader;
///Field `TASK_TIMER0_RST_EN` writer - Configures whether or not to enable ledc_timer0_rst task.\\0: Disable\\1: Enable
pub type TASK_TIMER0_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_TIMER1_RST_EN` reader - Configures whether or not to enable ledc_timer1_rst task.\\0: Disable\\1: Enable
pub type TASK_TIMER1_RST_EN_R = crate::BitReader;
///Field `TASK_TIMER1_RST_EN` writer - Configures whether or not to enable ledc_timer1_rst task.\\0: Disable\\1: Enable
pub type TASK_TIMER1_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_TIMER2_RST_EN` reader - Configures whether or not to enable ledc_timer2_rst task.\\0: Disable\\1: Enable
pub type TASK_TIMER2_RST_EN_R = crate::BitReader;
///Field `TASK_TIMER2_RST_EN` writer - Configures whether or not to enable ledc_timer2_rst task.\\0: Disable\\1: Enable
pub type TASK_TIMER2_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_TIMER3_RST_EN` reader - Configures whether or not to enable ledc_timer3_rst task.\\0: Disable\\1: Enable
pub type TASK_TIMER3_RST_EN_R = crate::BitReader;
///Field `TASK_TIMER3_RST_EN` writer - Configures whether or not to enable ledc_timer3_rst task.\\0: Disable\\1: Enable
pub type TASK_TIMER3_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_TIMER0_PAUSE_RESUME_EN` reader - Configures whether or not to enable ledc_timer0_pause_resume task.\\0: Disable\\1: Enable
pub type TASK_TIMER0_PAUSE_RESUME_EN_R = crate::BitReader;
///Field `TASK_TIMER0_PAUSE_RESUME_EN` writer - Configures whether or not to enable ledc_timer0_pause_resume task.\\0: Disable\\1: Enable
pub type TASK_TIMER0_PAUSE_RESUME_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_TIMER1_PAUSE_RESUME_EN` reader - Configures whether or not to enable ledc_timer1_pause_resume task.\\0: Disable\\1: Enable
pub type TASK_TIMER1_PAUSE_RESUME_EN_R = crate::BitReader;
///Field `TASK_TIMER1_PAUSE_RESUME_EN` writer - Configures whether or not to enable ledc_timer1_pause_resume task.\\0: Disable\\1: Enable
pub type TASK_TIMER1_PAUSE_RESUME_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_TIMER2_PAUSE_RESUME_EN` reader - Configures whether or not to enable ledc_timer2_pause_resume task.\\0: Disable\\1: Enable
pub type TASK_TIMER2_PAUSE_RESUME_EN_R = crate::BitReader;
///Field `TASK_TIMER2_PAUSE_RESUME_EN` writer - Configures whether or not to enable ledc_timer2_pause_resume task.\\0: Disable\\1: Enable
pub type TASK_TIMER2_PAUSE_RESUME_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TASK_TIMER3_PAUSE_RESUME_EN` reader - Configures whether or not to enable ledc_timer3_pause_resume task.\\0: Disable\\1: Enable
pub type TASK_TIMER3_PAUSE_RESUME_EN_R = crate::BitReader;
///Field `TASK_TIMER3_PAUSE_RESUME_EN` writer - Configures whether or not to enable ledc_timer3_pause_resume task.\\0: Disable\\1: Enable
pub type TASK_TIMER3_PAUSE_RESUME_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Configures whether or not to enable ledc_timer0_res_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer0_res_update_en(&self) -> TASK_TIMER0_RES_UPDATE_EN_R {
        TASK_TIMER0_RES_UPDATE_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Configures whether or not to enable ledc_timer1_res_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer1_res_update_en(&self) -> TASK_TIMER1_RES_UPDATE_EN_R {
        TASK_TIMER1_RES_UPDATE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Configures whether or not to enable ledc_timer2_res_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer2_res_update_en(&self) -> TASK_TIMER2_RES_UPDATE_EN_R {
        TASK_TIMER2_RES_UPDATE_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Configures whether or not to enable ledc_timer3_res_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer3_res_update_en(&self) -> TASK_TIMER3_RES_UPDATE_EN_R {
        TASK_TIMER3_RES_UPDATE_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Configures whether or not to enable ledc_timer0_cap task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer0_cap_en(&self) -> TASK_TIMER0_CAP_EN_R {
        TASK_TIMER0_CAP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Configures whether or not to enable ledc_timer1_cap task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer1_cap_en(&self) -> TASK_TIMER1_CAP_EN_R {
        TASK_TIMER1_CAP_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Configures whether or not to enable ledc_timer2_cap task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer2_cap_en(&self) -> TASK_TIMER2_CAP_EN_R {
        TASK_TIMER2_CAP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Configures whether or not to enable ledc_timer3_cap task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer3_cap_en(&self) -> TASK_TIMER3_CAP_EN_R {
        TASK_TIMER3_CAP_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Configures whether or not to enable ledc_ch0_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_sig_out_dis_ch0_en(&self) -> TASK_SIG_OUT_DIS_CH0_EN_R {
        TASK_SIG_OUT_DIS_CH0_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Configures whether or not to enable ledc_ch1_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_sig_out_dis_ch1_en(&self) -> TASK_SIG_OUT_DIS_CH1_EN_R {
        TASK_SIG_OUT_DIS_CH1_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Configures whether or not to enable ledc_ch2_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_sig_out_dis_ch2_en(&self) -> TASK_SIG_OUT_DIS_CH2_EN_R {
        TASK_SIG_OUT_DIS_CH2_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Configures whether or not to enable ledc_ch3_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_sig_out_dis_ch3_en(&self) -> TASK_SIG_OUT_DIS_CH3_EN_R {
        TASK_SIG_OUT_DIS_CH3_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Configures whether or not to enable ledc_ch4_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_sig_out_dis_ch4_en(&self) -> TASK_SIG_OUT_DIS_CH4_EN_R {
        TASK_SIG_OUT_DIS_CH4_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Configures whether or not to enable ledc_ch5_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_sig_out_dis_ch5_en(&self) -> TASK_SIG_OUT_DIS_CH5_EN_R {
        TASK_SIG_OUT_DIS_CH5_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Configures whether or not to enable ledc_ch6_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_sig_out_dis_ch6_en(&self) -> TASK_SIG_OUT_DIS_CH6_EN_R {
        TASK_SIG_OUT_DIS_CH6_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Configures whether or not to enable ledc_ch7_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_sig_out_dis_ch7_en(&self) -> TASK_SIG_OUT_DIS_CH7_EN_R {
        TASK_SIG_OUT_DIS_CH7_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Configures whether or not to enable ledc_ch0_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch0_en(&self) -> TASK_OVF_CNT_RST_CH0_EN_R {
        TASK_OVF_CNT_RST_CH0_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Configures whether or not to enable ledc_ch1_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch1_en(&self) -> TASK_OVF_CNT_RST_CH1_EN_R {
        TASK_OVF_CNT_RST_CH1_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Configures whether or not to enable ledc_ch2_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch2_en(&self) -> TASK_OVF_CNT_RST_CH2_EN_R {
        TASK_OVF_CNT_RST_CH2_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Configures whether or not to enable ledc_ch3_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch3_en(&self) -> TASK_OVF_CNT_RST_CH3_EN_R {
        TASK_OVF_CNT_RST_CH3_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Configures whether or not to enable ledc_ch4_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch4_en(&self) -> TASK_OVF_CNT_RST_CH4_EN_R {
        TASK_OVF_CNT_RST_CH4_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Configures whether or not to enable ledc_ch5_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch5_en(&self) -> TASK_OVF_CNT_RST_CH5_EN_R {
        TASK_OVF_CNT_RST_CH5_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Configures whether or not to enable ledc_ch6_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch6_en(&self) -> TASK_OVF_CNT_RST_CH6_EN_R {
        TASK_OVF_CNT_RST_CH6_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Configures whether or not to enable ledc_ch7_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_ovf_cnt_rst_ch7_en(&self) -> TASK_OVF_CNT_RST_CH7_EN_R {
        TASK_OVF_CNT_RST_CH7_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Configures whether or not to enable ledc_timer0_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer0_rst_en(&self) -> TASK_TIMER0_RST_EN_R {
        TASK_TIMER0_RST_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Configures whether or not to enable ledc_timer1_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer1_rst_en(&self) -> TASK_TIMER1_RST_EN_R {
        TASK_TIMER1_RST_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Configures whether or not to enable ledc_timer2_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer2_rst_en(&self) -> TASK_TIMER2_RST_EN_R {
        TASK_TIMER2_RST_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Configures whether or not to enable ledc_timer3_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer3_rst_en(&self) -> TASK_TIMER3_RST_EN_R {
        TASK_TIMER3_RST_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Configures whether or not to enable ledc_timer0_pause_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer0_pause_resume_en(&self) -> TASK_TIMER0_PAUSE_RESUME_EN_R {
        TASK_TIMER0_PAUSE_RESUME_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Configures whether or not to enable ledc_timer1_pause_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer1_pause_resume_en(&self) -> TASK_TIMER1_PAUSE_RESUME_EN_R {
        TASK_TIMER1_PAUSE_RESUME_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Configures whether or not to enable ledc_timer2_pause_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn task_timer2_pause_resume_en(&self) -> TASK_TIMER2_PAUSE_RESUME_EN_R {
        TASK_TIMER2_PAUSE_RESUME_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Configures whether or not to enable ledc_timer3_pause_resume task.\\0: Disable\\1: Enable
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
                &self.task_timer0_res_update_en(),
            )
            .field(
                "task_timer1_res_update_en",
                &self.task_timer1_res_update_en(),
            )
            .field(
                "task_timer2_res_update_en",
                &self.task_timer2_res_update_en(),
            )
            .field(
                "task_timer3_res_update_en",
                &self.task_timer3_res_update_en(),
            )
            .field("task_timer0_cap_en", &self.task_timer0_cap_en())
            .field("task_timer1_cap_en", &self.task_timer1_cap_en())
            .field("task_timer2_cap_en", &self.task_timer2_cap_en())
            .field("task_timer3_cap_en", &self.task_timer3_cap_en())
            .field("task_sig_out_dis_ch0_en", &self.task_sig_out_dis_ch0_en())
            .field("task_sig_out_dis_ch1_en", &self.task_sig_out_dis_ch1_en())
            .field("task_sig_out_dis_ch2_en", &self.task_sig_out_dis_ch2_en())
            .field("task_sig_out_dis_ch3_en", &self.task_sig_out_dis_ch3_en())
            .field("task_sig_out_dis_ch4_en", &self.task_sig_out_dis_ch4_en())
            .field("task_sig_out_dis_ch5_en", &self.task_sig_out_dis_ch5_en())
            .field("task_sig_out_dis_ch6_en", &self.task_sig_out_dis_ch6_en())
            .field("task_sig_out_dis_ch7_en", &self.task_sig_out_dis_ch7_en())
            .field("task_ovf_cnt_rst_ch0_en", &self.task_ovf_cnt_rst_ch0_en())
            .field("task_ovf_cnt_rst_ch1_en", &self.task_ovf_cnt_rst_ch1_en())
            .field("task_ovf_cnt_rst_ch2_en", &self.task_ovf_cnt_rst_ch2_en())
            .field("task_ovf_cnt_rst_ch3_en", &self.task_ovf_cnt_rst_ch3_en())
            .field("task_ovf_cnt_rst_ch4_en", &self.task_ovf_cnt_rst_ch4_en())
            .field("task_ovf_cnt_rst_ch5_en", &self.task_ovf_cnt_rst_ch5_en())
            .field("task_ovf_cnt_rst_ch6_en", &self.task_ovf_cnt_rst_ch6_en())
            .field("task_ovf_cnt_rst_ch7_en", &self.task_ovf_cnt_rst_ch7_en())
            .field("task_timer0_rst_en", &self.task_timer0_rst_en())
            .field("task_timer1_rst_en", &self.task_timer1_rst_en())
            .field("task_timer2_rst_en", &self.task_timer2_rst_en())
            .field("task_timer3_rst_en", &self.task_timer3_rst_en())
            .field(
                "task_timer0_pause_resume_en",
                &self.task_timer0_pause_resume_en(),
            )
            .field(
                "task_timer1_pause_resume_en",
                &self.task_timer1_pause_resume_en(),
            )
            .field(
                "task_timer2_pause_resume_en",
                &self.task_timer2_pause_resume_en(),
            )
            .field(
                "task_timer3_pause_resume_en",
                &self.task_timer3_pause_resume_en(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - Configures whether or not to enable ledc_timer0_res_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_timer0_res_update_en(&mut self) -> TASK_TIMER0_RES_UPDATE_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_TIMER0_RES_UPDATE_EN_W::new(self, 0)
    }
    ///Bit 1 - Configures whether or not to enable ledc_timer1_res_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_timer1_res_update_en(&mut self) -> TASK_TIMER1_RES_UPDATE_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_TIMER1_RES_UPDATE_EN_W::new(self, 1)
    }
    ///Bit 2 - Configures whether or not to enable ledc_timer2_res_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_timer2_res_update_en(&mut self) -> TASK_TIMER2_RES_UPDATE_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_TIMER2_RES_UPDATE_EN_W::new(self, 2)
    }
    ///Bit 3 - Configures whether or not to enable ledc_timer3_res_update task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_timer3_res_update_en(&mut self) -> TASK_TIMER3_RES_UPDATE_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_TIMER3_RES_UPDATE_EN_W::new(self, 3)
    }
    ///Bit 4 - Configures whether or not to enable ledc_timer0_cap task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_timer0_cap_en(&mut self) -> TASK_TIMER0_CAP_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_TIMER0_CAP_EN_W::new(self, 4)
    }
    ///Bit 5 - Configures whether or not to enable ledc_timer1_cap task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_timer1_cap_en(&mut self) -> TASK_TIMER1_CAP_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_TIMER1_CAP_EN_W::new(self, 5)
    }
    ///Bit 6 - Configures whether or not to enable ledc_timer2_cap task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_timer2_cap_en(&mut self) -> TASK_TIMER2_CAP_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_TIMER2_CAP_EN_W::new(self, 6)
    }
    ///Bit 7 - Configures whether or not to enable ledc_timer3_cap task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_timer3_cap_en(&mut self) -> TASK_TIMER3_CAP_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_TIMER3_CAP_EN_W::new(self, 7)
    }
    ///Bit 8 - Configures whether or not to enable ledc_ch0_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_sig_out_dis_ch0_en(&mut self) -> TASK_SIG_OUT_DIS_CH0_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_SIG_OUT_DIS_CH0_EN_W::new(self, 8)
    }
    ///Bit 9 - Configures whether or not to enable ledc_ch1_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_sig_out_dis_ch1_en(&mut self) -> TASK_SIG_OUT_DIS_CH1_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_SIG_OUT_DIS_CH1_EN_W::new(self, 9)
    }
    ///Bit 10 - Configures whether or not to enable ledc_ch2_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_sig_out_dis_ch2_en(&mut self) -> TASK_SIG_OUT_DIS_CH2_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_SIG_OUT_DIS_CH2_EN_W::new(self, 10)
    }
    ///Bit 11 - Configures whether or not to enable ledc_ch3_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_sig_out_dis_ch3_en(&mut self) -> TASK_SIG_OUT_DIS_CH3_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_SIG_OUT_DIS_CH3_EN_W::new(self, 11)
    }
    ///Bit 12 - Configures whether or not to enable ledc_ch4_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_sig_out_dis_ch4_en(&mut self) -> TASK_SIG_OUT_DIS_CH4_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_SIG_OUT_DIS_CH4_EN_W::new(self, 12)
    }
    ///Bit 13 - Configures whether or not to enable ledc_ch5_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_sig_out_dis_ch5_en(&mut self) -> TASK_SIG_OUT_DIS_CH5_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_SIG_OUT_DIS_CH5_EN_W::new(self, 13)
    }
    ///Bit 14 - Configures whether or not to enable ledc_ch6_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_sig_out_dis_ch6_en(&mut self) -> TASK_SIG_OUT_DIS_CH6_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_SIG_OUT_DIS_CH6_EN_W::new(self, 14)
    }
    ///Bit 15 - Configures whether or not to enable ledc_ch7_sig_out_dis task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_sig_out_dis_ch7_en(&mut self) -> TASK_SIG_OUT_DIS_CH7_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_SIG_OUT_DIS_CH7_EN_W::new(self, 15)
    }
    ///Bit 16 - Configures whether or not to enable ledc_ch0_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_ovf_cnt_rst_ch0_en(&mut self) -> TASK_OVF_CNT_RST_CH0_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_OVF_CNT_RST_CH0_EN_W::new(self, 16)
    }
    ///Bit 17 - Configures whether or not to enable ledc_ch1_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_ovf_cnt_rst_ch1_en(&mut self) -> TASK_OVF_CNT_RST_CH1_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_OVF_CNT_RST_CH1_EN_W::new(self, 17)
    }
    ///Bit 18 - Configures whether or not to enable ledc_ch2_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_ovf_cnt_rst_ch2_en(&mut self) -> TASK_OVF_CNT_RST_CH2_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_OVF_CNT_RST_CH2_EN_W::new(self, 18)
    }
    ///Bit 19 - Configures whether or not to enable ledc_ch3_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_ovf_cnt_rst_ch3_en(&mut self) -> TASK_OVF_CNT_RST_CH3_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_OVF_CNT_RST_CH3_EN_W::new(self, 19)
    }
    ///Bit 20 - Configures whether or not to enable ledc_ch4_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_ovf_cnt_rst_ch4_en(&mut self) -> TASK_OVF_CNT_RST_CH4_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_OVF_CNT_RST_CH4_EN_W::new(self, 20)
    }
    ///Bit 21 - Configures whether or not to enable ledc_ch5_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_ovf_cnt_rst_ch5_en(&mut self) -> TASK_OVF_CNT_RST_CH5_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_OVF_CNT_RST_CH5_EN_W::new(self, 21)
    }
    ///Bit 22 - Configures whether or not to enable ledc_ch6_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_ovf_cnt_rst_ch6_en(&mut self) -> TASK_OVF_CNT_RST_CH6_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_OVF_CNT_RST_CH6_EN_W::new(self, 22)
    }
    ///Bit 23 - Configures whether or not to enable ledc_ch7_ovf_cnt_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_ovf_cnt_rst_ch7_en(&mut self) -> TASK_OVF_CNT_RST_CH7_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_OVF_CNT_RST_CH7_EN_W::new(self, 23)
    }
    ///Bit 24 - Configures whether or not to enable ledc_timer0_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_timer0_rst_en(&mut self) -> TASK_TIMER0_RST_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_TIMER0_RST_EN_W::new(self, 24)
    }
    ///Bit 25 - Configures whether or not to enable ledc_timer1_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_timer1_rst_en(&mut self) -> TASK_TIMER1_RST_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_TIMER1_RST_EN_W::new(self, 25)
    }
    ///Bit 26 - Configures whether or not to enable ledc_timer2_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_timer2_rst_en(&mut self) -> TASK_TIMER2_RST_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_TIMER2_RST_EN_W::new(self, 26)
    }
    ///Bit 27 - Configures whether or not to enable ledc_timer3_rst task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_timer3_rst_en(&mut self) -> TASK_TIMER3_RST_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_TIMER3_RST_EN_W::new(self, 27)
    }
    ///Bit 28 - Configures whether or not to enable ledc_timer0_pause_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_timer0_pause_resume_en(
        &mut self,
    ) -> TASK_TIMER0_PAUSE_RESUME_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_TIMER0_PAUSE_RESUME_EN_W::new(self, 28)
    }
    ///Bit 29 - Configures whether or not to enable ledc_timer1_pause_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_timer1_pause_resume_en(
        &mut self,
    ) -> TASK_TIMER1_PAUSE_RESUME_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_TIMER1_PAUSE_RESUME_EN_W::new(self, 29)
    }
    ///Bit 30 - Configures whether or not to enable ledc_timer2_pause_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_timer2_pause_resume_en(
        &mut self,
    ) -> TASK_TIMER2_PAUSE_RESUME_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_TIMER2_PAUSE_RESUME_EN_W::new(self, 30)
    }
    ///Bit 31 - Configures whether or not to enable ledc_timer3_pause_resume task.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn task_timer3_pause_resume_en(
        &mut self,
    ) -> TASK_TIMER3_PAUSE_RESUME_EN_W<EVT_TASK_EN1_SPEC> {
        TASK_TIMER3_PAUSE_RESUME_EN_W::new(self, 31)
    }
}
/**Ledc event task enable bit register1.

You can [`read`](crate::generic::Reg::read) this register and get [`evt_task_en1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_task_en1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EVT_TASK_EN1_SPEC;
impl crate::RegisterSpec for EVT_TASK_EN1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`evt_task_en1::R`](R) reader structure
impl crate::Readable for EVT_TASK_EN1_SPEC {}
///`write(|w| ..)` method takes [`evt_task_en1::W`](W) writer structure
impl crate::Writable for EVT_TASK_EN1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EVT_TASK_EN1 to value 0
impl crate::Resettable for EVT_TASK_EN1_SPEC {
    const RESET_VALUE: u32 = 0;
}
