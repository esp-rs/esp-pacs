#[doc = "Register `VERSION` reader"]
pub type R = crate::R<VERSION_SPEC>;
#[doc = "Register `VERSION` writer"]
pub type W = crate::W<VERSION_SPEC>;
#[doc = "Field `BITSCRAMBLER_VER` reader - Reserved"]
pub type BITSCRAMBLER_VER_R = crate::FieldReader<u32>;
#[doc = "Field `BITSCRAMBLER_VER` writer - Reserved"]
pub type BITSCRAMBLER_VER_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Reserved"]
    #[inline(always)]
    pub fn bitscrambler_ver(&self) -> BITSCRAMBLER_VER_R {
        BITSCRAMBLER_VER_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VERSION")
            .field("bitscrambler_ver", &self.bitscrambler_ver())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - Reserved"]
    #[inline(always)]
    pub fn bitscrambler_ver(&mut self) -> BITSCRAMBLER_VER_W<VERSION_SPEC> {
        BITSCRAMBLER_VER_W::new(self, 0)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`version::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`version::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VERSION_SPEC;
impl crate::RegisterSpec for VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`version::R`](R) reader structure"]
impl crate::Readable for VERSION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`version::W`](W) writer structure"]
impl crate::Writable for VERSION_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VERSION to value 0x0230_3240"]
impl crate::Resettable for VERSION_SPEC {
    const RESET_VALUE: u32 = 0x0230_3240;
}
