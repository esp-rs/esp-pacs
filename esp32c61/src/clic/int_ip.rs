#[doc = "Register `INT_IP%s` reader"]
pub type R = crate::R<INT_IP_SPEC>;
#[doc = "Register `INT_IP%s` writer"]
pub type W = crate::W<INT_IP_SPEC>;
#[doc = "Field `INT_IP` reader - Interrupt pending bit."]
pub type INT_IP_R = crate::BitReader;
#[doc = "Field `INT_IP` writer - Interrupt pending bit."]
pub type INT_IP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt pending bit."]
    #[inline(always)]
    pub fn int_ip(&self) -> INT_IP_R {
        INT_IP_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_IP")
            .field("int_ip", &self.int_ip())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt pending bit."]
    #[inline(always)]
    pub fn int_ip(&mut self) -> INT_IP_W<'_, INT_IP_SPEC> {
        INT_IP_W::new(self, 0)
    }
}
#[doc = "Interrupt pending register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_IP_SPEC;
impl crate::RegisterSpec for INT_IP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`int_ip::R`](R) reader structure"]
impl crate::Readable for INT_IP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ip::W`](W) writer structure"]
impl crate::Writable for INT_IP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_IP%s to value 0"]
impl crate::Resettable for INT_IP_SPEC {}
