#[doc = "Register `SYNC_SW_THRES` reader"]
pub type R = crate::R<SYNC_SW_THRES_SPEC>;
#[doc = "Register `SYNC_SW_THRES` writer"]
pub type W = crate::W<SYNC_SW_THRES_SPEC>;
#[doc = "Field `TX_CNT_DIFF_SW_THRES` reader - tx fifo counter difference software threshold value, when difference larger than this threshold, interrupt will occur and hardware sync will not be executed."]
pub type TX_CNT_DIFF_SW_THRES_R = crate::FieldReader<u32>;
#[doc = "Field `TX_CNT_DIFF_SW_THRES` writer - tx fifo counter difference software threshold value, when difference larger than this threshold, interrupt will occur and hardware sync will not be executed."]
pub type TX_CNT_DIFF_SW_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - tx fifo counter difference software threshold value, when difference larger than this threshold, interrupt will occur and hardware sync will not be executed."]
    #[inline(always)]
    pub fn tx_cnt_diff_sw_thres(&self) -> TX_CNT_DIFF_SW_THRES_R {
        TX_CNT_DIFF_SW_THRES_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNC_SW_THRES")
            .field("tx_cnt_diff_sw_thres", &self.tx_cnt_diff_sw_thres())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:30 - tx fifo counter difference software threshold value, when difference larger than this threshold, interrupt will occur and hardware sync will not be executed."]
    #[inline(always)]
    pub fn tx_cnt_diff_sw_thres(&mut self) -> TX_CNT_DIFF_SW_THRES_W<'_, SYNC_SW_THRES_SPEC> {
        TX_CNT_DIFF_SW_THRES_W::new(self, 0)
    }
}
#[doc = "I2S sync counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_sw_thres::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_sw_thres::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNC_SW_THRES_SPEC;
impl crate::RegisterSpec for SYNC_SW_THRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync_sw_thres::R`](R) reader structure"]
impl crate::Readable for SYNC_SW_THRES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sync_sw_thres::W`](W) writer structure"]
impl crate::Writable for SYNC_SW_THRES_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYNC_SW_THRES to value 0"]
impl crate::Resettable for SYNC_SW_THRES_SPEC {}
