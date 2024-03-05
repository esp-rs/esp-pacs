#[doc = "Register `DIEPTXF4` reader"]
pub type R = crate::R<DIEPTXF4_SPEC>;
#[doc = "Register `DIEPTXF4` writer"]
pub type W = crate::W<DIEPTXF4_SPEC>;
#[doc = "Field `INEP4TXFSTADDR` reader - "]
pub type INEP4TXFSTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `INEP4TXFSTADDR` writer - "]
pub type INEP4TXFSTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INEP4TXFDEP` reader - "]
pub type INEP4TXFDEP_R = crate::FieldReader<u16>;
#[doc = "Field `INEP4TXFDEP` writer - "]
pub type INEP4TXFDEP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn inep4txfstaddr(&self) -> INEP4TXFSTADDR_R {
        INEP4TXFSTADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn inep4txfdep(&self) -> INEP4TXFDEP_R {
        INEP4TXFDEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTXF4")
            .field(
                "inep4txfstaddr",
                &format_args!("{}", self.inep4txfstaddr().bits()),
            )
            .field(
                "inep4txfdep",
                &format_args!("{}", self.inep4txfdep().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPTXF4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn inep4txfstaddr(&mut self) -> INEP4TXFSTADDR_W<DIEPTXF4_SPEC> {
        INEP4TXFSTADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn inep4txfdep(&mut self) -> INEP4TXFDEP_W<DIEPTXF4_SPEC> {
        INEP4TXFDEP_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTXF4_SPEC;
impl crate::RegisterSpec for DIEPTXF4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf4::R`](R) reader structure"]
impl crate::Readable for DIEPTXF4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptxf4::W`](W) writer structure"]
impl crate::Writable for DIEPTXF4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTXF4 to value 0x1000_0200"]
impl crate::Resettable for DIEPTXF4_SPEC {
    const RESET_VALUE: u32 = 0x1000_0200;
}
