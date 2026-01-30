#[doc = "Register `INT_CTL%s` reader"]
pub type R = crate::R<INT_CTL_SPEC>;
#[doc = "Register `INT_CTL%s` writer"]
pub type W = crate::W<INT_CTL_SPEC>;
#[doc = "Field `CTL` reader - Configures the level and priority of the ith CLIC machine mode interrupt."]
pub type CTL_R = crate::FieldReader;
#[doc = "Field `CTL` writer - Configures the level and priority of the ith CLIC machine mode interrupt."]
pub type CTL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 5:7 - Configures the level and priority of the ith CLIC machine mode interrupt."]
    #[inline(always)]
    pub fn ctl(&self) -> CTL_R {
        CTL_R::new((self.bits >> 5) & 7)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_CTL").field("ctl", &self.ctl()).finish()
    }
}
impl W {
    #[doc = "Bits 5:7 - Configures the level and priority of the ith CLIC machine mode interrupt."]
    #[inline(always)]
    pub fn ctl(&mut self) -> CTL_W<'_, INT_CTL_SPEC> {
        CTL_W::new(self, 5)
    }
}
#[doc = "Interrupt level control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CTL_SPEC;
impl crate::RegisterSpec for INT_CTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`int_ctl::R`](R) reader structure"]
impl crate::Readable for INT_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ctl::W`](W) writer structure"]
impl crate::Writable for INT_CTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_CTL%s to value 0"]
impl crate::Resettable for INT_CTL_SPEC {}
