#[doc = "Register `MEM_MMU_POWER_CTRL` reader"]
pub type R = crate::R<MEM_MMU_POWER_CTRL_SPEC>;
#[doc = "Register `MEM_MMU_POWER_CTRL` writer"]
pub type W = crate::W<MEM_MMU_POWER_CTRL_SPEC>;
#[doc = "Field `MMU_MEM_FORCE_ON` reader - Set this bit to enable mmu-memory clock force on"]
pub type MMU_MEM_FORCE_ON_R = crate::BitReader;
#[doc = "Field `MMU_MEM_FORCE_ON` writer - Set this bit to enable mmu-memory clock force on"]
pub type MMU_MEM_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMU_MEM_FORCE_PD` reader - Set this bit to force mmu-memory powerdown"]
pub type MMU_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `MMU_MEM_FORCE_PD` writer - Set this bit to force mmu-memory powerdown"]
pub type MMU_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMU_MEM_FORCE_PU` reader - Set this bit to force mmu-memory powerup, in this case, the power should also be controlled by rtc."]
pub type MMU_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `MMU_MEM_FORCE_PU` writer - Set this bit to force mmu-memory powerup, in this case, the power should also be controlled by rtc."]
pub type MMU_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMU_PAGE_SIZE` reader - 0: Max page size , 1: Max page size/2 , 2: Max page size/4, 3: Max page size/8"]
pub type MMU_PAGE_SIZE_R = crate::FieldReader;
#[doc = "Field `MMU_PAGE_SIZE` writer - 0: Max page size , 1: Max page size/2 , 2: Max page size/4, 3: Max page size/8"]
pub type MMU_PAGE_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM_AUX_CTRL` reader - MMU PSRAM aux control register"]
pub type MEM_AUX_CTRL_R = crate::FieldReader<u16>;
#[doc = "Field `MEM_AUX_CTRL` writer - MMU PSRAM aux control register"]
pub type MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `MEM_RDN_ENA` reader - ECO register enable bit"]
pub type MEM_RDN_ENA_R = crate::BitReader;
#[doc = "Field `MEM_RDN_ENA` writer - ECO register enable bit"]
pub type MEM_RDN_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_RDN_RESULT` reader - MSPI module clock domain and AXI clock domain ECO register result register"]
pub type MEM_RDN_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set this bit to enable mmu-memory clock force on"]
    #[inline(always)]
    pub fn mmu_mem_force_on(&self) -> MMU_MEM_FORCE_ON_R {
        MMU_MEM_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to force mmu-memory powerdown"]
    #[inline(always)]
    pub fn mmu_mem_force_pd(&self) -> MMU_MEM_FORCE_PD_R {
        MMU_MEM_FORCE_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force mmu-memory powerup, in this case, the power should also be controlled by rtc."]
    #[inline(always)]
    pub fn mmu_mem_force_pu(&self) -> MMU_MEM_FORCE_PU_R {
        MMU_MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 0: Max page size , 1: Max page size/2 , 2: Max page size/4, 3: Max page size/8"]
    #[inline(always)]
    pub fn mmu_page_size(&self) -> MMU_PAGE_SIZE_R {
        MMU_PAGE_SIZE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 16:29 - MMU PSRAM aux control register"]
    #[inline(always)]
    pub fn mem_aux_ctrl(&self) -> MEM_AUX_CTRL_R {
        MEM_AUX_CTRL_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 30 - ECO register enable bit"]
    #[inline(always)]
    pub fn mem_rdn_ena(&self) -> MEM_RDN_ENA_R {
        MEM_RDN_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - MSPI module clock domain and AXI clock domain ECO register result register"]
    #[inline(always)]
    pub fn mem_rdn_result(&self) -> MEM_RDN_RESULT_R {
        MEM_RDN_RESULT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_MMU_POWER_CTRL")
            .field("mmu_mem_force_on", &self.mmu_mem_force_on())
            .field("mmu_mem_force_pd", &self.mmu_mem_force_pd())
            .field("mmu_mem_force_pu", &self.mmu_mem_force_pu())
            .field("mmu_page_size", &self.mmu_page_size())
            .field("mem_aux_ctrl", &self.mem_aux_ctrl())
            .field("mem_rdn_ena", &self.mem_rdn_ena())
            .field("mem_rdn_result", &self.mem_rdn_result())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable mmu-memory clock force on"]
    #[inline(always)]
    pub fn mmu_mem_force_on(&mut self) -> MMU_MEM_FORCE_ON_W<MEM_MMU_POWER_CTRL_SPEC> {
        MMU_MEM_FORCE_ON_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to force mmu-memory powerdown"]
    #[inline(always)]
    pub fn mmu_mem_force_pd(&mut self) -> MMU_MEM_FORCE_PD_W<MEM_MMU_POWER_CTRL_SPEC> {
        MMU_MEM_FORCE_PD_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force mmu-memory powerup, in this case, the power should also be controlled by rtc."]
    #[inline(always)]
    pub fn mmu_mem_force_pu(&mut self) -> MMU_MEM_FORCE_PU_W<MEM_MMU_POWER_CTRL_SPEC> {
        MMU_MEM_FORCE_PU_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - 0: Max page size , 1: Max page size/2 , 2: Max page size/4, 3: Max page size/8"]
    #[inline(always)]
    pub fn mmu_page_size(&mut self) -> MMU_PAGE_SIZE_W<MEM_MMU_POWER_CTRL_SPEC> {
        MMU_PAGE_SIZE_W::new(self, 3)
    }
    #[doc = "Bits 16:29 - MMU PSRAM aux control register"]
    #[inline(always)]
    pub fn mem_aux_ctrl(&mut self) -> MEM_AUX_CTRL_W<MEM_MMU_POWER_CTRL_SPEC> {
        MEM_AUX_CTRL_W::new(self, 16)
    }
    #[doc = "Bit 30 - ECO register enable bit"]
    #[inline(always)]
    pub fn mem_rdn_ena(&mut self) -> MEM_RDN_ENA_W<MEM_MMU_POWER_CTRL_SPEC> {
        MEM_RDN_ENA_W::new(self, 30)
    }
}
#[doc = "MSPI MMU power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_mmu_power_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_mmu_power_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_MMU_POWER_CTRL_SPEC;
impl crate::RegisterSpec for MEM_MMU_POWER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_mmu_power_ctrl::R`](R) reader structure"]
impl crate::Readable for MEM_MMU_POWER_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_mmu_power_ctrl::W`](W) writer structure"]
impl crate::Writable for MEM_MMU_POWER_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_MMU_POWER_CTRL to value 0x1320_0004"]
impl crate::Resettable for MEM_MMU_POWER_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x1320_0004;
}
