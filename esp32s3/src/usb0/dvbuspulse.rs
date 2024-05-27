///Register `DVBUSPULSE` reader
pub type R = crate::R<DVBUSPULSE_SPEC>;
///Register `DVBUSPULSE` writer
pub type W = crate::W<DVBUSPULSE_SPEC>;
///Field `DVBUSPULSE` reader -
pub type DVBUSPULSE_R = crate::FieldReader<u16>;
///Field `DVBUSPULSE` writer -
pub type DVBUSPULSE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11
    #[inline(always)]
    pub fn dvbuspulse(&self) -> DVBUSPULSE_R {
        DVBUSPULSE_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DVBUSPULSE")
            .field("dvbuspulse", &self.dvbuspulse())
            .finish()
    }
}
impl W {
    ///Bits 0:11
    #[inline(always)]
    #[must_use]
    pub fn dvbuspulse(&mut self) -> DVBUSPULSE_W<DVBUSPULSE_SPEC> {
        DVBUSPULSE_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`dvbuspulse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbuspulse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DVBUSPULSE_SPEC;
impl crate::RegisterSpec for DVBUSPULSE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dvbuspulse::R`](R) reader structure
impl crate::Readable for DVBUSPULSE_SPEC {}
///`write(|w| ..)` method takes [`dvbuspulse::W`](W) writer structure
impl crate::Writable for DVBUSPULSE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DVBUSPULSE to value 0x05b8
impl crate::Resettable for DVBUSPULSE_SPEC {
    const RESET_VALUE: u32 = 0x05b8;
}
