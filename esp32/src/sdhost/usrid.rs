#[doc = "Register `USRID` reader"]
pub type R = crate::R<USRID_SPEC>;
#[doc = "Register `USRID` writer"]
pub type W = crate::W<USRID_SPEC>;
#[doc = "Field `USRID` reader - User identification register, value set by user. Can also be used as a scratchpad register by user."]
pub type USRID_R = crate::FieldReader<u32>;
#[doc = "Field `USRID` writer - User identification register, value set by user. Can also be used as a scratchpad register by user."]
pub type USRID_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User identification register, value set by user. Can also be used as a scratchpad register by user."]
    #[inline(always)]
    pub fn usrid(&self) -> USRID_R {
        USRID_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USRID")
            .field("usrid", &self.usrid())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - User identification register, value set by user. Can also be used as a scratchpad register by user."]
    #[inline(always)]
    pub fn usrid(&mut self) -> USRID_W<USRID_SPEC> {
        USRID_W::new(self, 0)
    }
}
#[doc = "User ID (scratchpad) register\n\nYou can [`read`](crate::Reg::read) this register and get [`usrid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usrid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USRID_SPEC;
impl crate::RegisterSpec for USRID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usrid::R`](R) reader structure"]
impl crate::Readable for USRID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usrid::W`](W) writer structure"]
impl crate::Writable for USRID_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USRID to value 0"]
impl crate::Resettable for USRID_SPEC {}
