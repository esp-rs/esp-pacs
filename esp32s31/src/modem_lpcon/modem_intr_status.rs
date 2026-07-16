#[doc = "Register `MODEM_INTR_STATUS` reader"]
pub type R = crate::R<MODEM_INTR_STATUS_SPEC>;
#[doc = "Register `MODEM_INTR_STATUS` writer"]
pub type W = crate::W<MODEM_INTR_STATUS_SPEC>;
#[doc = "Field `MODEM_INTR_VECTOR` reader - "]
pub type MODEM_INTR_VECTOR_R = crate::FieldReader<u16>;
#[doc = "Field `MODEM_INTR_VECTOR` writer - "]
pub type MODEM_INTR_VECTOR_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn modem_intr_vector(&self) -> MODEM_INTR_VECTOR_R {
        MODEM_INTR_VECTOR_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_INTR_STATUS")
            .field("modem_intr_vector", &self.modem_intr_vector())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn modem_intr_vector(&mut self) -> MODEM_INTR_VECTOR_W<'_, MODEM_INTR_STATUS_SPEC> {
        MODEM_INTR_VECTOR_W::new(self, 0)
    }
}
#[doc = "MODEM_INTR_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_intr_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_intr_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODEM_INTR_STATUS_SPEC;
impl crate::RegisterSpec for MODEM_INTR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modem_intr_status::R`](R) reader structure"]
impl crate::Readable for MODEM_INTR_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modem_intr_status::W`](W) writer structure"]
impl crate::Writable for MODEM_INTR_STATUS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODEM_INTR_STATUS to value 0"]
impl crate::Resettable for MODEM_INTR_STATUS_SPEC {}
