#[doc = "Register `SPROM_CTRL` reader"]
pub type R = crate::R<SPROM_CTRL_SPEC>;
#[doc = "Register `SPROM_CTRL` writer"]
pub type W = crate::W<SPROM_CTRL_SPEC>;
#[doc = "Field `SPROM_MEM_AUX_CTRL` reader - reserved"]
pub type SPROM_MEM_AUX_CTRL_R = crate::FieldReader<u32>;
#[doc = "Field `SPROM_MEM_AUX_CTRL` writer - reserved"]
pub type SPROM_MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn sprom_mem_aux_ctrl(&self) -> SPROM_MEM_AUX_CTRL_R {
        SPROM_MEM_AUX_CTRL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPROM_CTRL")
            .field("sprom_mem_aux_ctrl", &self.sprom_mem_aux_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn sprom_mem_aux_ctrl(&mut self) -> SPROM_MEM_AUX_CTRL_W<SPROM_CTRL_SPEC> {
        SPROM_MEM_AUX_CTRL_W::new(self, 0)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sprom_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprom_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPROM_CTRL_SPEC;
impl crate::RegisterSpec for SPROM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprom_ctrl::R`](R) reader structure"]
impl crate::Readable for SPROM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sprom_ctrl::W`](W) writer structure"]
impl crate::Writable for SPROM_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPROM_CTRL to value 0x70"]
impl crate::Resettable for SPROM_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x70;
}
