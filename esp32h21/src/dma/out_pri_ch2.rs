#[doc = "Register `OUT_PRI_CH2` reader"]
pub type R = crate::R<OUT_PRI_CH2_SPEC>;
#[doc = "Register `OUT_PRI_CH2` writer"]
pub type W = crate::W<OUT_PRI_CH2_SPEC>;
#[doc = "Field `TX_PRI_CH2` reader - The priority of Tx channel 2. The larger of the value the higher of the priority."]
pub type TX_PRI_CH2_R = crate::FieldReader;
#[doc = "Field `TX_PRI_CH2` writer - The priority of Tx channel 2. The larger of the value the higher of the priority."]
pub type TX_PRI_CH2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - The priority of Tx channel 2. The larger of the value the higher of the priority."]
    #[inline(always)]
    pub fn tx_pri_ch2(&self) -> TX_PRI_CH2_R {
        TX_PRI_CH2_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_PRI_CH2")
            .field("tx_pri_ch2", &self.tx_pri_ch2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - The priority of Tx channel 2. The larger of the value the higher of the priority."]
    #[inline(always)]
    pub fn tx_pri_ch2(&mut self) -> TX_PRI_CH2_W<'_, OUT_PRI_CH2_SPEC> {
        TX_PRI_CH2_W::new(self, 0)
    }
}
#[doc = "Priority register of Tx channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`out_pri_ch2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_pri_ch2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_PRI_CH2_SPEC;
impl crate::RegisterSpec for OUT_PRI_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_pri_ch2::R`](R) reader structure"]
impl crate::Readable for OUT_PRI_CH2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_pri_ch2::W`](W) writer structure"]
impl crate::Writable for OUT_PRI_CH2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_PRI_CH2 to value 0"]
impl crate::Resettable for OUT_PRI_CH2_SPEC {}
