///Register `DOEPTSIZ` reader
pub type R = crate::R<DOEPTSIZ_SPEC>;
///Register `DOEPTSIZ` writer
pub type W = crate::W<DOEPTSIZ_SPEC>;
///Field `XFERSIZE` reader -
pub type XFERSIZE_R = crate::FieldReader;
///Field `XFERSIZE` writer -
pub type XFERSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
///Field `PKTCNT` reader -
pub type PKTCNT_R = crate::BitReader;
///Field `PKTCNT` writer -
pub type PKTCNT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUPCNT` reader -
pub type SUPCNT_R = crate::FieldReader;
///Field `SUPCNT` writer -
pub type SUPCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:6
    #[inline(always)]
    pub fn xfersize(&self) -> XFERSIZE_R {
        XFERSIZE_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 19
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 29:30
    #[inline(always)]
    pub fn supcnt(&self) -> SUPCNT_R {
        SUPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("supcnt", &self.supcnt())
            .finish()
    }
}
impl W {
    ///Bits 0:6
    #[inline(always)]
    #[must_use]
    pub fn xfersize(&mut self) -> XFERSIZE_W<DOEPTSIZ_SPEC> {
        XFERSIZE_W::new(self, 0)
    }
    ///Bit 19
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<DOEPTSIZ_SPEC> {
        PKTCNT_W::new(self, 19)
    }
    ///Bits 29:30
    #[inline(always)]
    #[must_use]
    pub fn supcnt(&mut self) -> SUPCNT_W<DOEPTSIZ_SPEC> {
        SUPCNT_W::new(self, 29)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DOEPTSIZ_SPEC;
impl crate::RegisterSpec for DOEPTSIZ_SPEC {
    type Ux = u32;
}
///`read()` method returns [`doeptsiz::R`](R) reader structure
impl crate::Readable for DOEPTSIZ_SPEC {}
///`write(|w| ..)` method takes [`doeptsiz::W`](W) writer structure
impl crate::Writable for DOEPTSIZ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DOEPTSIZ to value 0
impl crate::Resettable for DOEPTSIZ_SPEC {
    const RESET_VALUE: u32 = 0;
}
