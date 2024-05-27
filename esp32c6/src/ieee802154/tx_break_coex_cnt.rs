#[doc = "Register `TX_BREAK_COEX_CNT` reader"]
pub type R = crate::R<TX_BREAK_COEX_CNT_SPEC>;
#[doc = "Register `TX_BREAK_COEX_CNT` writer"]
pub type W = crate::W<TX_BREAK_COEX_CNT_SPEC>;
#[doc = "Field `TX_BREAK_COEX_CNT` reader - "]
pub type TX_BREAK_COEX_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `TX_BREAK_COEX_CNT` writer - "]
pub type TX_BREAK_COEX_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tx_break_coex_cnt(&self) -> TX_BREAK_COEX_CNT_R {
        TX_BREAK_COEX_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_BREAK_COEX_CNT")
            .field("tx_break_coex_cnt", &self.tx_break_coex_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn tx_break_coex_cnt(&mut self) -> TX_BREAK_COEX_CNT_W<TX_BREAK_COEX_CNT_SPEC> {
        TX_BREAK_COEX_CNT_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_break_coex_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_break_coex_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_BREAK_COEX_CNT_SPEC;
impl crate::RegisterSpec for TX_BREAK_COEX_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_break_coex_cnt::R`](R) reader structure"]
impl crate::Readable for TX_BREAK_COEX_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_break_coex_cnt::W`](W) writer structure"]
impl crate::Writable for TX_BREAK_COEX_CNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_BREAK_COEX_CNT to value 0"]
impl crate::Resettable for TX_BREAK_COEX_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
