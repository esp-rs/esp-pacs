///Register `ED_ABORT_CNT` reader
pub type R = crate::R<ED_ABORT_CNT_SPEC>;
///Register `ED_ABORT_CNT` writer
pub type W = crate::W<ED_ABORT_CNT_SPEC>;
///Field `ED_ABORT_CNT` reader -
pub type ED_ABORT_CNT_R = crate::FieldReader<u16>;
///Field `ED_ABORT_CNT` writer -
pub type ED_ABORT_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15
    #[inline(always)]
    pub fn ed_abort_cnt(&self) -> ED_ABORT_CNT_R {
        ED_ABORT_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ED_ABORT_CNT")
            .field("ed_abort_cnt", &self.ed_abort_cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:15
    #[inline(always)]
    #[must_use]
    pub fn ed_abort_cnt(&mut self) -> ED_ABORT_CNT_W<ED_ABORT_CNT_SPEC> {
        ED_ABORT_CNT_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`ed_abort_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ed_abort_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ED_ABORT_CNT_SPEC;
impl crate::RegisterSpec for ED_ABORT_CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ed_abort_cnt::R`](R) reader structure
impl crate::Readable for ED_ABORT_CNT_SPEC {}
///`write(|w| ..)` method takes [`ed_abort_cnt::W`](W) writer structure
impl crate::Writable for ED_ABORT_CNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ED_ABORT_CNT to value 0
impl crate::Resettable for ED_ABORT_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
