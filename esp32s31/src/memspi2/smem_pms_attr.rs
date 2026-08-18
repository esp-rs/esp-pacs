#[doc = "Register `SMEM_PMS%s_ATTR` reader"]
pub type R = crate::R<SMEM_PMS_ATTR_SPEC>;
#[doc = "Register `SMEM_PMS%s_ATTR` writer"]
pub type W = crate::W<SMEM_PMS_ATTR_SPEC>;
#[doc = "Field `RD_ATTR` reader - "]
pub type RD_ATTR_R = crate::BitReader;
#[doc = "Field `RD_ATTR` writer - "]
pub type RD_ATTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_ATTR` reader - "]
pub type WR_ATTR_R = crate::BitReader;
#[doc = "Field `WR_ATTR` writer - "]
pub type WR_ATTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC` reader - "]
pub type ECC_R = crate::BitReader;
#[doc = "Field `ECC` writer - "]
pub type ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NONSECURE_RD_ATTR` reader - "]
pub type NONSECURE_RD_ATTR_R = crate::BitReader;
#[doc = "Field `NONSECURE_RD_ATTR` writer - "]
pub type NONSECURE_RD_ATTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NONSECURE_WR_ATTR` reader - "]
pub type NONSECURE_WR_ATTR_R = crate::BitReader;
#[doc = "Field `NONSECURE_WR_ATTR` writer - "]
pub type NONSECURE_WR_ATTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NONSECURE_ECC` reader - "]
pub type NONSECURE_ECC_R = crate::BitReader;
#[doc = "Field `NONSECURE_ECC` writer - "]
pub type NONSECURE_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rd_attr(&self) -> RD_ATTR_R {
        RD_ATTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wr_attr(&self) -> WR_ATTR_R {
        WR_ATTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn nonsecure_rd_attr(&self) -> NONSECURE_RD_ATTR_R {
        NONSECURE_RD_ATTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn nonsecure_wr_attr(&self) -> NONSECURE_WR_ATTR_R {
        NONSECURE_WR_ATTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn nonsecure_ecc(&self) -> NONSECURE_ECC_R {
        NONSECURE_ECC_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMEM_PMS_ATTR")
            .field("rd_attr", &self.rd_attr())
            .field("wr_attr", &self.wr_attr())
            .field("ecc", &self.ecc())
            .field("nonsecure_rd_attr", &self.nonsecure_rd_attr())
            .field("nonsecure_wr_attr", &self.nonsecure_wr_attr())
            .field("nonsecure_ecc", &self.nonsecure_ecc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rd_attr(&mut self) -> RD_ATTR_W<'_, SMEM_PMS_ATTR_SPEC> {
        RD_ATTR_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wr_attr(&mut self) -> WR_ATTR_W<'_, SMEM_PMS_ATTR_SPEC> {
        WR_ATTR_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ecc(&mut self) -> ECC_W<'_, SMEM_PMS_ATTR_SPEC> {
        ECC_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn nonsecure_rd_attr(&mut self) -> NONSECURE_RD_ATTR_W<'_, SMEM_PMS_ATTR_SPEC> {
        NONSECURE_RD_ATTR_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn nonsecure_wr_attr(&mut self) -> NONSECURE_WR_ATTR_W<'_, SMEM_PMS_ATTR_SPEC> {
        NONSECURE_WR_ATTR_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn nonsecure_ecc(&mut self) -> NONSECURE_ECC_W<'_, SMEM_PMS_ATTR_SPEC> {
        NONSECURE_ECC_W::new(self, 5)
    }
}
#[doc = "SPI1 external RAM PMS section 0 attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_pms_attr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_pms_attr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMEM_PMS_ATTR_SPEC;
impl crate::RegisterSpec for SMEM_PMS_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smem_pms_attr::R`](R) reader structure"]
impl crate::Readable for SMEM_PMS_ATTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smem_pms_attr::W`](W) writer structure"]
impl crate::Writable for SMEM_PMS_ATTR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMEM_PMS%s_ATTR to value 0x1b"]
impl crate::Resettable for SMEM_PMS_ATTR_SPEC {
    const RESET_VALUE: u32 = 0x1b;
}
