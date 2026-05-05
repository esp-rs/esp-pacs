#[doc = "Register `MESSAGE_%s` reader"]
pub type R = crate::R<MESSAGE__SPEC>;
#[doc = "Register `MESSAGE_%s` writer"]
pub type W = crate::W<MESSAGE__SPEC>;
#[doc = "Field `MESSAGE_0` reader - MB_MASSEGE_0"]
pub type MESSAGE_0_R = crate::FieldReader<u32>;
#[doc = "Field `MESSAGE_0` writer - MB_MASSEGE_0"]
pub type MESSAGE_0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MB_MASSEGE_0"]
    #[inline(always)]
    pub fn message_0(&self) -> MESSAGE_0_R {
        MESSAGE_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MESSAGE_")
            .field("message_0", &self.message_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - MB_MASSEGE_0"]
    #[inline(always)]
    pub fn message_0(&mut self) -> MESSAGE_0_W<'_, MESSAGE__SPEC> {
        MESSAGE_0_W::new(self, 0)
    }
}
#[doc = "MB_MASSEGE_0\n\nYou can [`read`](crate::Reg::read) this register and get [`message_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`message_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MESSAGE__SPEC;
impl crate::RegisterSpec for MESSAGE__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`message_::R`](R) reader structure"]
impl crate::Readable for MESSAGE__SPEC {}
#[doc = "`write(|w| ..)` method takes [`message_::W`](W) writer structure"]
impl crate::Writable for MESSAGE__SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MESSAGE_%s to value 0"]
impl crate::Resettable for MESSAGE__SPEC {}
