#[doc = "Register `KM_PD_CTRL` reader"]
pub type R = crate::R<KM_PD_CTRL_SPEC>;
#[doc = "Register `KM_PD_CTRL` writer"]
pub type W = crate::W<KM_PD_CTRL_SPEC>;
#[doc = "Field `KM_MEM_FORCE_PU` reader - Set this bit to force power up KM memory."]
pub type KM_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `KM_MEM_FORCE_PU` writer - Set this bit to force power up KM memory."]
pub type KM_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KM_MEM_FORCE_PD` reader - Set this bit to force power down KM memory."]
pub type KM_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `KM_MEM_FORCE_PD` writer - Set this bit to force power down KM memory."]
pub type KM_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Set this bit to force power up KM memory."]
    #[inline(always)]
    pub fn km_mem_force_pu(&self) -> KM_MEM_FORCE_PU_R {
        KM_MEM_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force power down KM memory."]
    #[inline(always)]
    pub fn km_mem_force_pd(&self) -> KM_MEM_FORCE_PD_R {
        KM_MEM_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KM_PD_CTRL")
            .field("km_mem_force_pu", &self.km_mem_force_pu())
            .field("km_mem_force_pd", &self.km_mem_force_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Set this bit to force power up KM memory."]
    #[inline(always)]
    pub fn km_mem_force_pu(&mut self) -> KM_MEM_FORCE_PU_W<KM_PD_CTRL_SPEC> {
        KM_MEM_FORCE_PU_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force power down KM memory."]
    #[inline(always)]
    pub fn km_mem_force_pd(&mut self) -> KM_MEM_FORCE_PD_W<KM_PD_CTRL_SPEC> {
        KM_MEM_FORCE_PD_W::new(self, 2)
    }
}
#[doc = "Key Manager power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`km_pd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`km_pd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KM_PD_CTRL_SPEC;
impl crate::RegisterSpec for KM_PD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`km_pd_ctrl::R`](R) reader structure"]
impl crate::Readable for KM_PD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`km_pd_ctrl::W`](W) writer structure"]
impl crate::Writable for KM_PD_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KM_PD_CTRL to value 0x04"]
impl crate::Resettable for KM_PD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
