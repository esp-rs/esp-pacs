///Register `EVT_ST2` reader
pub type R = crate::R<EVT_ST2_SPEC>;
///Register `EVT_ST2` writer
pub type W = crate::W<EVT_ST2_SPEC>;
///Field `MCPWM0_EVT_TIMER2_TEZ_ST` reader - Represents MCPWM0_evt_timer2_tez trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_TIMER2_TEZ_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_TIMER2_TEZ_ST` writer - Represents MCPWM0_evt_timer2_tez trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_TIMER2_TEZ_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_TIMER0_TEP_ST` reader - Represents MCPWM0_evt_timer0_tep trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_TIMER0_TEP_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_TIMER0_TEP_ST` writer - Represents MCPWM0_evt_timer0_tep trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_TIMER0_TEP_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_TIMER1_TEP_ST` reader - Represents MCPWM0_evt_timer1_tep trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_TIMER1_TEP_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_TIMER1_TEP_ST` writer - Represents MCPWM0_evt_timer1_tep trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_TIMER1_TEP_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_TIMER2_TEP_ST` reader - Represents MCPWM0_evt_timer2_tep trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_TIMER2_TEP_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_TIMER2_TEP_ST` writer - Represents MCPWM0_evt_timer2_tep trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_TIMER2_TEP_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_OP0_TEA_ST` reader - Represents MCPWM0_evt_op0_tea trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP0_TEA_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_OP0_TEA_ST` writer - Represents MCPWM0_evt_op0_tea trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP0_TEA_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_OP1_TEA_ST` reader - Represents MCPWM0_evt_op1_tea trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP1_TEA_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_OP1_TEA_ST` writer - Represents MCPWM0_evt_op1_tea trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP1_TEA_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_OP2_TEA_ST` reader - Represents MCPWM0_evt_op2_tea trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP2_TEA_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_OP2_TEA_ST` writer - Represents MCPWM0_evt_op2_tea trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP2_TEA_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_OP0_TEB_ST` reader - Represents MCPWM0_evt_op0_teb trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP0_TEB_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_OP0_TEB_ST` writer - Represents MCPWM0_evt_op0_teb trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP0_TEB_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_OP1_TEB_ST` reader - Represents MCPWM0_evt_op1_teb trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP1_TEB_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_OP1_TEB_ST` writer - Represents MCPWM0_evt_op1_teb trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP1_TEB_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_OP2_TEB_ST` reader - Represents MCPWM0_evt_op2_teb trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP2_TEB_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_OP2_TEB_ST` writer - Represents MCPWM0_evt_op2_teb trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP2_TEB_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_F0_ST` reader - Represents MCPWM0_evt_f0 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_F0_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_F0_ST` writer - Represents MCPWM0_evt_f0 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_F0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_F1_ST` reader - Represents MCPWM0_evt_f1 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_F1_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_F1_ST` writer - Represents MCPWM0_evt_f1 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_F1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_F2_ST` reader - Represents MCPWM0_evt_f2 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_F2_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_F2_ST` writer - Represents MCPWM0_evt_f2 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_F2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_F0_CLR_ST` reader - Represents MCPWM0_evt_f0_clr trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_F0_CLR_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_F0_CLR_ST` writer - Represents MCPWM0_evt_f0_clr trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_F0_CLR_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_F1_CLR_ST` reader - Represents MCPWM0_evt_f1_clr trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_F1_CLR_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_F1_CLR_ST` writer - Represents MCPWM0_evt_f1_clr trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_F1_CLR_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_F2_CLR_ST` reader - Represents MCPWM0_evt_f2_clr trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_F2_CLR_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_F2_CLR_ST` writer - Represents MCPWM0_evt_f2_clr trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_F2_CLR_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_TZ0_CBC_ST` reader - Represents MCPWM0_evt_tz0_cbc trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_TZ0_CBC_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_TZ0_CBC_ST` writer - Represents MCPWM0_evt_tz0_cbc trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_TZ0_CBC_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_TZ1_CBC_ST` reader - Represents MCPWM0_evt_tz1_cbc trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_TZ1_CBC_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_TZ1_CBC_ST` writer - Represents MCPWM0_evt_tz1_cbc trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_TZ1_CBC_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_TZ2_CBC_ST` reader - Represents MCPWM0_evt_tz2_cbc trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_TZ2_CBC_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_TZ2_CBC_ST` writer - Represents MCPWM0_evt_tz2_cbc trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_TZ2_CBC_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_TZ0_OST_ST` reader - Represents MCPWM0_evt_tz0_ost trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_TZ0_OST_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_TZ0_OST_ST` writer - Represents MCPWM0_evt_tz0_ost trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_TZ0_OST_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_TZ1_OST_ST` reader - Represents MCPWM0_evt_tz1_ost trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_TZ1_OST_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_TZ1_OST_ST` writer - Represents MCPWM0_evt_tz1_ost trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_TZ1_OST_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_TZ2_OST_ST` reader - Represents MCPWM0_evt_tz2_ost trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_TZ2_OST_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_TZ2_OST_ST` writer - Represents MCPWM0_evt_tz2_ost trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_TZ2_OST_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_CAP0_ST` reader - Represents MCPWM0_evt_cap0 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_CAP0_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_CAP0_ST` writer - Represents MCPWM0_evt_cap0 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_CAP0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_CAP1_ST` reader - Represents MCPWM0_evt_cap1 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_CAP1_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_CAP1_ST` writer - Represents MCPWM0_evt_cap1 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_CAP1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_CAP2_ST` reader - Represents MCPWM0_evt_cap2 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_CAP2_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_CAP2_ST` writer - Represents MCPWM0_evt_cap2 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_CAP2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_OP0_TEE1_ST` reader - Represents MCPWM0_evt_op0_tee1 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP0_TEE1_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_OP0_TEE1_ST` writer - Represents MCPWM0_evt_op0_tee1 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP0_TEE1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_OP1_TEE1_ST` reader - Represents MCPWM0_evt_op1_tee1 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP1_TEE1_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_OP1_TEE1_ST` writer - Represents MCPWM0_evt_op1_tee1 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP1_TEE1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_OP2_TEE1_ST` reader - Represents MCPWM0_evt_op2_tee1 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP2_TEE1_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_OP2_TEE1_ST` writer - Represents MCPWM0_evt_op2_tee1 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP2_TEE1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_OP0_TEE2_ST` reader - Represents MCPWM0_evt_op0_tee2 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP0_TEE2_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_OP0_TEE2_ST` writer - Represents MCPWM0_evt_op0_tee2 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP0_TEE2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_OP1_TEE2_ST` reader - Represents MCPWM0_evt_op1_tee2 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP1_TEE2_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_OP1_TEE2_ST` writer - Represents MCPWM0_evt_op1_tee2 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP1_TEE2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM0_EVT_OP2_TEE2_ST` reader - Represents MCPWM0_evt_op2_tee2 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP2_TEE2_ST_R = crate::BitReader;
///Field `MCPWM0_EVT_OP2_TEE2_ST` writer - Represents MCPWM0_evt_op2_tee2 trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM0_EVT_OP2_TEE2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCPWM1_EVT_TIMER0_STOP_ST` reader - Represents MCPWM1_evt_timer0_stop trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM1_EVT_TIMER0_STOP_ST_R = crate::BitReader;
///Field `MCPWM1_EVT_TIMER0_STOP_ST` writer - Represents MCPWM1_evt_timer0_stop trigger status.\\0: Not triggered\\1: Triggered
pub type MCPWM1_EVT_TIMER0_STOP_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Represents MCPWM0_evt_timer2_tez trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_timer2_tez_st(&self) -> MCPWM0_EVT_TIMER2_TEZ_ST_R {
        MCPWM0_EVT_TIMER2_TEZ_ST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Represents MCPWM0_evt_timer0_tep trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_timer0_tep_st(&self) -> MCPWM0_EVT_TIMER0_TEP_ST_R {
        MCPWM0_EVT_TIMER0_TEP_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Represents MCPWM0_evt_timer1_tep trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_timer1_tep_st(&self) -> MCPWM0_EVT_TIMER1_TEP_ST_R {
        MCPWM0_EVT_TIMER1_TEP_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Represents MCPWM0_evt_timer2_tep trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_timer2_tep_st(&self) -> MCPWM0_EVT_TIMER2_TEP_ST_R {
        MCPWM0_EVT_TIMER2_TEP_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Represents MCPWM0_evt_op0_tea trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_op0_tea_st(&self) -> MCPWM0_EVT_OP0_TEA_ST_R {
        MCPWM0_EVT_OP0_TEA_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Represents MCPWM0_evt_op1_tea trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_op1_tea_st(&self) -> MCPWM0_EVT_OP1_TEA_ST_R {
        MCPWM0_EVT_OP1_TEA_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Represents MCPWM0_evt_op2_tea trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_op2_tea_st(&self) -> MCPWM0_EVT_OP2_TEA_ST_R {
        MCPWM0_EVT_OP2_TEA_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Represents MCPWM0_evt_op0_teb trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_op0_teb_st(&self) -> MCPWM0_EVT_OP0_TEB_ST_R {
        MCPWM0_EVT_OP0_TEB_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Represents MCPWM0_evt_op1_teb trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_op1_teb_st(&self) -> MCPWM0_EVT_OP1_TEB_ST_R {
        MCPWM0_EVT_OP1_TEB_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Represents MCPWM0_evt_op2_teb trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_op2_teb_st(&self) -> MCPWM0_EVT_OP2_TEB_ST_R {
        MCPWM0_EVT_OP2_TEB_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Represents MCPWM0_evt_f0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_f0_st(&self) -> MCPWM0_EVT_F0_ST_R {
        MCPWM0_EVT_F0_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Represents MCPWM0_evt_f1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_f1_st(&self) -> MCPWM0_EVT_F1_ST_R {
        MCPWM0_EVT_F1_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Represents MCPWM0_evt_f2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_f2_st(&self) -> MCPWM0_EVT_F2_ST_R {
        MCPWM0_EVT_F2_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Represents MCPWM0_evt_f0_clr trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_f0_clr_st(&self) -> MCPWM0_EVT_F0_CLR_ST_R {
        MCPWM0_EVT_F0_CLR_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Represents MCPWM0_evt_f1_clr trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_f1_clr_st(&self) -> MCPWM0_EVT_F1_CLR_ST_R {
        MCPWM0_EVT_F1_CLR_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Represents MCPWM0_evt_f2_clr trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_f2_clr_st(&self) -> MCPWM0_EVT_F2_CLR_ST_R {
        MCPWM0_EVT_F2_CLR_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Represents MCPWM0_evt_tz0_cbc trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_tz0_cbc_st(&self) -> MCPWM0_EVT_TZ0_CBC_ST_R {
        MCPWM0_EVT_TZ0_CBC_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Represents MCPWM0_evt_tz1_cbc trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_tz1_cbc_st(&self) -> MCPWM0_EVT_TZ1_CBC_ST_R {
        MCPWM0_EVT_TZ1_CBC_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Represents MCPWM0_evt_tz2_cbc trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_tz2_cbc_st(&self) -> MCPWM0_EVT_TZ2_CBC_ST_R {
        MCPWM0_EVT_TZ2_CBC_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Represents MCPWM0_evt_tz0_ost trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_tz0_ost_st(&self) -> MCPWM0_EVT_TZ0_OST_ST_R {
        MCPWM0_EVT_TZ0_OST_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Represents MCPWM0_evt_tz1_ost trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_tz1_ost_st(&self) -> MCPWM0_EVT_TZ1_OST_ST_R {
        MCPWM0_EVT_TZ1_OST_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Represents MCPWM0_evt_tz2_ost trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_tz2_ost_st(&self) -> MCPWM0_EVT_TZ2_OST_ST_R {
        MCPWM0_EVT_TZ2_OST_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Represents MCPWM0_evt_cap0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_cap0_st(&self) -> MCPWM0_EVT_CAP0_ST_R {
        MCPWM0_EVT_CAP0_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Represents MCPWM0_evt_cap1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_cap1_st(&self) -> MCPWM0_EVT_CAP1_ST_R {
        MCPWM0_EVT_CAP1_ST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Represents MCPWM0_evt_cap2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_cap2_st(&self) -> MCPWM0_EVT_CAP2_ST_R {
        MCPWM0_EVT_CAP2_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Represents MCPWM0_evt_op0_tee1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_op0_tee1_st(&self) -> MCPWM0_EVT_OP0_TEE1_ST_R {
        MCPWM0_EVT_OP0_TEE1_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Represents MCPWM0_evt_op1_tee1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_op1_tee1_st(&self) -> MCPWM0_EVT_OP1_TEE1_ST_R {
        MCPWM0_EVT_OP1_TEE1_ST_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Represents MCPWM0_evt_op2_tee1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_op2_tee1_st(&self) -> MCPWM0_EVT_OP2_TEE1_ST_R {
        MCPWM0_EVT_OP2_TEE1_ST_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Represents MCPWM0_evt_op0_tee2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_op0_tee2_st(&self) -> MCPWM0_EVT_OP0_TEE2_ST_R {
        MCPWM0_EVT_OP0_TEE2_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Represents MCPWM0_evt_op1_tee2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_op1_tee2_st(&self) -> MCPWM0_EVT_OP1_TEE2_ST_R {
        MCPWM0_EVT_OP1_TEE2_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Represents MCPWM0_evt_op2_tee2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm0_evt_op2_tee2_st(&self) -> MCPWM0_EVT_OP2_TEE2_ST_R {
        MCPWM0_EVT_OP2_TEE2_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Represents MCPWM1_evt_timer0_stop trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn mcpwm1_evt_timer0_stop_st(&self) -> MCPWM1_EVT_TIMER0_STOP_ST_R {
        MCPWM1_EVT_TIMER0_STOP_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVT_ST2")
            .field("mcpwm0_evt_timer2_tez_st", &self.mcpwm0_evt_timer2_tez_st())
            .field("mcpwm0_evt_timer0_tep_st", &self.mcpwm0_evt_timer0_tep_st())
            .field("mcpwm0_evt_timer1_tep_st", &self.mcpwm0_evt_timer1_tep_st())
            .field("mcpwm0_evt_timer2_tep_st", &self.mcpwm0_evt_timer2_tep_st())
            .field("mcpwm0_evt_op0_tea_st", &self.mcpwm0_evt_op0_tea_st())
            .field("mcpwm0_evt_op1_tea_st", &self.mcpwm0_evt_op1_tea_st())
            .field("mcpwm0_evt_op2_tea_st", &self.mcpwm0_evt_op2_tea_st())
            .field("mcpwm0_evt_op0_teb_st", &self.mcpwm0_evt_op0_teb_st())
            .field("mcpwm0_evt_op1_teb_st", &self.mcpwm0_evt_op1_teb_st())
            .field("mcpwm0_evt_op2_teb_st", &self.mcpwm0_evt_op2_teb_st())
            .field("mcpwm0_evt_f0_st", &self.mcpwm0_evt_f0_st())
            .field("mcpwm0_evt_f1_st", &self.mcpwm0_evt_f1_st())
            .field("mcpwm0_evt_f2_st", &self.mcpwm0_evt_f2_st())
            .field("mcpwm0_evt_f0_clr_st", &self.mcpwm0_evt_f0_clr_st())
            .field("mcpwm0_evt_f1_clr_st", &self.mcpwm0_evt_f1_clr_st())
            .field("mcpwm0_evt_f2_clr_st", &self.mcpwm0_evt_f2_clr_st())
            .field("mcpwm0_evt_tz0_cbc_st", &self.mcpwm0_evt_tz0_cbc_st())
            .field("mcpwm0_evt_tz1_cbc_st", &self.mcpwm0_evt_tz1_cbc_st())
            .field("mcpwm0_evt_tz2_cbc_st", &self.mcpwm0_evt_tz2_cbc_st())
            .field("mcpwm0_evt_tz0_ost_st", &self.mcpwm0_evt_tz0_ost_st())
            .field("mcpwm0_evt_tz1_ost_st", &self.mcpwm0_evt_tz1_ost_st())
            .field("mcpwm0_evt_tz2_ost_st", &self.mcpwm0_evt_tz2_ost_st())
            .field("mcpwm0_evt_cap0_st", &self.mcpwm0_evt_cap0_st())
            .field("mcpwm0_evt_cap1_st", &self.mcpwm0_evt_cap1_st())
            .field("mcpwm0_evt_cap2_st", &self.mcpwm0_evt_cap2_st())
            .field("mcpwm0_evt_op0_tee1_st", &self.mcpwm0_evt_op0_tee1_st())
            .field("mcpwm0_evt_op1_tee1_st", &self.mcpwm0_evt_op1_tee1_st())
            .field("mcpwm0_evt_op2_tee1_st", &self.mcpwm0_evt_op2_tee1_st())
            .field("mcpwm0_evt_op0_tee2_st", &self.mcpwm0_evt_op0_tee2_st())
            .field("mcpwm0_evt_op1_tee2_st", &self.mcpwm0_evt_op1_tee2_st())
            .field("mcpwm0_evt_op2_tee2_st", &self.mcpwm0_evt_op2_tee2_st())
            .field(
                "mcpwm1_evt_timer0_stop_st",
                &self.mcpwm1_evt_timer0_stop_st(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - Represents MCPWM0_evt_timer2_tez trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_timer2_tez_st(&mut self) -> MCPWM0_EVT_TIMER2_TEZ_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_TIMER2_TEZ_ST_W::new(self, 0)
    }
    ///Bit 1 - Represents MCPWM0_evt_timer0_tep trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_timer0_tep_st(&mut self) -> MCPWM0_EVT_TIMER0_TEP_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_TIMER0_TEP_ST_W::new(self, 1)
    }
    ///Bit 2 - Represents MCPWM0_evt_timer1_tep trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_timer1_tep_st(&mut self) -> MCPWM0_EVT_TIMER1_TEP_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_TIMER1_TEP_ST_W::new(self, 2)
    }
    ///Bit 3 - Represents MCPWM0_evt_timer2_tep trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_timer2_tep_st(&mut self) -> MCPWM0_EVT_TIMER2_TEP_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_TIMER2_TEP_ST_W::new(self, 3)
    }
    ///Bit 4 - Represents MCPWM0_evt_op0_tea trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_op0_tea_st(&mut self) -> MCPWM0_EVT_OP0_TEA_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_OP0_TEA_ST_W::new(self, 4)
    }
    ///Bit 5 - Represents MCPWM0_evt_op1_tea trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_op1_tea_st(&mut self) -> MCPWM0_EVT_OP1_TEA_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_OP1_TEA_ST_W::new(self, 5)
    }
    ///Bit 6 - Represents MCPWM0_evt_op2_tea trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_op2_tea_st(&mut self) -> MCPWM0_EVT_OP2_TEA_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_OP2_TEA_ST_W::new(self, 6)
    }
    ///Bit 7 - Represents MCPWM0_evt_op0_teb trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_op0_teb_st(&mut self) -> MCPWM0_EVT_OP0_TEB_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_OP0_TEB_ST_W::new(self, 7)
    }
    ///Bit 8 - Represents MCPWM0_evt_op1_teb trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_op1_teb_st(&mut self) -> MCPWM0_EVT_OP1_TEB_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_OP1_TEB_ST_W::new(self, 8)
    }
    ///Bit 9 - Represents MCPWM0_evt_op2_teb trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_op2_teb_st(&mut self) -> MCPWM0_EVT_OP2_TEB_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_OP2_TEB_ST_W::new(self, 9)
    }
    ///Bit 10 - Represents MCPWM0_evt_f0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_f0_st(&mut self) -> MCPWM0_EVT_F0_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_F0_ST_W::new(self, 10)
    }
    ///Bit 11 - Represents MCPWM0_evt_f1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_f1_st(&mut self) -> MCPWM0_EVT_F1_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_F1_ST_W::new(self, 11)
    }
    ///Bit 12 - Represents MCPWM0_evt_f2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_f2_st(&mut self) -> MCPWM0_EVT_F2_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_F2_ST_W::new(self, 12)
    }
    ///Bit 13 - Represents MCPWM0_evt_f0_clr trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_f0_clr_st(&mut self) -> MCPWM0_EVT_F0_CLR_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_F0_CLR_ST_W::new(self, 13)
    }
    ///Bit 14 - Represents MCPWM0_evt_f1_clr trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_f1_clr_st(&mut self) -> MCPWM0_EVT_F1_CLR_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_F1_CLR_ST_W::new(self, 14)
    }
    ///Bit 15 - Represents MCPWM0_evt_f2_clr trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_f2_clr_st(&mut self) -> MCPWM0_EVT_F2_CLR_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_F2_CLR_ST_W::new(self, 15)
    }
    ///Bit 16 - Represents MCPWM0_evt_tz0_cbc trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_tz0_cbc_st(&mut self) -> MCPWM0_EVT_TZ0_CBC_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_TZ0_CBC_ST_W::new(self, 16)
    }
    ///Bit 17 - Represents MCPWM0_evt_tz1_cbc trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_tz1_cbc_st(&mut self) -> MCPWM0_EVT_TZ1_CBC_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_TZ1_CBC_ST_W::new(self, 17)
    }
    ///Bit 18 - Represents MCPWM0_evt_tz2_cbc trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_tz2_cbc_st(&mut self) -> MCPWM0_EVT_TZ2_CBC_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_TZ2_CBC_ST_W::new(self, 18)
    }
    ///Bit 19 - Represents MCPWM0_evt_tz0_ost trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_tz0_ost_st(&mut self) -> MCPWM0_EVT_TZ0_OST_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_TZ0_OST_ST_W::new(self, 19)
    }
    ///Bit 20 - Represents MCPWM0_evt_tz1_ost trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_tz1_ost_st(&mut self) -> MCPWM0_EVT_TZ1_OST_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_TZ1_OST_ST_W::new(self, 20)
    }
    ///Bit 21 - Represents MCPWM0_evt_tz2_ost trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_tz2_ost_st(&mut self) -> MCPWM0_EVT_TZ2_OST_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_TZ2_OST_ST_W::new(self, 21)
    }
    ///Bit 22 - Represents MCPWM0_evt_cap0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_cap0_st(&mut self) -> MCPWM0_EVT_CAP0_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_CAP0_ST_W::new(self, 22)
    }
    ///Bit 23 - Represents MCPWM0_evt_cap1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_cap1_st(&mut self) -> MCPWM0_EVT_CAP1_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_CAP1_ST_W::new(self, 23)
    }
    ///Bit 24 - Represents MCPWM0_evt_cap2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_cap2_st(&mut self) -> MCPWM0_EVT_CAP2_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_CAP2_ST_W::new(self, 24)
    }
    ///Bit 25 - Represents MCPWM0_evt_op0_tee1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_op0_tee1_st(&mut self) -> MCPWM0_EVT_OP0_TEE1_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_OP0_TEE1_ST_W::new(self, 25)
    }
    ///Bit 26 - Represents MCPWM0_evt_op1_tee1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_op1_tee1_st(&mut self) -> MCPWM0_EVT_OP1_TEE1_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_OP1_TEE1_ST_W::new(self, 26)
    }
    ///Bit 27 - Represents MCPWM0_evt_op2_tee1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_op2_tee1_st(&mut self) -> MCPWM0_EVT_OP2_TEE1_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_OP2_TEE1_ST_W::new(self, 27)
    }
    ///Bit 28 - Represents MCPWM0_evt_op0_tee2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_op0_tee2_st(&mut self) -> MCPWM0_EVT_OP0_TEE2_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_OP0_TEE2_ST_W::new(self, 28)
    }
    ///Bit 29 - Represents MCPWM0_evt_op1_tee2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_op1_tee2_st(&mut self) -> MCPWM0_EVT_OP1_TEE2_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_OP1_TEE2_ST_W::new(self, 29)
    }
    ///Bit 30 - Represents MCPWM0_evt_op2_tee2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm0_evt_op2_tee2_st(&mut self) -> MCPWM0_EVT_OP2_TEE2_ST_W<EVT_ST2_SPEC> {
        MCPWM0_EVT_OP2_TEE2_ST_W::new(self, 30)
    }
    ///Bit 31 - Represents MCPWM1_evt_timer0_stop trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn mcpwm1_evt_timer0_stop_st(&mut self) -> MCPWM1_EVT_TIMER0_STOP_ST_W<EVT_ST2_SPEC> {
        MCPWM1_EVT_TIMER0_STOP_ST_W::new(self, 31)
    }
}
/**Events trigger status register

You can [`read`](crate::generic::Reg::read) this register and get [`evt_st2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_st2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EVT_ST2_SPEC;
impl crate::RegisterSpec for EVT_ST2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`evt_st2::R`](R) reader structure
impl crate::Readable for EVT_ST2_SPEC {}
///`write(|w| ..)` method takes [`evt_st2::W`](W) writer structure
impl crate::Writable for EVT_ST2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EVT_ST2 to value 0
impl crate::Resettable for EVT_ST2_SPEC {
    const RESET_VALUE: u32 = 0;
}
