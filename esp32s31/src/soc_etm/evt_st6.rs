#[doc = "Register `EVT_ST6` reader"]
pub type R = crate::R<EVT_ST6_SPEC>;
#[doc = "Register `EVT_ST6` writer"]
pub type W = crate::W<EVT_ST6_SPEC>;
#[doc = "Field `MCPWM2_EVT_TZ2_CBC_ST` reader - Represents MCPWM2_evt_tz2_cbc trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_TZ2_CBC_ST_R = crate::BitReader;
#[doc = "Field `MCPWM2_EVT_TZ2_CBC_ST` writer - Represents MCPWM2_evt_tz2_cbc trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_TZ2_CBC_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM2_EVT_TZ0_OST_ST` reader - Represents MCPWM2_evt_tz0_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_TZ0_OST_ST_R = crate::BitReader;
#[doc = "Field `MCPWM2_EVT_TZ0_OST_ST` writer - Represents MCPWM2_evt_tz0_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_TZ0_OST_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM2_EVT_TZ1_OST_ST` reader - Represents MCPWM2_evt_tz1_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_TZ1_OST_ST_R = crate::BitReader;
#[doc = "Field `MCPWM2_EVT_TZ1_OST_ST` writer - Represents MCPWM2_evt_tz1_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_TZ1_OST_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM2_EVT_TZ2_OST_ST` reader - Represents MCPWM2_evt_tz2_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_TZ2_OST_ST_R = crate::BitReader;
#[doc = "Field `MCPWM2_EVT_TZ2_OST_ST` writer - Represents MCPWM2_evt_tz2_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_TZ2_OST_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM2_EVT_CAP0_ST` reader - Represents MCPWM2_evt_cap0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_CAP0_ST_R = crate::BitReader;
#[doc = "Field `MCPWM2_EVT_CAP0_ST` writer - Represents MCPWM2_evt_cap0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_CAP0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM2_EVT_CAP1_ST` reader - Represents MCPWM2_evt_cap1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_CAP1_ST_R = crate::BitReader;
#[doc = "Field `MCPWM2_EVT_CAP1_ST` writer - Represents MCPWM2_evt_cap1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_CAP1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM2_EVT_CAP2_ST` reader - Represents MCPWM2_evt_cap2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_CAP2_ST_R = crate::BitReader;
#[doc = "Field `MCPWM2_EVT_CAP2_ST` writer - Represents MCPWM2_evt_cap2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_CAP2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM2_EVT_OP0_TEE1_ST` reader - Represents MCPWM2_evt_op0_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_OP0_TEE1_ST_R = crate::BitReader;
#[doc = "Field `MCPWM2_EVT_OP0_TEE1_ST` writer - Represents MCPWM2_evt_op0_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_OP0_TEE1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM2_EVT_OP1_TEE1_ST` reader - Represents MCPWM2_evt_op1_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_OP1_TEE1_ST_R = crate::BitReader;
#[doc = "Field `MCPWM2_EVT_OP1_TEE1_ST` writer - Represents MCPWM2_evt_op1_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_OP1_TEE1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM2_EVT_OP2_TEE1_ST` reader - Represents MCPWM2_evt_op2_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_OP2_TEE1_ST_R = crate::BitReader;
#[doc = "Field `MCPWM2_EVT_OP2_TEE1_ST` writer - Represents MCPWM2_evt_op2_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_OP2_TEE1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM2_EVT_OP0_TEE2_ST` reader - Represents MCPWM2_evt_op0_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_OP0_TEE2_ST_R = crate::BitReader;
#[doc = "Field `MCPWM2_EVT_OP0_TEE2_ST` writer - Represents MCPWM2_evt_op0_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_OP0_TEE2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM2_EVT_OP1_TEE2_ST` reader - Represents MCPWM2_evt_op1_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_OP1_TEE2_ST_R = crate::BitReader;
#[doc = "Field `MCPWM2_EVT_OP1_TEE2_ST` writer - Represents MCPWM2_evt_op1_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_OP1_TEE2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM2_EVT_OP2_TEE2_ST` reader - Represents MCPWM2_evt_op2_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_OP2_TEE2_ST_R = crate::BitReader;
#[doc = "Field `MCPWM2_EVT_OP2_TEE2_ST` writer - Represents MCPWM2_evt_op2_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM2_EVT_OP2_TEE2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_EVT_TIMER0_STOP_ST` reader - Represents MCPWM3_evt_timer0_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER0_STOP_ST_R = crate::BitReader;
#[doc = "Field `MCPWM3_EVT_TIMER0_STOP_ST` writer - Represents MCPWM3_evt_timer0_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER0_STOP_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_EVT_TIMER1_STOP_ST` reader - Represents MCPWM3_evt_timer1_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER1_STOP_ST_R = crate::BitReader;
#[doc = "Field `MCPWM3_EVT_TIMER1_STOP_ST` writer - Represents MCPWM3_evt_timer1_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER1_STOP_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_EVT_TIMER2_STOP_ST` reader - Represents MCPWM3_evt_timer2_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER2_STOP_ST_R = crate::BitReader;
#[doc = "Field `MCPWM3_EVT_TIMER2_STOP_ST` writer - Represents MCPWM3_evt_timer2_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER2_STOP_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_EVT_TIMER0_TEZ_ST` reader - Represents MCPWM3_evt_timer0_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER0_TEZ_ST_R = crate::BitReader;
#[doc = "Field `MCPWM3_EVT_TIMER0_TEZ_ST` writer - Represents MCPWM3_evt_timer0_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER0_TEZ_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_EVT_TIMER1_TEZ_ST` reader - Represents MCPWM3_evt_timer1_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER1_TEZ_ST_R = crate::BitReader;
#[doc = "Field `MCPWM3_EVT_TIMER1_TEZ_ST` writer - Represents MCPWM3_evt_timer1_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER1_TEZ_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_EVT_TIMER2_TEZ_ST` reader - Represents MCPWM3_evt_timer2_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER2_TEZ_ST_R = crate::BitReader;
#[doc = "Field `MCPWM3_EVT_TIMER2_TEZ_ST` writer - Represents MCPWM3_evt_timer2_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER2_TEZ_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_EVT_TIMER0_TEP_ST` reader - Represents MCPWM3_evt_timer0_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER0_TEP_ST_R = crate::BitReader;
#[doc = "Field `MCPWM3_EVT_TIMER0_TEP_ST` writer - Represents MCPWM3_evt_timer0_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER0_TEP_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_EVT_TIMER1_TEP_ST` reader - Represents MCPWM3_evt_timer1_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER1_TEP_ST_R = crate::BitReader;
#[doc = "Field `MCPWM3_EVT_TIMER1_TEP_ST` writer - Represents MCPWM3_evt_timer1_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER1_TEP_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_EVT_TIMER2_TEP_ST` reader - Represents MCPWM3_evt_timer2_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER2_TEP_ST_R = crate::BitReader;
#[doc = "Field `MCPWM3_EVT_TIMER2_TEP_ST` writer - Represents MCPWM3_evt_timer2_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER2_TEP_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_EVT_TIMER0_TEA_ST` reader - Represents MCPWM3_evt_timer0_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER0_TEA_ST_R = crate::BitReader;
#[doc = "Field `MCPWM3_EVT_TIMER0_TEA_ST` writer - Represents MCPWM3_evt_timer0_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER0_TEA_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_EVT_TIMER1_TEA_ST` reader - Represents MCPWM3_evt_timer1_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER1_TEA_ST_R = crate::BitReader;
#[doc = "Field `MCPWM3_EVT_TIMER1_TEA_ST` writer - Represents MCPWM3_evt_timer1_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER1_TEA_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_EVT_TIMER2_TEA_ST` reader - Represents MCPWM3_evt_timer2_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER2_TEA_ST_R = crate::BitReader;
#[doc = "Field `MCPWM3_EVT_TIMER2_TEA_ST` writer - Represents MCPWM3_evt_timer2_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER2_TEA_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_EVT_TIMER0_TEB_ST` reader - Represents MCPWM3_evt_timer0_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER0_TEB_ST_R = crate::BitReader;
#[doc = "Field `MCPWM3_EVT_TIMER0_TEB_ST` writer - Represents MCPWM3_evt_timer0_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER0_TEB_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_EVT_TIMER1_TEB_ST` reader - Represents MCPWM3_evt_timer1_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER1_TEB_ST_R = crate::BitReader;
#[doc = "Field `MCPWM3_EVT_TIMER1_TEB_ST` writer - Represents MCPWM3_evt_timer1_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER1_TEB_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_EVT_TIMER2_TEB_ST` reader - Represents MCPWM3_evt_timer2_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER2_TEB_ST_R = crate::BitReader;
#[doc = "Field `MCPWM3_EVT_TIMER2_TEB_ST` writer - Represents MCPWM3_evt_timer2_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_TIMER2_TEB_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_EVT_OP0_TEA_ST` reader - Represents MCPWM3_evt_op0_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_OP0_TEA_ST_R = crate::BitReader;
#[doc = "Field `MCPWM3_EVT_OP0_TEA_ST` writer - Represents MCPWM3_evt_op0_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_OP0_TEA_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_EVT_OP1_TEA_ST` reader - Represents MCPWM3_evt_op1_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_OP1_TEA_ST_R = crate::BitReader;
#[doc = "Field `MCPWM3_EVT_OP1_TEA_ST` writer - Represents MCPWM3_evt_op1_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_OP1_TEA_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_EVT_OP2_TEA_ST` reader - Represents MCPWM3_evt_op2_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_OP2_TEA_ST_R = crate::BitReader;
#[doc = "Field `MCPWM3_EVT_OP2_TEA_ST` writer - Represents MCPWM3_evt_op2_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_OP2_TEA_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_EVT_OP0_TEB_ST` reader - Represents MCPWM3_evt_op0_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_OP0_TEB_ST_R = crate::BitReader;
#[doc = "Field `MCPWM3_EVT_OP0_TEB_ST` writer - Represents MCPWM3_evt_op0_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM3_EVT_OP0_TEB_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents MCPWM2_evt_tz2_cbc trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_tz2_cbc_st(&self) -> MCPWM2_EVT_TZ2_CBC_ST_R {
        MCPWM2_EVT_TZ2_CBC_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents MCPWM2_evt_tz0_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_tz0_ost_st(&self) -> MCPWM2_EVT_TZ0_OST_ST_R {
        MCPWM2_EVT_TZ0_OST_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents MCPWM2_evt_tz1_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_tz1_ost_st(&self) -> MCPWM2_EVT_TZ1_OST_ST_R {
        MCPWM2_EVT_TZ1_OST_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents MCPWM2_evt_tz2_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_tz2_ost_st(&self) -> MCPWM2_EVT_TZ2_OST_ST_R {
        MCPWM2_EVT_TZ2_OST_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents MCPWM2_evt_cap0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_cap0_st(&self) -> MCPWM2_EVT_CAP0_ST_R {
        MCPWM2_EVT_CAP0_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents MCPWM2_evt_cap1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_cap1_st(&self) -> MCPWM2_EVT_CAP1_ST_R {
        MCPWM2_EVT_CAP1_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents MCPWM2_evt_cap2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_cap2_st(&self) -> MCPWM2_EVT_CAP2_ST_R {
        MCPWM2_EVT_CAP2_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents MCPWM2_evt_op0_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_op0_tee1_st(&self) -> MCPWM2_EVT_OP0_TEE1_ST_R {
        MCPWM2_EVT_OP0_TEE1_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents MCPWM2_evt_op1_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_op1_tee1_st(&self) -> MCPWM2_EVT_OP1_TEE1_ST_R {
        MCPWM2_EVT_OP1_TEE1_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents MCPWM2_evt_op2_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_op2_tee1_st(&self) -> MCPWM2_EVT_OP2_TEE1_ST_R {
        MCPWM2_EVT_OP2_TEE1_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents MCPWM2_evt_op0_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_op0_tee2_st(&self) -> MCPWM2_EVT_OP0_TEE2_ST_R {
        MCPWM2_EVT_OP0_TEE2_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents MCPWM2_evt_op1_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_op1_tee2_st(&self) -> MCPWM2_EVT_OP1_TEE2_ST_R {
        MCPWM2_EVT_OP1_TEE2_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents MCPWM2_evt_op2_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_op2_tee2_st(&self) -> MCPWM2_EVT_OP2_TEE2_ST_R {
        MCPWM2_EVT_OP2_TEE2_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents MCPWM3_evt_timer0_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer0_stop_st(&self) -> MCPWM3_EVT_TIMER0_STOP_ST_R {
        MCPWM3_EVT_TIMER0_STOP_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents MCPWM3_evt_timer1_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer1_stop_st(&self) -> MCPWM3_EVT_TIMER1_STOP_ST_R {
        MCPWM3_EVT_TIMER1_STOP_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Represents MCPWM3_evt_timer2_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer2_stop_st(&self) -> MCPWM3_EVT_TIMER2_STOP_ST_R {
        MCPWM3_EVT_TIMER2_STOP_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Represents MCPWM3_evt_timer0_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer0_tez_st(&self) -> MCPWM3_EVT_TIMER0_TEZ_ST_R {
        MCPWM3_EVT_TIMER0_TEZ_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents MCPWM3_evt_timer1_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer1_tez_st(&self) -> MCPWM3_EVT_TIMER1_TEZ_ST_R {
        MCPWM3_EVT_TIMER1_TEZ_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Represents MCPWM3_evt_timer2_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer2_tez_st(&self) -> MCPWM3_EVT_TIMER2_TEZ_ST_R {
        MCPWM3_EVT_TIMER2_TEZ_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Represents MCPWM3_evt_timer0_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer0_tep_st(&self) -> MCPWM3_EVT_TIMER0_TEP_ST_R {
        MCPWM3_EVT_TIMER0_TEP_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents MCPWM3_evt_timer1_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer1_tep_st(&self) -> MCPWM3_EVT_TIMER1_TEP_ST_R {
        MCPWM3_EVT_TIMER1_TEP_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents MCPWM3_evt_timer2_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer2_tep_st(&self) -> MCPWM3_EVT_TIMER2_TEP_ST_R {
        MCPWM3_EVT_TIMER2_TEP_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Represents MCPWM3_evt_timer0_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer0_tea_st(&self) -> MCPWM3_EVT_TIMER0_TEA_ST_R {
        MCPWM3_EVT_TIMER0_TEA_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Represents MCPWM3_evt_timer1_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer1_tea_st(&self) -> MCPWM3_EVT_TIMER1_TEA_ST_R {
        MCPWM3_EVT_TIMER1_TEA_ST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Represents MCPWM3_evt_timer2_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer2_tea_st(&self) -> MCPWM3_EVT_TIMER2_TEA_ST_R {
        MCPWM3_EVT_TIMER2_TEA_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Represents MCPWM3_evt_timer0_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer0_teb_st(&self) -> MCPWM3_EVT_TIMER0_TEB_ST_R {
        MCPWM3_EVT_TIMER0_TEB_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Represents MCPWM3_evt_timer1_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer1_teb_st(&self) -> MCPWM3_EVT_TIMER1_TEB_ST_R {
        MCPWM3_EVT_TIMER1_TEB_ST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Represents MCPWM3_evt_timer2_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer2_teb_st(&self) -> MCPWM3_EVT_TIMER2_TEB_ST_R {
        MCPWM3_EVT_TIMER2_TEB_ST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Represents MCPWM3_evt_op0_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_op0_tea_st(&self) -> MCPWM3_EVT_OP0_TEA_ST_R {
        MCPWM3_EVT_OP0_TEA_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Represents MCPWM3_evt_op1_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_op1_tea_st(&self) -> MCPWM3_EVT_OP1_TEA_ST_R {
        MCPWM3_EVT_OP1_TEA_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Represents MCPWM3_evt_op2_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_op2_tea_st(&self) -> MCPWM3_EVT_OP2_TEA_ST_R {
        MCPWM3_EVT_OP2_TEA_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Represents MCPWM3_evt_op0_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_op0_teb_st(&self) -> MCPWM3_EVT_OP0_TEB_ST_R {
        MCPWM3_EVT_OP0_TEB_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVT_ST6")
            .field("mcpwm2_evt_tz2_cbc_st", &self.mcpwm2_evt_tz2_cbc_st())
            .field("mcpwm2_evt_tz0_ost_st", &self.mcpwm2_evt_tz0_ost_st())
            .field("mcpwm2_evt_tz1_ost_st", &self.mcpwm2_evt_tz1_ost_st())
            .field("mcpwm2_evt_tz2_ost_st", &self.mcpwm2_evt_tz2_ost_st())
            .field("mcpwm2_evt_cap0_st", &self.mcpwm2_evt_cap0_st())
            .field("mcpwm2_evt_cap1_st", &self.mcpwm2_evt_cap1_st())
            .field("mcpwm2_evt_cap2_st", &self.mcpwm2_evt_cap2_st())
            .field("mcpwm2_evt_op0_tee1_st", &self.mcpwm2_evt_op0_tee1_st())
            .field("mcpwm2_evt_op1_tee1_st", &self.mcpwm2_evt_op1_tee1_st())
            .field("mcpwm2_evt_op2_tee1_st", &self.mcpwm2_evt_op2_tee1_st())
            .field("mcpwm2_evt_op0_tee2_st", &self.mcpwm2_evt_op0_tee2_st())
            .field("mcpwm2_evt_op1_tee2_st", &self.mcpwm2_evt_op1_tee2_st())
            .field("mcpwm2_evt_op2_tee2_st", &self.mcpwm2_evt_op2_tee2_st())
            .field(
                "mcpwm3_evt_timer0_stop_st",
                &self.mcpwm3_evt_timer0_stop_st(),
            )
            .field(
                "mcpwm3_evt_timer1_stop_st",
                &self.mcpwm3_evt_timer1_stop_st(),
            )
            .field(
                "mcpwm3_evt_timer2_stop_st",
                &self.mcpwm3_evt_timer2_stop_st(),
            )
            .field("mcpwm3_evt_timer0_tez_st", &self.mcpwm3_evt_timer0_tez_st())
            .field("mcpwm3_evt_timer1_tez_st", &self.mcpwm3_evt_timer1_tez_st())
            .field("mcpwm3_evt_timer2_tez_st", &self.mcpwm3_evt_timer2_tez_st())
            .field("mcpwm3_evt_timer0_tep_st", &self.mcpwm3_evt_timer0_tep_st())
            .field("mcpwm3_evt_timer1_tep_st", &self.mcpwm3_evt_timer1_tep_st())
            .field("mcpwm3_evt_timer2_tep_st", &self.mcpwm3_evt_timer2_tep_st())
            .field("mcpwm3_evt_timer0_tea_st", &self.mcpwm3_evt_timer0_tea_st())
            .field("mcpwm3_evt_timer1_tea_st", &self.mcpwm3_evt_timer1_tea_st())
            .field("mcpwm3_evt_timer2_tea_st", &self.mcpwm3_evt_timer2_tea_st())
            .field("mcpwm3_evt_timer0_teb_st", &self.mcpwm3_evt_timer0_teb_st())
            .field("mcpwm3_evt_timer1_teb_st", &self.mcpwm3_evt_timer1_teb_st())
            .field("mcpwm3_evt_timer2_teb_st", &self.mcpwm3_evt_timer2_teb_st())
            .field("mcpwm3_evt_op0_tea_st", &self.mcpwm3_evt_op0_tea_st())
            .field("mcpwm3_evt_op1_tea_st", &self.mcpwm3_evt_op1_tea_st())
            .field("mcpwm3_evt_op2_tea_st", &self.mcpwm3_evt_op2_tea_st())
            .field("mcpwm3_evt_op0_teb_st", &self.mcpwm3_evt_op0_teb_st())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Represents MCPWM2_evt_tz2_cbc trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_tz2_cbc_st(&mut self) -> MCPWM2_EVT_TZ2_CBC_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM2_EVT_TZ2_CBC_ST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Represents MCPWM2_evt_tz0_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_tz0_ost_st(&mut self) -> MCPWM2_EVT_TZ0_OST_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM2_EVT_TZ0_OST_ST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Represents MCPWM2_evt_tz1_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_tz1_ost_st(&mut self) -> MCPWM2_EVT_TZ1_OST_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM2_EVT_TZ1_OST_ST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Represents MCPWM2_evt_tz2_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_tz2_ost_st(&mut self) -> MCPWM2_EVT_TZ2_OST_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM2_EVT_TZ2_OST_ST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Represents MCPWM2_evt_cap0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_cap0_st(&mut self) -> MCPWM2_EVT_CAP0_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM2_EVT_CAP0_ST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Represents MCPWM2_evt_cap1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_cap1_st(&mut self) -> MCPWM2_EVT_CAP1_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM2_EVT_CAP1_ST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Represents MCPWM2_evt_cap2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_cap2_st(&mut self) -> MCPWM2_EVT_CAP2_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM2_EVT_CAP2_ST_W::new(self, 6)
    }
    #[doc = "Bit 7 - Represents MCPWM2_evt_op0_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_op0_tee1_st(&mut self) -> MCPWM2_EVT_OP0_TEE1_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM2_EVT_OP0_TEE1_ST_W::new(self, 7)
    }
    #[doc = "Bit 8 - Represents MCPWM2_evt_op1_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_op1_tee1_st(&mut self) -> MCPWM2_EVT_OP1_TEE1_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM2_EVT_OP1_TEE1_ST_W::new(self, 8)
    }
    #[doc = "Bit 9 - Represents MCPWM2_evt_op2_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_op2_tee1_st(&mut self) -> MCPWM2_EVT_OP2_TEE1_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM2_EVT_OP2_TEE1_ST_W::new(self, 9)
    }
    #[doc = "Bit 10 - Represents MCPWM2_evt_op0_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_op0_tee2_st(&mut self) -> MCPWM2_EVT_OP0_TEE2_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM2_EVT_OP0_TEE2_ST_W::new(self, 10)
    }
    #[doc = "Bit 11 - Represents MCPWM2_evt_op1_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_op1_tee2_st(&mut self) -> MCPWM2_EVT_OP1_TEE2_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM2_EVT_OP1_TEE2_ST_W::new(self, 11)
    }
    #[doc = "Bit 12 - Represents MCPWM2_evt_op2_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm2_evt_op2_tee2_st(&mut self) -> MCPWM2_EVT_OP2_TEE2_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM2_EVT_OP2_TEE2_ST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Represents MCPWM3_evt_timer0_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer0_stop_st(&mut self) -> MCPWM3_EVT_TIMER0_STOP_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM3_EVT_TIMER0_STOP_ST_W::new(self, 13)
    }
    #[doc = "Bit 14 - Represents MCPWM3_evt_timer1_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer1_stop_st(&mut self) -> MCPWM3_EVT_TIMER1_STOP_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM3_EVT_TIMER1_STOP_ST_W::new(self, 14)
    }
    #[doc = "Bit 15 - Represents MCPWM3_evt_timer2_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer2_stop_st(&mut self) -> MCPWM3_EVT_TIMER2_STOP_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM3_EVT_TIMER2_STOP_ST_W::new(self, 15)
    }
    #[doc = "Bit 16 - Represents MCPWM3_evt_timer0_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer0_tez_st(&mut self) -> MCPWM3_EVT_TIMER0_TEZ_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM3_EVT_TIMER0_TEZ_ST_W::new(self, 16)
    }
    #[doc = "Bit 17 - Represents MCPWM3_evt_timer1_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer1_tez_st(&mut self) -> MCPWM3_EVT_TIMER1_TEZ_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM3_EVT_TIMER1_TEZ_ST_W::new(self, 17)
    }
    #[doc = "Bit 18 - Represents MCPWM3_evt_timer2_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer2_tez_st(&mut self) -> MCPWM3_EVT_TIMER2_TEZ_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM3_EVT_TIMER2_TEZ_ST_W::new(self, 18)
    }
    #[doc = "Bit 19 - Represents MCPWM3_evt_timer0_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer0_tep_st(&mut self) -> MCPWM3_EVT_TIMER0_TEP_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM3_EVT_TIMER0_TEP_ST_W::new(self, 19)
    }
    #[doc = "Bit 20 - Represents MCPWM3_evt_timer1_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer1_tep_st(&mut self) -> MCPWM3_EVT_TIMER1_TEP_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM3_EVT_TIMER1_TEP_ST_W::new(self, 20)
    }
    #[doc = "Bit 21 - Represents MCPWM3_evt_timer2_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer2_tep_st(&mut self) -> MCPWM3_EVT_TIMER2_TEP_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM3_EVT_TIMER2_TEP_ST_W::new(self, 21)
    }
    #[doc = "Bit 22 - Represents MCPWM3_evt_timer0_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer0_tea_st(&mut self) -> MCPWM3_EVT_TIMER0_TEA_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM3_EVT_TIMER0_TEA_ST_W::new(self, 22)
    }
    #[doc = "Bit 23 - Represents MCPWM3_evt_timer1_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer1_tea_st(&mut self) -> MCPWM3_EVT_TIMER1_TEA_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM3_EVT_TIMER1_TEA_ST_W::new(self, 23)
    }
    #[doc = "Bit 24 - Represents MCPWM3_evt_timer2_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer2_tea_st(&mut self) -> MCPWM3_EVT_TIMER2_TEA_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM3_EVT_TIMER2_TEA_ST_W::new(self, 24)
    }
    #[doc = "Bit 25 - Represents MCPWM3_evt_timer0_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer0_teb_st(&mut self) -> MCPWM3_EVT_TIMER0_TEB_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM3_EVT_TIMER0_TEB_ST_W::new(self, 25)
    }
    #[doc = "Bit 26 - Represents MCPWM3_evt_timer1_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer1_teb_st(&mut self) -> MCPWM3_EVT_TIMER1_TEB_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM3_EVT_TIMER1_TEB_ST_W::new(self, 26)
    }
    #[doc = "Bit 27 - Represents MCPWM3_evt_timer2_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_timer2_teb_st(&mut self) -> MCPWM3_EVT_TIMER2_TEB_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM3_EVT_TIMER2_TEB_ST_W::new(self, 27)
    }
    #[doc = "Bit 28 - Represents MCPWM3_evt_op0_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_op0_tea_st(&mut self) -> MCPWM3_EVT_OP0_TEA_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM3_EVT_OP0_TEA_ST_W::new(self, 28)
    }
    #[doc = "Bit 29 - Represents MCPWM3_evt_op1_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_op1_tea_st(&mut self) -> MCPWM3_EVT_OP1_TEA_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM3_EVT_OP1_TEA_ST_W::new(self, 29)
    }
    #[doc = "Bit 30 - Represents MCPWM3_evt_op2_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_op2_tea_st(&mut self) -> MCPWM3_EVT_OP2_TEA_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM3_EVT_OP2_TEA_ST_W::new(self, 30)
    }
    #[doc = "Bit 31 - Represents MCPWM3_evt_op0_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm3_evt_op0_teb_st(&mut self) -> MCPWM3_EVT_OP0_TEB_ST_W<'_, EVT_ST6_SPEC> {
        MCPWM3_EVT_OP0_TEB_ST_W::new(self, 31)
    }
}
#[doc = "Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVT_ST6_SPEC;
impl crate::RegisterSpec for EVT_ST6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evt_st6::R`](R) reader structure"]
impl crate::Readable for EVT_ST6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evt_st6::W`](W) writer structure"]
impl crate::Writable for EVT_ST6_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVT_ST6 to value 0"]
impl crate::Resettable for EVT_ST6_SPEC {}
