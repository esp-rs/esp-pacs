#[doc = "Register `INT_IE%s` reader"]
pub type R = crate::R<INT_IE_SPEC>;
#[doc = "Register `INT_IE%s` writer"]
pub type W = crate::W<INT_IE_SPEC>;
#[doc = "Field `INT_IE` reader - Interrupt enable bit."]
pub type INT_IE_R = crate::BitReader;
#[doc = "Field `INT_IE` writer - Interrupt enable bit."]
pub type INT_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt enable bit."]
    #[inline(always)]
    pub fn int_ie(&self) -> INT_IE_R {
        INT_IE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_IE")
            .field("int_ie", &self.int_ie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable bit."]
    #[inline(always)]
    pub fn int_ie(&mut self) -> INT_IE_W<'_, INT_IE_SPEC> {
        INT_IE_W::new(self, 0)
    }
}
#[doc = "Interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_IE_SPEC;
impl crate::RegisterSpec for INT_IE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`int_ie::R`](R) reader structure"]
impl crate::Readable for INT_IE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ie::W`](W) writer structure"]
impl crate::Writable for INT_IE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_IE%s to value 0"]
impl crate::Resettable for INT_IE_SPEC {}
