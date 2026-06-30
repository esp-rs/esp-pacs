#[doc = "Register `FILTER_CNT` reader"]
pub type R = crate::R<FILTER_CNT_SPEC>;
#[doc = "Register `FILTER_CNT` writer"]
pub type W = crate::W<FILTER_CNT_SPEC>;
#[doc = "Field `FILTER_CNT` reader - protect time after det the frist zero det int,it must be greater than or equal to 1"]
pub type FILTER_CNT_R = crate::FieldReader<u32>;
#[doc = "Field `FILTER_CNT` writer - protect time after det the frist zero det int,it must be greater than or equal to 1"]
pub type FILTER_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - protect time after det the frist zero det int,it must be greater than or equal to 1"]
    #[inline(always)]
    pub fn filter_cnt(&self) -> FILTER_CNT_R {
        FILTER_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER_CNT")
            .field("filter_cnt", &self.filter_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - protect time after det the frist zero det int,it must be greater than or equal to 1"]
    #[inline(always)]
    pub fn filter_cnt(&mut self) -> FILTER_CNT_W<'_, FILTER_CNT_SPEC> {
        FILTER_CNT_W::new(self, 0)
    }
}
#[doc = "protect time reg\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTER_CNT_SPEC;
impl crate::RegisterSpec for FILTER_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter_cnt::R`](R) reader structure"]
impl crate::Readable for FILTER_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filter_cnt::W`](W) writer structure"]
impl crate::Writable for FILTER_CNT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FILTER_CNT to value 0xff"]
impl crate::Resettable for FILTER_CNT_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
