#[doc = "Register `MEM_PMS_REJECT` reader"]
pub type R = crate::R<MEM_PMS_REJECT_SPEC>;
#[doc = "Register `MEM_PMS_REJECT` writer"]
pub type W = crate::W<MEM_PMS_REJECT_SPEC>;
#[doc = "Field `MEM_PM_EN` reader - Set this bit to enable SPI0/1 transfer permission control function."]
pub type MEM_PM_EN_R = crate::BitReader;
#[doc = "Field `MEM_PM_EN` writer - Set this bit to enable SPI0/1 transfer permission control function."]
pub type MEM_PM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_PMS_LD` reader - 1: SPI1 write access error. 0: No write access error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
pub type MEM_PMS_LD_R = crate::BitReader;
#[doc = "Field `MEM_PMS_ST` reader - 1: SPI1 read access error. 0: No read access error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
pub type MEM_PMS_ST_R = crate::BitReader;
#[doc = "Field `MEM_PMS_MULTI_HIT` reader - 1: SPI1 access is rejected because of address miss. 0: No address miss error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
pub type MEM_PMS_MULTI_HIT_R = crate::BitReader;
#[doc = "Field `MEM_PMS_IVD` reader - 1: SPI1 access is rejected because of address multi-hit. 0: No address multi-hit error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
pub type MEM_PMS_IVD_R = crate::BitReader;
impl R {
    #[doc = "Bit 27 - Set this bit to enable SPI0/1 transfer permission control function."]
    #[inline(always)]
    pub fn mem_pm_en(&self) -> MEM_PM_EN_R {
        MEM_PM_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: SPI1 write access error. 0: No write access error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
    #[inline(always)]
    pub fn mem_pms_ld(&self) -> MEM_PMS_LD_R {
        MEM_PMS_LD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: SPI1 read access error. 0: No read access error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
    #[inline(always)]
    pub fn mem_pms_st(&self) -> MEM_PMS_ST_R {
        MEM_PMS_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: SPI1 access is rejected because of address miss. 0: No address miss error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
    #[inline(always)]
    pub fn mem_pms_multi_hit(&self) -> MEM_PMS_MULTI_HIT_R {
        MEM_PMS_MULTI_HIT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: SPI1 access is rejected because of address multi-hit. 0: No address multi-hit error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
    #[inline(always)]
    pub fn mem_pms_ivd(&self) -> MEM_PMS_IVD_R {
        MEM_PMS_IVD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_PMS_REJECT")
            .field("mem_pm_en", &self.mem_pm_en())
            .field("mem_pms_ld", &self.mem_pms_ld())
            .field("mem_pms_st", &self.mem_pms_st())
            .field("mem_pms_multi_hit", &self.mem_pms_multi_hit())
            .field("mem_pms_ivd", &self.mem_pms_ivd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 27 - Set this bit to enable SPI0/1 transfer permission control function."]
    #[inline(always)]
    pub fn mem_pm_en(&mut self) -> MEM_PM_EN_W<MEM_PMS_REJECT_SPEC> {
        MEM_PM_EN_W::new(self, 27)
    }
}
#[doc = "SPI1 access reject register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_pms_reject::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_pms_reject::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_PMS_REJECT_SPEC;
impl crate::RegisterSpec for MEM_PMS_REJECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_pms_reject::R`](R) reader structure"]
impl crate::Readable for MEM_PMS_REJECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_pms_reject::W`](W) writer structure"]
impl crate::Writable for MEM_PMS_REJECT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_PMS_REJECT to value 0"]
impl crate::Resettable for MEM_PMS_REJECT_SPEC {}
