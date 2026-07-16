#[doc = "Register `CNT_DIFF` reader"]
pub type R = crate::R<CNT_DIFF_SPEC>;
#[doc = "Register `CNT_DIFF` writer"]
pub type W = crate::W<CNT_DIFF_SPEC>;
#[doc = "Field `TX_CNT_DIFF` reader - tx bck counter value."]
pub type TX_CNT_DIFF_R = crate::FieldReader<u32>;
#[doc = "Field `TX_CNT_DIFF_RST` writer - Set this bit to reset fifo counter difference."]
pub type TX_CNT_DIFF_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - tx bck counter value."]
    #[inline(always)]
    pub fn tx_cnt_diff(&self) -> TX_CNT_DIFF_R {
        TX_CNT_DIFF_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNT_DIFF")
            .field("tx_cnt_diff", &self.tx_cnt_diff())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - Set this bit to reset fifo counter difference."]
    #[inline(always)]
    pub fn tx_cnt_diff_rst(&mut self) -> TX_CNT_DIFF_RST_W<'_, CNT_DIFF_SPEC> {
        TX_CNT_DIFF_RST_W::new(self, 31)
    }
}
#[doc = "I2S sync counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt_diff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt_diff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNT_DIFF_SPEC;
impl crate::RegisterSpec for CNT_DIFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt_diff::R`](R) reader structure"]
impl crate::Readable for CNT_DIFF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cnt_diff::W`](W) writer structure"]
impl crate::Writable for CNT_DIFF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNT_DIFF to value 0"]
impl crate::Resettable for CNT_DIFF_SPEC {}
