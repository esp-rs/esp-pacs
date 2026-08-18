#[doc = "Register `SMEM_ECC_CTRL` reader"]
pub type R = crate::R<SMEM_ECC_CTRL_SPEC>;
#[doc = "Register `SMEM_ECC_CTRL` writer"]
pub type W = crate::W<SMEM_ECC_CTRL_SPEC>;
#[doc = "Field `ECC_ERR_INT_EN` reader - "]
pub type ECC_ERR_INT_EN_R = crate::BitReader;
#[doc = "Field `ECC_ERR_INT_EN` writer - "]
pub type ECC_ERR_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAGE_SIZE` reader - "]
pub type PAGE_SIZE_R = crate::FieldReader;
#[doc = "Field `PAGE_SIZE` writer - "]
pub type PAGE_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECC_ADDR_EN` reader - "]
pub type ECC_ADDR_EN_R = crate::BitReader;
#[doc = "Field `ECC_ADDR_EN` writer - "]
pub type ECC_ADDR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ecc_err_int_en(&self) -> ECC_ERR_INT_EN_R {
        ECC_ERR_INT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn page_size(&self) -> PAGE_SIZE_R {
        PAGE_SIZE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ecc_addr_en(&self) -> ECC_ADDR_EN_R {
        ECC_ADDR_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMEM_ECC_CTRL")
            .field("ecc_err_int_en", &self.ecc_err_int_en())
            .field("page_size", &self.page_size())
            .field("ecc_addr_en", &self.ecc_addr_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ecc_err_int_en(&mut self) -> ECC_ERR_INT_EN_W<'_, SMEM_ECC_CTRL_SPEC> {
        ECC_ERR_INT_EN_W::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn page_size(&mut self) -> PAGE_SIZE_W<'_, SMEM_ECC_CTRL_SPEC> {
        PAGE_SIZE_W::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ecc_addr_en(&mut self) -> ECC_ADDR_EN_W<'_, SMEM_ECC_CTRL_SPEC> {
        ECC_ADDR_EN_W::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_ecc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_ecc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMEM_ECC_CTRL_SPEC;
impl crate::RegisterSpec for SMEM_ECC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smem_ecc_ctrl::R`](R) reader structure"]
impl crate::Readable for SMEM_ECC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smem_ecc_ctrl::W`](W) writer structure"]
impl crate::Writable for SMEM_ECC_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMEM_ECC_CTRL to value 0"]
impl crate::Resettable for SMEM_ECC_CTRL_SPEC {}
