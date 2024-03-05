#[doc = "Register `CH%s_TX_LIM` reader"]
pub type R = crate::R<CH_TX_LIM_SPEC>;
#[doc = "Register `CH%s_TX_LIM` writer"]
pub type W = crate::W<CH_TX_LIM_SPEC>;
#[doc = "Field `TX_LIM` reader - reg_rmt_tx_lim_ch0."]
pub type TX_LIM_R = crate::FieldReader<u16>;
#[doc = "Field `TX_LIM` writer - reg_rmt_tx_lim_ch0."]
pub type TX_LIM_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TX_LOOP_NUM` reader - reg_rmt_tx_loop_num_ch0."]
pub type TX_LOOP_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `TX_LOOP_NUM` writer - reg_rmt_tx_loop_num_ch0."]
pub type TX_LOOP_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TX_LOOP_CNT_EN` reader - reg_rmt_tx_loop_cnt_en_ch0."]
pub type TX_LOOP_CNT_EN_R = crate::BitReader;
#[doc = "Field `TX_LOOP_CNT_EN` writer - reg_rmt_tx_loop_cnt_en_ch0."]
pub type TX_LOOP_CNT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOP_COUNT_RESET` writer - reg_loop_count_reset_ch0."]
pub type LOOP_COUNT_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - reg_rmt_tx_lim_ch0."]
    #[inline(always)]
    pub fn tx_lim(&self) -> TX_LIM_R {
        TX_LIM_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:18 - reg_rmt_tx_loop_num_ch0."]
    #[inline(always)]
    pub fn tx_loop_num(&self) -> TX_LOOP_NUM_R {
        TX_LOOP_NUM_R::new(((self.bits >> 9) & 0x03ff) as u16)
    }
    #[doc = "Bit 19 - reg_rmt_tx_loop_cnt_en_ch0."]
    #[inline(always)]
    pub fn tx_loop_cnt_en(&self) -> TX_LOOP_CNT_EN_R {
        TX_LOOP_CNT_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_TX_LIM")
            .field("tx_lim", &format_args!("{}", self.tx_lim().bits()))
            .field(
                "tx_loop_num",
                &format_args!("{}", self.tx_loop_num().bits()),
            )
            .field(
                "tx_loop_cnt_en",
                &format_args!("{}", self.tx_loop_cnt_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_TX_LIM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:8 - reg_rmt_tx_lim_ch0."]
    #[inline(always)]
    #[must_use]
    pub fn tx_lim(&mut self) -> TX_LIM_W<CH_TX_LIM_SPEC> {
        TX_LIM_W::new(self, 0)
    }
    #[doc = "Bits 9:18 - reg_rmt_tx_loop_num_ch0."]
    #[inline(always)]
    #[must_use]
    pub fn tx_loop_num(&mut self) -> TX_LOOP_NUM_W<CH_TX_LIM_SPEC> {
        TX_LOOP_NUM_W::new(self, 9)
    }
    #[doc = "Bit 19 - reg_rmt_tx_loop_cnt_en_ch0."]
    #[inline(always)]
    #[must_use]
    pub fn tx_loop_cnt_en(&mut self) -> TX_LOOP_CNT_EN_W<CH_TX_LIM_SPEC> {
        TX_LOOP_CNT_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - reg_loop_count_reset_ch0."]
    #[inline(always)]
    #[must_use]
    pub fn loop_count_reset(&mut self) -> LOOP_COUNT_RESET_W<CH_TX_LIM_SPEC> {
        LOOP_COUNT_RESET_W::new(self, 20)
    }
}
#[doc = "RMT_CH%s_TX_LIM_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_tx_lim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_tx_lim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_TX_LIM_SPEC;
impl crate::RegisterSpec for CH_TX_LIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_tx_lim::R`](R) reader structure"]
impl crate::Readable for CH_TX_LIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_tx_lim::W`](W) writer structure"]
impl crate::Writable for CH_TX_LIM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH%s_TX_LIM to value 0x80"]
impl crate::Resettable for CH_TX_LIM_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
