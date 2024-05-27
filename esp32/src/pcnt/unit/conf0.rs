///Register `CONF0` reader
pub type R = crate::R<CONF0_SPEC>;
///Register `CONF0` writer
pub type W = crate::W<CONF0_SPEC>;
///Field `FILTER_THRES` reader - This register is used to filter pluse whose width is smaller than this value for unit0.
pub type FILTER_THRES_R = crate::FieldReader<u16>;
///Field `FILTER_THRES` writer - This register is used to filter pluse whose width is smaller than this value for unit0.
pub type FILTER_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `FILTER_EN` reader - This is the enable bit for filtering input signals for unit0.
pub type FILTER_EN_R = crate::BitReader;
///Field `FILTER_EN` writer - This is the enable bit for filtering input signals for unit0.
pub type FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THR_ZERO_EN` reader - This is the enable bit for comparing unit0's count with 0 value.
pub type THR_ZERO_EN_R = crate::BitReader;
///Field `THR_ZERO_EN` writer - This is the enable bit for comparing unit0's count with 0 value.
pub type THR_ZERO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THR_H_LIM_EN` reader - This is the enable bit for comparing unit0's count with thr_h_lim value.
pub type THR_H_LIM_EN_R = crate::BitReader;
///Field `THR_H_LIM_EN` writer - This is the enable bit for comparing unit0's count with thr_h_lim value.
pub type THR_H_LIM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THR_L_LIM_EN` reader - This is the enable bit for comparing unit0's count with thr_l_lim value.
pub type THR_L_LIM_EN_R = crate::BitReader;
///Field `THR_L_LIM_EN` writer - This is the enable bit for comparing unit0's count with thr_l_lim value.
pub type THR_L_LIM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THR_THRES0_EN` reader - This is the enable bit for comparing unit0's count with thres0 value.
pub type THR_THRES0_EN_R = crate::BitReader;
///Field `THR_THRES0_EN` writer - This is the enable bit for comparing unit0's count with thres0 value.
pub type THR_THRES0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THR_THRES1_EN` reader - This is the enable bit for comparing unit0's count with thres1 value .
pub type THR_THRES1_EN_R = crate::BitReader;
///Field `THR_THRES1_EN` writer - This is the enable bit for comparing unit0's count with thres1 value .
pub type THR_THRES1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Configures the behavior when the signal input of channel %s detects a negative edge.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDGE_MODE {
    ///1: Increase the counter
    Increment = 1,
    ///2: Decrease the counter
    Decrement = 2,
    ///0: No effect on counter
    Hold = 0,
}
impl From<EDGE_MODE> for u8 {
    #[inline(always)]
    fn from(variant: EDGE_MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EDGE_MODE {
    type Ux = u8;
}
impl crate::IsEnum for EDGE_MODE {}
///Field `CH_NEG_MODE(0-1)` reader - Configures the behavior when the signal input of channel %s detects a negative edge.
pub type CH_NEG_MODE_R = crate::FieldReader<EDGE_MODE>;
impl CH_NEG_MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EDGE_MODE {
        match self.bits {
            1 => EDGE_MODE::Increment,
            2 => EDGE_MODE::Decrement,
            _ => EDGE_MODE::Hold,
        }
    }
    ///Increase the counter
    #[inline(always)]
    pub fn is_increment(&self) -> bool {
        *self == EDGE_MODE::Increment
    }
    ///Decrease the counter
    #[inline(always)]
    pub fn is_decrement(&self) -> bool {
        *self == EDGE_MODE::Decrement
    }
    ///No effect on counter
    #[inline(always)]
    pub fn is_hold(&self) -> bool {
        matches!(self.variant(), EDGE_MODE::Hold)
    }
}
///Field `CH_NEG_MODE(0-1)` writer - Configures the behavior when the signal input of channel %s detects a negative edge.
pub type CH_NEG_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EDGE_MODE, crate::Safe>;
impl<'a, REG> CH_NEG_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Increase the counter
    #[inline(always)]
    pub fn increment(self) -> &'a mut crate::W<REG> {
        self.variant(EDGE_MODE::Increment)
    }
    ///Decrease the counter
    #[inline(always)]
    pub fn decrement(self) -> &'a mut crate::W<REG> {
        self.variant(EDGE_MODE::Decrement)
    }
    ///No effect on counter
    #[inline(always)]
    pub fn hold(self) -> &'a mut crate::W<REG> {
        self.variant(EDGE_MODE::Hold)
    }
}
///Field `CH_POS_MODE(0-1)` reader - Configures the behavior when the signal input of channel %s detects a positive edge.
pub use CH_NEG_MODE_R as CH_POS_MODE_R;
///Field `CH_POS_MODE(0-1)` writer - Configures the behavior when the signal input of channel %s detects a positive edge.
pub use CH_NEG_MODE_W as CH_POS_MODE_W;
/**Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is high.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTRL_MODE {
    ///0: No modification
    Keep = 0,
    ///1: Invert behavior (increase -> decrease
    Reverse = 1,
    ///2: Inhibit counter modification
    Disable = 2,
}
impl From<CTRL_MODE> for u8 {
    #[inline(always)]
    fn from(variant: CTRL_MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTRL_MODE {
    type Ux = u8;
}
impl crate::IsEnum for CTRL_MODE {}
///Field `CH_HCTRL_MODE(0-1)` reader - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is high.
pub type CH_HCTRL_MODE_R = crate::FieldReader<CTRL_MODE>;
impl CH_HCTRL_MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTRL_MODE {
        match self.bits {
            0 => CTRL_MODE::Keep,
            1 => CTRL_MODE::Reverse,
            _ => CTRL_MODE::Disable,
        }
    }
    ///No modification
    #[inline(always)]
    pub fn is_keep(&self) -> bool {
        *self == CTRL_MODE::Keep
    }
    ///Invert behavior (increase -> decrease
    #[inline(always)]
    pub fn is_reverse(&self) -> bool {
        *self == CTRL_MODE::Reverse
    }
    ///Inhibit counter modification
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        matches!(self.variant(), CTRL_MODE::Disable)
    }
}
///Field `CH_HCTRL_MODE(0-1)` writer - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is high.
pub type CH_HCTRL_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CTRL_MODE, crate::Safe>;
impl<'a, REG> CH_HCTRL_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No modification
    #[inline(always)]
    pub fn keep(self) -> &'a mut crate::W<REG> {
        self.variant(CTRL_MODE::Keep)
    }
    ///Invert behavior (increase -> decrease
    #[inline(always)]
    pub fn reverse(self) -> &'a mut crate::W<REG> {
        self.variant(CTRL_MODE::Reverse)
    }
    ///Inhibit counter modification
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CTRL_MODE::Disable)
    }
}
///Field `CH_LCTRL_MODE(0-1)` reader - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is low.
pub use CH_HCTRL_MODE_R as CH_LCTRL_MODE_R;
///Field `CH_LCTRL_MODE(0-1)` writer - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is low.
pub use CH_HCTRL_MODE_W as CH_LCTRL_MODE_W;
impl R {
    ///Bits 0:9 - This register is used to filter pluse whose width is smaller than this value for unit0.
    #[inline(always)]
    pub fn filter_thres(&self) -> FILTER_THRES_R {
        FILTER_THRES_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 10 - This is the enable bit for filtering input signals for unit0.
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - This is the enable bit for comparing unit0's count with 0 value.
    #[inline(always)]
    pub fn thr_zero_en(&self) -> THR_ZERO_EN_R {
        THR_ZERO_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - This is the enable bit for comparing unit0's count with thr_h_lim value.
    #[inline(always)]
    pub fn thr_h_lim_en(&self) -> THR_H_LIM_EN_R {
        THR_H_LIM_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - This is the enable bit for comparing unit0's count with thr_l_lim value.
    #[inline(always)]
    pub fn thr_l_lim_en(&self) -> THR_L_LIM_EN_R {
        THR_L_LIM_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - This is the enable bit for comparing unit0's count with thres0 value.
    #[inline(always)]
    pub fn thr_thres0_en(&self) -> THR_THRES0_EN_R {
        THR_THRES0_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - This is the enable bit for comparing unit0's count with thres1 value .
    #[inline(always)]
    pub fn thr_thres1_en(&self) -> THR_THRES1_EN_R {
        THR_THRES1_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Configures the behavior when the signal input of channel (0-1) detects a negative edge.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_NEG_MODE` field
    #[inline(always)]
    pub fn ch_neg_mode(&self, n: u8) -> CH_NEG_MODE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_NEG_MODE_R::new(((self.bits >> (n * 8 + 16)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Configures the behavior when the signal input of channel (0-1) detects a negative edge.
    #[inline(always)]
    pub fn ch_neg_mode_iter(&self) -> impl Iterator<Item = CH_NEG_MODE_R> + '_ {
        (0..2).map(move |n| CH_NEG_MODE_R::new(((self.bits >> (n * 8 + 16)) & 3) as u8))
    }
    ///Bits 16:17 - Configures the behavior when the signal input of channel 0 detects a negative edge.
    #[inline(always)]
    pub fn ch0_neg_mode(&self) -> CH_NEG_MODE_R {
        CH_NEG_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 24:25 - Configures the behavior when the signal input of channel 1 detects a negative edge.
    #[inline(always)]
    pub fn ch1_neg_mode(&self) -> CH_NEG_MODE_R {
        CH_NEG_MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Configures the behavior when the signal input of channel (0-1) detects a positive edge.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_POS_MODE` field
    #[inline(always)]
    pub fn ch_pos_mode(&self, n: u8) -> CH_POS_MODE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_POS_MODE_R::new(((self.bits >> (n * 8 + 18)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Configures the behavior when the signal input of channel (0-1) detects a positive edge.
    #[inline(always)]
    pub fn ch_pos_mode_iter(&self) -> impl Iterator<Item = CH_POS_MODE_R> + '_ {
        (0..2).map(move |n| CH_POS_MODE_R::new(((self.bits >> (n * 8 + 18)) & 3) as u8))
    }
    ///Bits 18:19 - Configures the behavior when the signal input of channel 0 detects a positive edge.
    #[inline(always)]
    pub fn ch0_pos_mode(&self) -> CH_POS_MODE_R {
        CH_POS_MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 26:27 - Configures the behavior when the signal input of channel 1 detects a positive edge.
    #[inline(always)]
    pub fn ch1_pos_mode(&self) -> CH_POS_MODE_R {
        CH_POS_MODE_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is high.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_HCTRL_MODE` field
    #[inline(always)]
    pub fn ch_hctrl_mode(&self, n: u8) -> CH_HCTRL_MODE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_HCTRL_MODE_R::new(((self.bits >> (n * 8 + 20)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is high.
    #[inline(always)]
    pub fn ch_hctrl_mode_iter(&self) -> impl Iterator<Item = CH_HCTRL_MODE_R> + '_ {
        (0..2).map(move |n| CH_HCTRL_MODE_R::new(((self.bits >> (n * 8 + 20)) & 3) as u8))
    }
    ///Bits 20:21 - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is high.
    #[inline(always)]
    pub fn ch0_hctrl_mode(&self) -> CH_HCTRL_MODE_R {
        CH_HCTRL_MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 28:29 - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is high.
    #[inline(always)]
    pub fn ch1_hctrl_mode(&self) -> CH_HCTRL_MODE_R {
        CH_HCTRL_MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is low.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_LCTRL_MODE` field
    #[inline(always)]
    pub fn ch_lctrl_mode(&self, n: u8) -> CH_LCTRL_MODE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_LCTRL_MODE_R::new(((self.bits >> (n * 8 + 22)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is low.
    #[inline(always)]
    pub fn ch_lctrl_mode_iter(&self) -> impl Iterator<Item = CH_LCTRL_MODE_R> + '_ {
        (0..2).map(move |n| CH_LCTRL_MODE_R::new(((self.bits >> (n * 8 + 22)) & 3) as u8))
    }
    ///Bits 22:23 - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is low.
    #[inline(always)]
    pub fn ch0_lctrl_mode(&self) -> CH_LCTRL_MODE_R {
        CH_LCTRL_MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 30:31 - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is low.
    #[inline(always)]
    pub fn ch1_lctrl_mode(&self) -> CH_LCTRL_MODE_R {
        CH_LCTRL_MODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF0")
            .field("filter_thres", &self.filter_thres())
            .field("filter_en", &self.filter_en())
            .field("thr_zero_en", &self.thr_zero_en())
            .field("thr_h_lim_en", &self.thr_h_lim_en())
            .field("thr_l_lim_en", &self.thr_l_lim_en())
            .field("thr_thres0_en", &self.thr_thres0_en())
            .field("thr_thres1_en", &self.thr_thres1_en())
            .field("ch0_neg_mode", &self.ch0_neg_mode())
            .field("ch1_neg_mode", &self.ch1_neg_mode())
            .field("ch0_pos_mode", &self.ch0_pos_mode())
            .field("ch1_pos_mode", &self.ch1_pos_mode())
            .field("ch0_hctrl_mode", &self.ch0_hctrl_mode())
            .field("ch1_hctrl_mode", &self.ch1_hctrl_mode())
            .field("ch0_lctrl_mode", &self.ch0_lctrl_mode())
            .field("ch1_lctrl_mode", &self.ch1_lctrl_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - This register is used to filter pluse whose width is smaller than this value for unit0.
    #[inline(always)]
    #[must_use]
    pub fn filter_thres(&mut self) -> FILTER_THRES_W<CONF0_SPEC> {
        FILTER_THRES_W::new(self, 0)
    }
    ///Bit 10 - This is the enable bit for filtering input signals for unit0.
    #[inline(always)]
    #[must_use]
    pub fn filter_en(&mut self) -> FILTER_EN_W<CONF0_SPEC> {
        FILTER_EN_W::new(self, 10)
    }
    ///Bit 11 - This is the enable bit for comparing unit0's count with 0 value.
    #[inline(always)]
    #[must_use]
    pub fn thr_zero_en(&mut self) -> THR_ZERO_EN_W<CONF0_SPEC> {
        THR_ZERO_EN_W::new(self, 11)
    }
    ///Bit 12 - This is the enable bit for comparing unit0's count with thr_h_lim value.
    #[inline(always)]
    #[must_use]
    pub fn thr_h_lim_en(&mut self) -> THR_H_LIM_EN_W<CONF0_SPEC> {
        THR_H_LIM_EN_W::new(self, 12)
    }
    ///Bit 13 - This is the enable bit for comparing unit0's count with thr_l_lim value.
    #[inline(always)]
    #[must_use]
    pub fn thr_l_lim_en(&mut self) -> THR_L_LIM_EN_W<CONF0_SPEC> {
        THR_L_LIM_EN_W::new(self, 13)
    }
    ///Bit 14 - This is the enable bit for comparing unit0's count with thres0 value.
    #[inline(always)]
    #[must_use]
    pub fn thr_thres0_en(&mut self) -> THR_THRES0_EN_W<CONF0_SPEC> {
        THR_THRES0_EN_W::new(self, 14)
    }
    ///Bit 15 - This is the enable bit for comparing unit0's count with thres1 value .
    #[inline(always)]
    #[must_use]
    pub fn thr_thres1_en(&mut self) -> THR_THRES1_EN_W<CONF0_SPEC> {
        THR_THRES1_EN_W::new(self, 15)
    }
    ///Configures the behavior when the signal input of channel (0-1) detects a negative edge.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_NEG_MODE` field
    #[inline(always)]
    #[must_use]
    pub fn ch_neg_mode(&mut self, n: u8) -> CH_NEG_MODE_W<CONF0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_NEG_MODE_W::new(self, n * 8 + 16)
    }
    ///Bits 16:17 - Configures the behavior when the signal input of channel 0 detects a negative edge.
    #[inline(always)]
    #[must_use]
    pub fn ch0_neg_mode(&mut self) -> CH_NEG_MODE_W<CONF0_SPEC> {
        CH_NEG_MODE_W::new(self, 16)
    }
    ///Bits 24:25 - Configures the behavior when the signal input of channel 1 detects a negative edge.
    #[inline(always)]
    #[must_use]
    pub fn ch1_neg_mode(&mut self) -> CH_NEG_MODE_W<CONF0_SPEC> {
        CH_NEG_MODE_W::new(self, 24)
    }
    ///Configures the behavior when the signal input of channel (0-1) detects a positive edge.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_POS_MODE` field
    #[inline(always)]
    #[must_use]
    pub fn ch_pos_mode(&mut self, n: u8) -> CH_POS_MODE_W<CONF0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_POS_MODE_W::new(self, n * 8 + 18)
    }
    ///Bits 18:19 - Configures the behavior when the signal input of channel 0 detects a positive edge.
    #[inline(always)]
    #[must_use]
    pub fn ch0_pos_mode(&mut self) -> CH_POS_MODE_W<CONF0_SPEC> {
        CH_POS_MODE_W::new(self, 18)
    }
    ///Bits 26:27 - Configures the behavior when the signal input of channel 1 detects a positive edge.
    #[inline(always)]
    #[must_use]
    pub fn ch1_pos_mode(&mut self) -> CH_POS_MODE_W<CONF0_SPEC> {
        CH_POS_MODE_W::new(self, 26)
    }
    ///Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is high.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_HCTRL_MODE` field
    #[inline(always)]
    #[must_use]
    pub fn ch_hctrl_mode(&mut self, n: u8) -> CH_HCTRL_MODE_W<CONF0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_HCTRL_MODE_W::new(self, n * 8 + 20)
    }
    ///Bits 20:21 - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is high.
    #[inline(always)]
    #[must_use]
    pub fn ch0_hctrl_mode(&mut self) -> CH_HCTRL_MODE_W<CONF0_SPEC> {
        CH_HCTRL_MODE_W::new(self, 20)
    }
    ///Bits 28:29 - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is high.
    #[inline(always)]
    #[must_use]
    pub fn ch1_hctrl_mode(&mut self) -> CH_HCTRL_MODE_W<CONF0_SPEC> {
        CH_HCTRL_MODE_W::new(self, 28)
    }
    ///Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is low.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_LCTRL_MODE` field
    #[inline(always)]
    #[must_use]
    pub fn ch_lctrl_mode(&mut self, n: u8) -> CH_LCTRL_MODE_W<CONF0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_LCTRL_MODE_W::new(self, n * 8 + 22)
    }
    ///Bits 22:23 - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is low.
    #[inline(always)]
    #[must_use]
    pub fn ch0_lctrl_mode(&mut self) -> CH_LCTRL_MODE_W<CONF0_SPEC> {
        CH_LCTRL_MODE_W::new(self, 22)
    }
    ///Bits 30:31 - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is low.
    #[inline(always)]
    #[must_use]
    pub fn ch1_lctrl_mode(&mut self) -> CH_LCTRL_MODE_W<CONF0_SPEC> {
        CH_LCTRL_MODE_W::new(self, 30)
    }
}
/**Configuration register 0 for unit

You can [`read`](crate::generic::Reg::read) this register and get [`conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CONF0_SPEC;
impl crate::RegisterSpec for CONF0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`conf0::R`](R) reader structure
impl crate::Readable for CONF0_SPEC {}
///`write(|w| ..)` method takes [`conf0::W`](W) writer structure
impl crate::Writable for CONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CONF0 to value 0x3c10
impl crate::Resettable for CONF0_SPEC {
    const RESET_VALUE: u32 = 0x3c10;
}
