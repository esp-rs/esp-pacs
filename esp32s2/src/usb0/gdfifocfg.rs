///Register `GDFIFOCFG` reader
pub type R = crate::R<GDFIFOCFG_SPEC>;
///Register `GDFIFOCFG` writer
pub type W = crate::W<GDFIFOCFG_SPEC>;
///Field `GDFIFOCFG` reader -
pub type GDFIFOCFG_R = crate::FieldReader<u16>;
///Field `GDFIFOCFG` writer -
pub type GDFIFOCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `EPINFOBASEADDR` reader -
pub type EPINFOBASEADDR_R = crate::FieldReader<u16>;
///Field `EPINFOBASEADDR` writer -
pub type EPINFOBASEADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15
    #[inline(always)]
    pub fn gdfifocfg(&self) -> GDFIFOCFG_R {
        GDFIFOCFG_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn epinfobaseaddr(&self) -> EPINFOBASEADDR_R {
        EPINFOBASEADDR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GDFIFOCFG")
            .field("gdfifocfg", &self.gdfifocfg())
            .field("epinfobaseaddr", &self.epinfobaseaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:15
    #[inline(always)]
    #[must_use]
    pub fn gdfifocfg(&mut self) -> GDFIFOCFG_W<GDFIFOCFG_SPEC> {
        GDFIFOCFG_W::new(self, 0)
    }
    ///Bits 16:31
    #[inline(always)]
    #[must_use]
    pub fn epinfobaseaddr(&mut self) -> EPINFOBASEADDR_W<GDFIFOCFG_SPEC> {
        EPINFOBASEADDR_W::new(self, 16)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`gdfifocfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdfifocfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GDFIFOCFG_SPEC;
impl crate::RegisterSpec for GDFIFOCFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gdfifocfg::R`](R) reader structure
impl crate::Readable for GDFIFOCFG_SPEC {}
///`write(|w| ..)` method takes [`gdfifocfg::W`](W) writer structure
impl crate::Writable for GDFIFOCFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GDFIFOCFG to value 0
impl crate::Resettable for GDFIFOCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
