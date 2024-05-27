///Register `GNPTXFSIZ` reader
pub type R = crate::R<GNPTXFSIZ_SPEC>;
///Register `GNPTXFSIZ` writer
pub type W = crate::W<GNPTXFSIZ_SPEC>;
///Field `NPTXFSTADDR` reader -
pub type NPTXFSTADDR_R = crate::FieldReader<u16>;
///Field `NPTXFSTADDR` writer -
pub type NPTXFSTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `NPTXFDEP` reader -
pub type NPTXFDEP_R = crate::FieldReader<u16>;
///Field `NPTXFDEP` writer -
pub type NPTXFDEP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15
    #[inline(always)]
    pub fn nptxfstaddr(&self) -> NPTXFSTADDR_R {
        NPTXFSTADDR_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn nptxfdep(&self) -> NPTXFDEP_R {
        NPTXFDEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GNPTXFSIZ")
            .field("nptxfstaddr", &self.nptxfstaddr())
            .field("nptxfdep", &self.nptxfdep())
            .finish()
    }
}
impl W {
    ///Bits 0:15
    #[inline(always)]
    #[must_use]
    pub fn nptxfstaddr(&mut self) -> NPTXFSTADDR_W<GNPTXFSIZ_SPEC> {
        NPTXFSTADDR_W::new(self, 0)
    }
    ///Bits 16:31
    #[inline(always)]
    #[must_use]
    pub fn nptxfdep(&mut self) -> NPTXFDEP_W<GNPTXFSIZ_SPEC> {
        NPTXFDEP_W::new(self, 16)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GNPTXFSIZ_SPEC;
impl crate::RegisterSpec for GNPTXFSIZ_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gnptxfsiz::R`](R) reader structure
impl crate::Readable for GNPTXFSIZ_SPEC {}
///`write(|w| ..)` method takes [`gnptxfsiz::W`](W) writer structure
impl crate::Writable for GNPTXFSIZ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GNPTXFSIZ to value 0x0100_0100
impl crate::Resettable for GNPTXFSIZ_SPEC {
    const RESET_VALUE: u32 = 0x0100_0100;
}
