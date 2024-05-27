#[doc = "Register `CH%s_TX_LIM` reader"]
pub type R = crate::R<CH_TX_LIM_SPEC>;
#[doc = "Register `CH%s_TX_LIM` writer"]
pub type W = crate::W<CH_TX_LIM_SPEC>;
#[doc = "Field `TX_LIM` reader - When channel0 sends more than reg_rmt_tx_lim_ch0 datas then channel0 produce the relative interrupt."]
pub type TX_LIM_R = crate::FieldReader<u16>;
#[doc = "Field `TX_LIM` writer - When channel0 sends more than reg_rmt_tx_lim_ch0 datas then channel0 produce the relative interrupt."]
pub type TX_LIM_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - When channel0 sends more than reg_rmt_tx_lim_ch0 datas then channel0 produce the relative interrupt."]
    #[inline(always)]
    pub fn tx_lim(&self) -> TX_LIM_R {
        TX_LIM_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_TX_LIM")
            .field("tx_lim", &self.tx_lim())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - When channel0 sends more than reg_rmt_tx_lim_ch0 datas then channel0 produce the relative interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_lim(&mut self) -> TX_LIM_W<CH_TX_LIM_SPEC> {
        TX_LIM_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_tx_lim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_tx_lim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
