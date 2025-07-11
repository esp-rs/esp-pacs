#[doc = "Register `OUT_PERI_CH0` reader"]
pub type R = crate::R<OUT_PERI_CH0_SPEC>;
#[doc = "Register `OUT_PERI_CH0` writer"]
pub type W = crate::W<OUT_PERI_CH0_SPEC>;
#[doc = "Field `TX_PRI_CH0` reader - Configures the priority of TX channel 0.The larger of the value, the higher of the priority.."]
pub type TX_PRI_CH0_R = crate::FieldReader;
#[doc = "Field `TX_PRI_CH0` writer - Configures the priority of TX channel 0.The larger of the value, the higher of the priority.."]
pub type TX_PRI_CH0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configures the priority of TX channel 0.The larger of the value, the higher of the priority.."]
    #[inline(always)]
    pub fn tx_pri_ch0(&self) -> TX_PRI_CH0_R {
        TX_PRI_CH0_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_PERI_CH0")
            .field("tx_pri_ch0", &self.tx_pri_ch0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Configures the priority of TX channel 0.The larger of the value, the higher of the priority.."]
    #[inline(always)]
    pub fn tx_pri_ch0(&mut self) -> TX_PRI_CH0_W<OUT_PERI_CH0_SPEC> {
        TX_PRI_CH0_W::new(self, 0)
    }
}
#[doc = "Priority register of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_peri_ch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_peri_ch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_PERI_CH0_SPEC;
impl crate::RegisterSpec for OUT_PERI_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_peri_ch0::R`](R) reader structure"]
impl crate::Readable for OUT_PERI_CH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_peri_ch0::W`](W) writer structure"]
impl crate::Writable for OUT_PERI_CH0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_PERI_CH0 to value 0"]
impl crate::Resettable for OUT_PERI_CH0_SPEC {}
