#[doc = "Register `ECC_ERR_ADDR` reader"]
pub type R = crate::R<ECC_ERR_ADDR_SPEC>;
#[doc = "Register `ECC_ERR_ADDR` writer"]
pub type W = crate::W<ECC_ERR_ADDR_SPEC>;
#[doc = "Field `ECC_ERR_ADDR` reader - "]
pub type ECC_ERR_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ECC_ERR_ADDR` writer - "]
pub type ECC_ERR_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28"]
    #[inline(always)]
    pub fn ecc_err_addr(&self) -> ECC_ERR_ADDR_R {
        ECC_ERR_ADDR_R::new(self.bits & 0x1fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECC_ERR_ADDR")
            .field("ecc_err_addr", &self.ecc_err_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:28"]
    #[inline(always)]
    pub fn ecc_err_addr(&mut self) -> ECC_ERR_ADDR_W<'_, ECC_ERR_ADDR_SPEC> {
        ECC_ERR_ADDR_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_err_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_err_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECC_ERR_ADDR_SPEC;
impl crate::RegisterSpec for ECC_ERR_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_err_addr::R`](R) reader structure"]
impl crate::Readable for ECC_ERR_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecc_err_addr::W`](W) writer structure"]
impl crate::Writable for ECC_ERR_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECC_ERR_ADDR to value 0"]
impl crate::Resettable for ECC_ERR_ADDR_SPEC {}
