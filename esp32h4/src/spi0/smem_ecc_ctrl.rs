#[doc = "Register `SMEM_ECC_CTRL` reader"]
pub type R = crate::R<SMEM_ECC_CTRL_SPEC>;
#[doc = "Register `SMEM_ECC_CTRL` writer"]
pub type W = crate::W<SMEM_ECC_CTRL_SPEC>;
#[doc = "Field `SMEM_ECC_ERR_INT_EN` reader - Set this bit to calculate the error times of MSPI ECC read when accesses to external RAM."]
pub type SMEM_ECC_ERR_INT_EN_R = crate::BitReader;
#[doc = "Field `SMEM_ECC_ERR_INT_EN` writer - Set this bit to calculate the error times of MSPI ECC read when accesses to external RAM."]
pub type SMEM_ECC_ERR_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_PAGE_SIZE` reader - Set the page size of the external RAM accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub type SMEM_PAGE_SIZE_R = crate::FieldReader;
#[doc = "Field `SMEM_PAGE_SIZE` writer - Set the page size of the external RAM accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub type SMEM_PAGE_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SMEM_ECC_ADDR_EN` reader - Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of external RAM. If there is no ECC region in external RAM, this bit should be 0. Otherwise, this bit should be 1."]
pub type SMEM_ECC_ADDR_EN_R = crate::BitReader;
#[doc = "Field `SMEM_ECC_ADDR_EN` writer - Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of external RAM. If there is no ECC region in external RAM, this bit should be 0. Otherwise, this bit should be 1."]
pub type SMEM_ECC_ADDR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 17 - Set this bit to calculate the error times of MSPI ECC read when accesses to external RAM."]
    #[inline(always)]
    pub fn smem_ecc_err_int_en(&self) -> SMEM_ECC_ERR_INT_EN_R {
        SMEM_ECC_ERR_INT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Set the page size of the external RAM accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
    #[inline(always)]
    pub fn smem_page_size(&self) -> SMEM_PAGE_SIZE_R {
        SMEM_PAGE_SIZE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of external RAM. If there is no ECC region in external RAM, this bit should be 0. Otherwise, this bit should be 1."]
    #[inline(always)]
    pub fn smem_ecc_addr_en(&self) -> SMEM_ECC_ADDR_EN_R {
        SMEM_ECC_ADDR_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMEM_ECC_CTRL")
            .field("smem_ecc_err_int_en", &self.smem_ecc_err_int_en())
            .field("smem_page_size", &self.smem_page_size())
            .field("smem_ecc_addr_en", &self.smem_ecc_addr_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 17 - Set this bit to calculate the error times of MSPI ECC read when accesses to external RAM."]
    #[inline(always)]
    pub fn smem_ecc_err_int_en(&mut self) -> SMEM_ECC_ERR_INT_EN_W<'_, SMEM_ECC_CTRL_SPEC> {
        SMEM_ECC_ERR_INT_EN_W::new(self, 17)
    }
    #[doc = "Bits 18:19 - Set the page size of the external RAM accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
    #[inline(always)]
    pub fn smem_page_size(&mut self) -> SMEM_PAGE_SIZE_W<'_, SMEM_ECC_CTRL_SPEC> {
        SMEM_PAGE_SIZE_W::new(self, 18)
    }
    #[doc = "Bit 20 - Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of external RAM. If there is no ECC region in external RAM, this bit should be 0. Otherwise, this bit should be 1."]
    #[inline(always)]
    pub fn smem_ecc_addr_en(&mut self) -> SMEM_ECC_ADDR_EN_W<'_, SMEM_ECC_CTRL_SPEC> {
        SMEM_ECC_ADDR_EN_W::new(self, 20)
    }
}
#[doc = "MSPI ECC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_ecc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_ecc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets SMEM_ECC_CTRL to value 0x0008_0000"]
impl crate::Resettable for SMEM_ECC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0008_0000;
}
