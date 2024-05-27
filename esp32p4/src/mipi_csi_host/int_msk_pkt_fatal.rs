#[doc = "Register `INT_MSK_PKT_FATAL` reader"]
pub type R = crate::R<INT_MSK_PKT_FATAL_SPEC>;
#[doc = "Register `INT_MSK_PKT_FATAL` writer"]
pub type W = crate::W<INT_MSK_PKT_FATAL_SPEC>;
#[doc = "Field `MASK_ERR_ECC_DOUBLE` reader - NA"]
pub type MASK_ERR_ECC_DOUBLE_R = crate::BitReader;
#[doc = "Field `MASK_ERR_ECC_DOUBLE` writer - NA"]
pub type MASK_ERR_ECC_DOUBLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_SHORTER_PAYLOAD` reader - NA"]
pub type MASK_SHORTER_PAYLOAD_R = crate::BitReader;
#[doc = "Field `MASK_SHORTER_PAYLOAD` writer - NA"]
pub type MASK_SHORTER_PAYLOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn mask_err_ecc_double(&self) -> MASK_ERR_ECC_DOUBLE_R {
        MASK_ERR_ECC_DOUBLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn mask_shorter_payload(&self) -> MASK_SHORTER_PAYLOAD_R {
        MASK_SHORTER_PAYLOAD_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_MSK_PKT_FATAL")
            .field("mask_err_ecc_double", &self.mask_err_ecc_double())
            .field("mask_shorter_payload", &self.mask_shorter_payload())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn mask_err_ecc_double(&mut self) -> MASK_ERR_ECC_DOUBLE_W<INT_MSK_PKT_FATAL_SPEC> {
        MASK_ERR_ECC_DOUBLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn mask_shorter_payload(&mut self) -> MASK_SHORTER_PAYLOAD_W<INT_MSK_PKT_FATAL_SPEC> {
        MASK_SHORTER_PAYLOAD_W::new(self, 1)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_msk_pkt_fatal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_msk_pkt_fatal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_MSK_PKT_FATAL_SPEC;
impl crate::RegisterSpec for INT_MSK_PKT_FATAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_msk_pkt_fatal::R`](R) reader structure"]
impl crate::Readable for INT_MSK_PKT_FATAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_msk_pkt_fatal::W`](W) writer structure"]
impl crate::Writable for INT_MSK_PKT_FATAL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_MSK_PKT_FATAL to value 0"]
impl crate::Resettable for INT_MSK_PKT_FATAL_SPEC {
    const RESET_VALUE: u32 = 0;
}
