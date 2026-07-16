#[doc = "Register `FIFO_CNT` reader"]
pub type R = crate::R<FIFO_CNT_SPEC>;
#[doc = "Register `FIFO_CNT` writer"]
pub type W = crate::W<FIFO_CNT_SPEC>;
#[doc = "Field `TX_FIFO_CNT` reader - tx fifo counter value."]
pub type TX_FIFO_CNT_R = crate::FieldReader<u32>;
#[doc = "Field `TX_FIFO_CNT_RST` writer - Set this bit to reset tx fifo counter."]
pub type TX_FIFO_CNT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - tx fifo counter value."]
    #[inline(always)]
    pub fn tx_fifo_cnt(&self) -> TX_FIFO_CNT_R {
        TX_FIFO_CNT_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_CNT")
            .field("tx_fifo_cnt", &self.tx_fifo_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - Set this bit to reset tx fifo counter."]
    #[inline(always)]
    pub fn tx_fifo_cnt_rst(&mut self) -> TX_FIFO_CNT_RST_W<'_, FIFO_CNT_SPEC> {
        TX_FIFO_CNT_RST_W::new(self, 31)
    }
}
#[doc = "I2S sync counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_CNT_SPEC;
impl crate::RegisterSpec for FIFO_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_cnt::R`](R) reader structure"]
impl crate::Readable for FIFO_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo_cnt::W`](W) writer structure"]
impl crate::Writable for FIFO_CNT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO_CNT to value 0"]
impl crate::Resettable for FIFO_CNT_SPEC {}
