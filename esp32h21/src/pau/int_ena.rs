#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `DONE_INT_ENA` reader - backup done flag"]
pub type DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `DONE_INT_ENA` writer - backup done flag"]
pub type DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR_INT_ENA` reader - error flag"]
pub type ERROR_INT_ENA_R = crate::BitReader;
#[doc = "Field `ERROR_INT_ENA` writer - error flag"]
pub type ERROR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - backup done flag"]
    #[inline(always)]
    pub fn done_int_ena(&self) -> DONE_INT_ENA_R {
        DONE_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - error flag"]
    #[inline(always)]
    pub fn error_int_ena(&self) -> ERROR_INT_ENA_R {
        ERROR_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("done_int_ena", &self.done_int_ena())
            .field("error_int_ena", &self.error_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - backup done flag"]
    #[inline(always)]
    pub fn done_int_ena(&mut self) -> DONE_INT_ENA_W<'_, INT_ENA_SPEC> {
        DONE_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - error flag"]
    #[inline(always)]
    pub fn error_int_ena(&mut self) -> ERROR_INT_ENA_W<'_, INT_ENA_SPEC> {
        ERROR_INT_ENA_W::new(self, 1)
    }
}
#[doc = "Read only register for error and done\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
