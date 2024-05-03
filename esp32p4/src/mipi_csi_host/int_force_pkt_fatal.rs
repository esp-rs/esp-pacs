#[doc = "Register `INT_FORCE_PKT_FATAL` reader"]
pub type R = crate::R<INT_FORCE_PKT_FATAL_SPEC>;
#[doc = "Register `INT_FORCE_PKT_FATAL` writer"]
pub type W = crate::W<INT_FORCE_PKT_FATAL_SPEC>;
#[doc = "Field `FORCE_ERR_ECC_DOUBLE` reader - NA"]
pub type FORCE_ERR_ECC_DOUBLE_R = crate::BitReader;
#[doc = "Field `FORCE_ERR_ECC_DOUBLE` writer - NA"]
pub type FORCE_ERR_ECC_DOUBLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_SHORTER_PAYLOAD` reader - NA"]
pub type FORCE_SHORTER_PAYLOAD_R = crate::BitReader;
#[doc = "Field `FORCE_SHORTER_PAYLOAD` writer - NA"]
pub type FORCE_SHORTER_PAYLOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn force_err_ecc_double(&self) -> FORCE_ERR_ECC_DOUBLE_R {
        FORCE_ERR_ECC_DOUBLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn force_shorter_payload(&self) -> FORCE_SHORTER_PAYLOAD_R {
        FORCE_SHORTER_PAYLOAD_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_FORCE_PKT_FATAL")
            .field("force_err_ecc_double", &self.force_err_ecc_double().bit())
            .field("force_shorter_payload", &self.force_shorter_payload().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_FORCE_PKT_FATAL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_err_ecc_double(&mut self) -> FORCE_ERR_ECC_DOUBLE_W<INT_FORCE_PKT_FATAL_SPEC> {
        FORCE_ERR_ECC_DOUBLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_shorter_payload(&mut self) -> FORCE_SHORTER_PAYLOAD_W<INT_FORCE_PKT_FATAL_SPEC> {
        FORCE_SHORTER_PAYLOAD_W::new(self, 1)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_force_pkt_fatal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_force_pkt_fatal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_FORCE_PKT_FATAL_SPEC;
impl crate::RegisterSpec for INT_FORCE_PKT_FATAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_force_pkt_fatal::R`](R) reader structure"]
impl crate::Readable for INT_FORCE_PKT_FATAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_force_pkt_fatal::W`](W) writer structure"]
impl crate::Writable for INT_FORCE_PKT_FATAL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_FORCE_PKT_FATAL to value 0"]
impl crate::Resettable for INT_FORCE_PKT_FATAL_SPEC {
    const RESET_VALUE: u32 = 0;
}
