#[doc = "Register `SMEM_PMS%s_ADDR` reader"]
pub type R = crate::R<SMEM_PMS_ADDR_SPEC>;
#[doc = "Register `SMEM_PMS%s_ADDR` writer"]
pub type W = crate::W<SMEM_PMS_ADDR_SPEC>;
#[doc = "Field `S` reader - SPI1 external RAM PMS section %s start address value"]
pub type S_R = crate::FieldReader<u32>;
#[doc = "Field `S` writer - SPI1 external RAM PMS section %s start address value"]
pub type S_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - SPI1 external RAM PMS section %s start address value"]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(self.bits & 0x1fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMEM_PMS_ADDR")
            .field("s", &self.s())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:28 - SPI1 external RAM PMS section %s start address value"]
    #[inline(always)]
    pub fn s(&mut self) -> S_W<'_, SMEM_PMS_ADDR_SPEC> {
        S_W::new(self, 0)
    }
}
#[doc = "SPI1 external RAM PMS section %s start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_pms_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_pms_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMEM_PMS_ADDR_SPEC;
impl crate::RegisterSpec for SMEM_PMS_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smem_pms_addr::R`](R) reader structure"]
impl crate::Readable for SMEM_PMS_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smem_pms_addr::W`](W) writer structure"]
impl crate::Writable for SMEM_PMS_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMEM_PMS%s_ADDR to value 0"]
impl crate::Resettable for SMEM_PMS_ADDR_SPEC {}
