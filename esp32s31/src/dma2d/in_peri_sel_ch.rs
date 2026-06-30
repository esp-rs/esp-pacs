#[doc = "Register `IN_PERI_SEL_CH%s` reader"]
pub type R = crate::R<IN_PERI_SEL_CH_SPEC>;
#[doc = "Register `IN_PERI_SEL_CH%s` writer"]
pub type W = crate::W<IN_PERI_SEL_CH_SPEC>;
#[doc = "Field `IN_PERI_SEL_CH` reader - This register is used to select peripheral for Rx channel 0: jpeg 1: display-1 2: display-2 7: no choose"]
pub type IN_PERI_SEL_CH_R = crate::FieldReader;
#[doc = "Field `IN_PERI_SEL_CH` writer - This register is used to select peripheral for Rx channel 0: jpeg 1: display-1 2: display-2 7: no choose"]
pub type IN_PERI_SEL_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - This register is used to select peripheral for Rx channel 0: jpeg 1: display-1 2: display-2 7: no choose"]
    #[inline(always)]
    pub fn in_peri_sel_ch(&self) -> IN_PERI_SEL_CH_R {
        IN_PERI_SEL_CH_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_PERI_SEL_CH")
            .field("in_peri_sel_ch", &self.in_peri_sel_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - This register is used to select peripheral for Rx channel 0: jpeg 1: display-1 2: display-2 7: no choose"]
    #[inline(always)]
    pub fn in_peri_sel_ch(&mut self) -> IN_PERI_SEL_CH_W<'_, IN_PERI_SEL_CH_SPEC> {
        IN_PERI_SEL_CH_W::new(self, 0)
    }
}
#[doc = "Configures the rx peripheral of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_peri_sel_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_peri_sel_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_PERI_SEL_CH_SPEC;
impl crate::RegisterSpec for IN_PERI_SEL_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_peri_sel_ch::R`](R) reader structure"]
impl crate::Readable for IN_PERI_SEL_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_peri_sel_ch::W`](W) writer structure"]
impl crate::Writable for IN_PERI_SEL_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_PERI_SEL_CH%s to value 0x07"]
impl crate::Resettable for IN_PERI_SEL_CH_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
