#[doc = "Register `DIEPTXF%s` reader"]
pub type R = crate::R<DIEPTXF_SPEC>;
#[doc = "Register `DIEPTXF%s` writer"]
pub type W = crate::W<DIEPTXF_SPEC>;
#[doc = "Field `INEP1TXFSTADDR` reader - "]
pub type INEP1TXFSTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `INEP1TXFSTADDR` writer - "]
pub type INEP1TXFSTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INEP1TXFDEP` reader - "]
pub type INEP1TXFDEP_R = crate::FieldReader<u16>;
#[doc = "Field `INEP1TXFDEP` writer - "]
pub type INEP1TXFDEP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn inep1txfstaddr(&self) -> INEP1TXFSTADDR_R {
        INEP1TXFSTADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn inep1txfdep(&self) -> INEP1TXFDEP_R {
        INEP1TXFDEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTXF")
            .field("inep1txfstaddr", &self.inep1txfstaddr())
            .field("inep1txfdep", &self.inep1txfdep())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn inep1txfstaddr(&mut self) -> INEP1TXFSTADDR_W<'_, DIEPTXF_SPEC> {
        INEP1TXFSTADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn inep1txfdep(&mut self) -> INEP1TXFDEP_W<'_, DIEPTXF_SPEC> {
        INEP1TXFDEP_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTXF_SPEC;
impl crate::RegisterSpec for DIEPTXF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf::R`](R) reader structure"]
impl crate::Readable for DIEPTXF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptxf::W`](W) writer structure"]
impl crate::Writable for DIEPTXF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPTXF%s to value 0x1000_0200"]
impl crate::Resettable for DIEPTXF_SPEC {
    const RESET_VALUE: u32 = 0x1000_0200;
}
