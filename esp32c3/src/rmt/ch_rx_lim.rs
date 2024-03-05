#[doc = "Register `CH%s_RX_LIM` reader"]
pub type R = crate::R<CH_RX_LIM_SPEC>;
#[doc = "Register `CH%s_RX_LIM` writer"]
pub type W = crate::W<CH_RX_LIM_SPEC>;
#[doc = "Field `RX_LIM` reader - reg_rmt_rx_lim_ch2."]
pub type RX_LIM_R = crate::FieldReader<u16>;
#[doc = "Field `RX_LIM` writer - reg_rmt_rx_lim_ch2."]
pub type RX_LIM_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - reg_rmt_rx_lim_ch2."]
    #[inline(always)]
    pub fn rx_lim(&self) -> RX_LIM_R {
        RX_LIM_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_RX_LIM")
            .field("rx_lim", &format_args!("{}", self.rx_lim().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_RX_LIM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:8 - reg_rmt_rx_lim_ch2."]
    #[inline(always)]
    #[must_use]
    pub fn rx_lim(&mut self) -> RX_LIM_W<CH_RX_LIM_SPEC> {
        RX_LIM_W::new(self, 0)
    }
}
#[doc = "RMT_CH2_RX_LIM_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_rx_lim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_rx_lim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_RX_LIM_SPEC;
impl crate::RegisterSpec for CH_RX_LIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_rx_lim::R`](R) reader structure"]
impl crate::Readable for CH_RX_LIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_rx_lim::W`](W) writer structure"]
impl crate::Writable for CH_RX_LIM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH%s_RX_LIM to value 0x80"]
impl crate::Resettable for CH_RX_LIM_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
