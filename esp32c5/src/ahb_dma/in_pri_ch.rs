#[doc = "Register `IN_PRI_CH%s` reader"]
pub type R = crate::R<IN_PRI_CH_SPEC>;
#[doc = "Register `IN_PRI_CH%s` writer"]
pub type W = crate::W<IN_PRI_CH_SPEC>;
#[doc = "Field `RX_PRI` reader - Configures the priority of RX channel %s.The larger of the value, the higher of the priority."]
pub type RX_PRI_R = crate::FieldReader;
#[doc = "Field `RX_PRI` writer - Configures the priority of RX channel %s.The larger of the value, the higher of the priority."]
pub type RX_PRI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configures the priority of RX channel %s.The larger of the value, the higher of the priority."]
    #[inline(always)]
    pub fn rx_pri(&self) -> RX_PRI_R {
        RX_PRI_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_PRI_CH")
            .field("rx_pri", &self.rx_pri())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Configures the priority of RX channel %s.The larger of the value, the higher of the priority."]
    #[inline(always)]
    pub fn rx_pri(&mut self) -> RX_PRI_W<IN_PRI_CH_SPEC> {
        RX_PRI_W::new(self, 0)
    }
}
#[doc = "Priority register of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_pri_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_pri_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_PRI_CH_SPEC;
impl crate::RegisterSpec for IN_PRI_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_pri_ch::R`](R) reader structure"]
impl crate::Readable for IN_PRI_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_pri_ch::W`](W) writer structure"]
impl crate::Writable for IN_PRI_CH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_PRI_CH%s to value 0"]
impl crate::Resettable for IN_PRI_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
