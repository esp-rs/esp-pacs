#[doc = "Register `DIEPTXF3` reader"]
pub type R = crate::R<DIEPTXF3_SPEC>;
#[doc = "Register `DIEPTXF3` writer"]
pub type W = crate::W<DIEPTXF3_SPEC>;
#[doc = "Field `INEP3TXFSTADDR` reader - "]
pub type INEP3TXFSTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `INEP3TXFSTADDR` writer - "]
pub type INEP3TXFSTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INEP3TXFDEP` reader - "]
pub type INEP3TXFDEP_R = crate::FieldReader<u16>;
#[doc = "Field `INEP3TXFDEP` writer - "]
pub type INEP3TXFDEP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn inep3txfstaddr(&self) -> INEP3TXFSTADDR_R {
        INEP3TXFSTADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn inep3txfdep(&self) -> INEP3TXFDEP_R {
        INEP3TXFDEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTXF3")
            .field(
                "inep3txfstaddr",
                &format_args!("{}", self.inep3txfstaddr().bits()),
            )
            .field(
                "inep3txfdep",
                &format_args!("{}", self.inep3txfdep().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPTXF3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn inep3txfstaddr(&mut self) -> INEP3TXFSTADDR_W<DIEPTXF3_SPEC> {
        INEP3TXFSTADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn inep3txfdep(&mut self) -> INEP3TXFDEP_W<DIEPTXF3_SPEC> {
        INEP3TXFDEP_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTXF3_SPEC;
impl crate::RegisterSpec for DIEPTXF3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf3::R`](R) reader structure"]
impl crate::Readable for DIEPTXF3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptxf3::W`](W) writer structure"]
impl crate::Writable for DIEPTXF3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTXF3 to value 0x1000_0200"]
impl crate::Resettable for DIEPTXF3_SPEC {
    const RESET_VALUE: u32 = 0x1000_0200;
}
