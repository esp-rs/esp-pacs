///Register `POWER_PD_HPMEM_MASK` reader
pub type R = crate::R<POWER_PD_HPMEM_MASK_SPEC>;
///Register `POWER_PD_HPMEM_MASK` writer
pub type W = crate::W<POWER_PD_HPMEM_MASK_SPEC>;
///Field `XPD_HP_MEM_MASK` reader - need_des
pub type XPD_HP_MEM_MASK_R = crate::FieldReader;
///Field `XPD_HP_MEM_MASK` writer - need_des
pub type XPD_HP_MEM_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `PD_HP_MEM_MASK` reader - need_des
pub type PD_HP_MEM_MASK_R = crate::FieldReader;
///Field `PD_HP_MEM_MASK` writer - need_des
pub type PD_HP_MEM_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - need_des
    #[inline(always)]
    pub fn xpd_hp_mem_mask(&self) -> XPD_HP_MEM_MASK_R {
        XPD_HP_MEM_MASK_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 26:31 - need_des
    #[inline(always)]
    pub fn pd_hp_mem_mask(&self) -> PD_HP_MEM_MASK_R {
        PD_HP_MEM_MASK_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_PD_HPMEM_MASK")
            .field("xpd_hp_mem_mask", &self.xpd_hp_mem_mask())
            .field("pd_hp_mem_mask", &self.pd_hp_mem_mask())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - need_des
    #[inline(always)]
    #[must_use]
    pub fn xpd_hp_mem_mask(&mut self) -> XPD_HP_MEM_MASK_W<POWER_PD_HPMEM_MASK_SPEC> {
        XPD_HP_MEM_MASK_W::new(self, 0)
    }
    ///Bits 26:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn pd_hp_mem_mask(&mut self) -> PD_HP_MEM_MASK_W<POWER_PD_HPMEM_MASK_SPEC> {
        PD_HP_MEM_MASK_W::new(self, 26)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`power_pd_hpmem_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_pd_hpmem_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct POWER_PD_HPMEM_MASK_SPEC;
impl crate::RegisterSpec for POWER_PD_HPMEM_MASK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`power_pd_hpmem_mask::R`](R) reader structure
impl crate::Readable for POWER_PD_HPMEM_MASK_SPEC {}
///`write(|w| ..)` method takes [`power_pd_hpmem_mask::W`](W) writer structure
impl crate::Writable for POWER_PD_HPMEM_MASK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets POWER_PD_HPMEM_MASK to value 0
impl crate::Resettable for POWER_PD_HPMEM_MASK_SPEC {
    const RESET_VALUE: u32 = 0;
}
