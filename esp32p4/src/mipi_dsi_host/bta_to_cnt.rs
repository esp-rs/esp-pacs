#[doc = "Register `BTA_TO_CNT` reader"]
pub type R = crate::R<BTA_TO_CNT_SPEC>;
#[doc = "Register `BTA_TO_CNT` writer"]
pub type W = crate::W<BTA_TO_CNT_SPEC>;
#[doc = "Field `BTA_TO_CNT` reader - NA"]
pub type BTA_TO_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `BTA_TO_CNT` writer - NA"]
pub type BTA_TO_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn bta_to_cnt(&self) -> BTA_TO_CNT_R {
        BTA_TO_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BTA_TO_CNT")
            .field("bta_to_cnt", &self.bta_to_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn bta_to_cnt(&mut self) -> BTA_TO_CNT_W<'_, BTA_TO_CNT_SPEC> {
        BTA_TO_CNT_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`bta_to_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bta_to_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTA_TO_CNT_SPEC;
impl crate::RegisterSpec for BTA_TO_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bta_to_cnt::R`](R) reader structure"]
impl crate::Readable for BTA_TO_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bta_to_cnt::W`](W) writer structure"]
impl crate::Writable for BTA_TO_CNT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BTA_TO_CNT to value 0"]
impl crate::Resettable for BTA_TO_CNT_SPEC {}
