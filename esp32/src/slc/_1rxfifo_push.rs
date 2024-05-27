///Register `_1RXFIFO_PUSH` reader
pub type R = crate::R<_1RXFIFO_PUSH_SPEC>;
///Register `_1RXFIFO_PUSH` writer
pub type W = crate::W<_1RXFIFO_PUSH_SPEC>;
///Field `SLC1_RXFIFO_WDATA` reader -
pub type SLC1_RXFIFO_WDATA_R = crate::FieldReader<u16>;
///Field `SLC1_RXFIFO_WDATA` writer -
pub type SLC1_RXFIFO_WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `SLC1_RXFIFO_PUSH` reader -
pub type SLC1_RXFIFO_PUSH_R = crate::BitReader;
///Field `SLC1_RXFIFO_PUSH` writer -
pub type SLC1_RXFIFO_PUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:8
    #[inline(always)]
    pub fn slc1_rxfifo_wdata(&self) -> SLC1_RXFIFO_WDATA_R {
        SLC1_RXFIFO_WDATA_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 16
    #[inline(always)]
    pub fn slc1_rxfifo_push(&self) -> SLC1_RXFIFO_PUSH_R {
        SLC1_RXFIFO_PUSH_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_1RXFIFO_PUSH")
            .field("slc1_rxfifo_wdata", &self.slc1_rxfifo_wdata())
            .field("slc1_rxfifo_push", &self.slc1_rxfifo_push())
            .finish()
    }
}
impl W {
    ///Bits 0:8
    #[inline(always)]
    #[must_use]
    pub fn slc1_rxfifo_wdata(&mut self) -> SLC1_RXFIFO_WDATA_W<_1RXFIFO_PUSH_SPEC> {
        SLC1_RXFIFO_WDATA_W::new(self, 0)
    }
    ///Bit 16
    #[inline(always)]
    #[must_use]
    pub fn slc1_rxfifo_push(&mut self) -> SLC1_RXFIFO_PUSH_W<_1RXFIFO_PUSH_SPEC> {
        SLC1_RXFIFO_PUSH_W::new(self, 16)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`_1rxfifo_push::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1rxfifo_push::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct _1RXFIFO_PUSH_SPEC;
impl crate::RegisterSpec for _1RXFIFO_PUSH_SPEC {
    type Ux = u32;
}
///`read()` method returns [`_1rxfifo_push::R`](R) reader structure
impl crate::Readable for _1RXFIFO_PUSH_SPEC {}
///`write(|w| ..)` method takes [`_1rxfifo_push::W`](W) writer structure
impl crate::Writable for _1RXFIFO_PUSH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets _1RXFIFO_PUSH to value 0
impl crate::Resettable for _1RXFIFO_PUSH_SPEC {
    const RESET_VALUE: u32 = 0;
}
