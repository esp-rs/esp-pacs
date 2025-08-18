#[doc = "Register `IN_PERI_CH0` reader"]
pub type R = crate::R<IN_PERI_CH0_SPEC>;
#[doc = "Register `IN_PERI_CH0` writer"]
pub type W = crate::W<IN_PERI_CH0_SPEC>;
#[doc = "Field `RX_PRI_CH0` reader - Configures the priority of RX channel 0.The larger of the value, the higher of the priority.."]
pub type RX_PRI_CH0_R = crate::FieldReader;
#[doc = "Field `RX_PRI_CH0` writer - Configures the priority of RX channel 0.The larger of the value, the higher of the priority.."]
pub type RX_PRI_CH0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configures the priority of RX channel 0.The larger of the value, the higher of the priority.."]
    #[inline(always)]
    pub fn rx_pri_ch0(&self) -> RX_PRI_CH0_R {
        RX_PRI_CH0_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_PERI_CH0")
            .field("rx_pri_ch0", &self.rx_pri_ch0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Configures the priority of RX channel 0.The larger of the value, the higher of the priority.."]
    #[inline(always)]
    pub fn rx_pri_ch0(&mut self) -> RX_PRI_CH0_W<'_, IN_PERI_CH0_SPEC> {
        RX_PRI_CH0_W::new(self, 0)
    }
}
#[doc = "Priority register of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_peri_ch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_peri_ch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_PERI_CH0_SPEC;
impl crate::RegisterSpec for IN_PERI_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_peri_ch0::R`](R) reader structure"]
impl crate::Readable for IN_PERI_CH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_peri_ch0::W`](W) writer structure"]
impl crate::Writable for IN_PERI_CH0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_PERI_CH0 to value 0"]
impl crate::Resettable for IN_PERI_CH0_SPEC {}
