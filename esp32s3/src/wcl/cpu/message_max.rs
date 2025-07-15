#[doc = "Register `MESSAGE_MAX` reader"]
pub type R = crate::R<MESSAGE_MAX_SPEC>;
#[doc = "Register `MESSAGE_MAX` writer"]
pub type W = crate::W<MESSAGE_MAX_SPEC>;
#[doc = "Field `MESSAGE_MAX` reader - This filed is used to set the max value of clear write_buffer"]
pub type MESSAGE_MAX_R = crate::FieldReader;
#[doc = "Field `MESSAGE_MAX` writer - This filed is used to set the max value of clear write_buffer"]
pub type MESSAGE_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - This filed is used to set the max value of clear write_buffer"]
    #[inline(always)]
    pub fn message_max(&self) -> MESSAGE_MAX_R {
        MESSAGE_MAX_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MESSAGE_MAX")
            .field("message_max", &self.message_max())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - This filed is used to set the max value of clear write_buffer"]
    #[inline(always)]
    pub fn message_max(&mut self) -> MESSAGE_MAX_W<MESSAGE_MAX_SPEC> {
        MESSAGE_MAX_W::new(self, 0)
    }
}
#[doc = "Clear writer_buffer write number configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`message_max::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`message_max::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MESSAGE_MAX_SPEC;
impl crate::RegisterSpec for MESSAGE_MAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`message_max::R`](R) reader structure"]
impl crate::Readable for MESSAGE_MAX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`message_max::W`](W) writer structure"]
impl crate::Writable for MESSAGE_MAX_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MESSAGE_MAX to value 0"]
impl crate::Resettable for MESSAGE_MAX_SPEC {}
