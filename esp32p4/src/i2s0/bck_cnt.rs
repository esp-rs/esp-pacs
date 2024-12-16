#[doc = "Register `BCK_CNT` reader"]
pub type R = crate::R<BCK_CNT_SPEC>;
#[doc = "Register `BCK_CNT` writer"]
pub type W = crate::W<BCK_CNT_SPEC>;
#[doc = "Field `TX_BCK_CNT` reader - tx bck counter value."]
pub type TX_BCK_CNT_R = crate::FieldReader<u32>;
#[doc = "Field `TX_BCK_CNT_RST` writer - Set this bit to reset tx bck counter."]
pub type TX_BCK_CNT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - tx bck counter value."]
    #[inline(always)]
    pub fn tx_bck_cnt(&self) -> TX_BCK_CNT_R {
        TX_BCK_CNT_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCK_CNT")
            .field("tx_bck_cnt", &self.tx_bck_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - Set this bit to reset tx bck counter."]
    #[inline(always)]
    pub fn tx_bck_cnt_rst(&mut self) -> TX_BCK_CNT_RST_W<BCK_CNT_SPEC> {
        TX_BCK_CNT_RST_W::new(self, 31)
    }
}
#[doc = "I2S sync counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`bck_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bck_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCK_CNT_SPEC;
impl crate::RegisterSpec for BCK_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bck_cnt::R`](R) reader structure"]
impl crate::Readable for BCK_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bck_cnt::W`](W) writer structure"]
impl crate::Writable for BCK_CNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCK_CNT to value 0"]
impl crate::Resettable for BCK_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
