///Register `BTA_TO_CNT` reader
pub type R = crate::R<BTA_TO_CNT_SPEC>;
///Register `BTA_TO_CNT` writer
pub type W = crate::W<BTA_TO_CNT_SPEC>;
///Field `BTA_TO_CNT` reader - NA
pub type BTA_TO_CNT_R = crate::FieldReader<u16>;
///Field `BTA_TO_CNT` writer - NA
pub type BTA_TO_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - NA
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
    ///Bits 0:15 - NA
    #[inline(always)]
    #[must_use]
    pub fn bta_to_cnt(&mut self) -> BTA_TO_CNT_W<BTA_TO_CNT_SPEC> {
        BTA_TO_CNT_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`bta_to_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bta_to_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BTA_TO_CNT_SPEC;
impl crate::RegisterSpec for BTA_TO_CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`bta_to_cnt::R`](R) reader structure
impl crate::Readable for BTA_TO_CNT_SPEC {}
///`write(|w| ..)` method takes [`bta_to_cnt::W`](W) writer structure
impl crate::Writable for BTA_TO_CNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BTA_TO_CNT to value 0
impl crate::Resettable for BTA_TO_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
