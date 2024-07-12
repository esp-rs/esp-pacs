#[doc = "Register `RX_CH_ARB_WEIGH` reader"]
pub type R = crate::R<RX_CH_ARB_WEIGH_SPEC>;
#[doc = "Register `RX_CH_ARB_WEIGH` writer"]
pub type W = crate::W<RX_CH_ARB_WEIGH_SPEC>;
#[doc = "Field `RX_CH_ARB_WEIGH` reader - reserved"]
pub type RX_CH_ARB_WEIGH_R = crate::FieldReader;
#[doc = "Field `RX_CH_ARB_WEIGH` writer - reserved"]
pub type RX_CH_ARB_WEIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - reserved"]
    #[inline(always)]
    pub fn rx_ch_arb_weigh(&self) -> RX_CH_ARB_WEIGH_R {
        RX_CH_ARB_WEIGH_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CH_ARB_WEIGH")
            .field("rx_ch_arb_weigh", &self.rx_ch_arb_weigh())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn rx_ch_arb_weigh(&mut self) -> RX_CH_ARB_WEIGH_W<RX_CH_ARB_WEIGH_SPEC> {
        RX_CH_ARB_WEIGH_W::new(self, 0)
    }
}
#[doc = "This register is used to config ch0 arbiter weigh\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ch_arb_weigh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch_arb_weigh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CH_ARB_WEIGH_SPEC;
impl crate::RegisterSpec for RX_CH_ARB_WEIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ch_arb_weigh::R`](R) reader structure"]
impl crate::Readable for RX_CH_ARB_WEIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_ch_arb_weigh::W`](W) writer structure"]
impl crate::Writable for RX_CH_ARB_WEIGH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_CH_ARB_WEIGH to value 0"]
impl crate::Resettable for RX_CH_ARB_WEIGH_SPEC {
    const RESET_VALUE: u32 = 0;
}
