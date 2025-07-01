#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `COCPU_TSENS_WAKE` reader - Tsens wakeup interrupt enable."]
pub type COCPU_TSENS_WAKE_R = crate::BitReader;
#[doc = "Field `COCPU_TSENS_WAKE` writer - Tsens wakeup interrupt enable."]
pub type COCPU_TSENS_WAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tsens wakeup interrupt enable."]
    #[inline(always)]
    pub fn cocpu_tsens_wake(&self) -> COCPU_TSENS_WAKE_R {
        COCPU_TSENS_WAKE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("cocpu_tsens_wake", &self.cocpu_tsens_wake())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Tsens wakeup interrupt enable."]
    #[inline(always)]
    pub fn cocpu_tsens_wake(&mut self) -> COCPU_TSENS_WAKE_W<INT_ENA_SPEC> {
        COCPU_TSENS_WAKE_W::new(self, 0)
    }
}
#[doc = "Tsens interrupt enable registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {}
