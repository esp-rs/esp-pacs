///Register `DIEPTSIZ` reader
pub type R = crate::R<DIEPTSIZ_SPEC>;
///Register `DIEPTSIZ` writer
pub type W = crate::W<DIEPTSIZ_SPEC>;
///Field `XFERSIZE` reader -
pub type XFERSIZE_R = crate::FieldReader<u32>;
///Field `XFERSIZE` writer -
pub type XFERSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32, crate::Safe>;
///Field `PKTCNT` reader -
pub type PKTCNT_R = crate::FieldReader<u16>;
///Field `PKTCNT` writer -
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
impl R {
    ///Bits 0:18
    #[inline(always)]
    pub fn xfersize(&self) -> XFERSIZE_R {
        XFERSIZE_R::new(self.bits & 0x0007_ffff)
    }
    ///Bits 19:28
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTSIZ")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .finish()
    }
}
impl W {
    ///Bits 0:18
    #[inline(always)]
    #[must_use]
    pub fn xfersize(&mut self) -> XFERSIZE_W<DIEPTSIZ_SPEC> {
        XFERSIZE_W::new(self, 0)
    }
    ///Bits 19:28
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<DIEPTSIZ_SPEC> {
        PKTCNT_W::new(self, 19)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DIEPTSIZ_SPEC;
impl crate::RegisterSpec for DIEPTSIZ_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dieptsiz::R`](R) reader structure
impl crate::Readable for DIEPTSIZ_SPEC {}
///`write(|w| ..)` method takes [`dieptsiz::W`](W) writer structure
impl crate::Writable for DIEPTSIZ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DIEPTSIZ to value 0
impl crate::Resettable for DIEPTSIZ_SPEC {
    const RESET_VALUE: u32 = 0;
}
