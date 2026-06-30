#[doc = "Register `IDEAL_CNT` reader"]
pub type R = crate::R<IDEAL_CNT_SPEC>;
#[doc = "Register `IDEAL_CNT` writer"]
pub type W = crate::W<IDEAL_CNT_SPEC>;
#[doc = "Field `TX_IDEAL_CNT` reader - tx fifo counter ideal value."]
pub type TX_IDEAL_CNT_R = crate::FieldReader<u32>;
#[doc = "Field `TX_IDEAL_CNT` writer - tx fifo counter ideal value."]
pub type TX_IDEAL_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - tx fifo counter ideal value."]
    #[inline(always)]
    pub fn tx_ideal_cnt(&self) -> TX_IDEAL_CNT_R {
        TX_IDEAL_CNT_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDEAL_CNT")
            .field("tx_ideal_cnt", &self.tx_ideal_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:30 - tx fifo counter ideal value."]
    #[inline(always)]
    pub fn tx_ideal_cnt(&mut self) -> TX_IDEAL_CNT_W<'_, IDEAL_CNT_SPEC> {
        TX_IDEAL_CNT_W::new(self, 0)
    }
}
#[doc = "I2S sync counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ideal_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ideal_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDEAL_CNT_SPEC;
impl crate::RegisterSpec for IDEAL_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ideal_cnt::R`](R) reader structure"]
impl crate::Readable for IDEAL_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ideal_cnt::W`](W) writer structure"]
impl crate::Writable for IDEAL_CNT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDEAL_CNT to value 0"]
impl crate::Resettable for IDEAL_CNT_SPEC {}
