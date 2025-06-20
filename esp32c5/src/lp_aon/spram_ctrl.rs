#[doc = "Register `SPRAM_CTRL` reader"]
pub type R = crate::R<SPRAM_CTRL_SPEC>;
#[doc = "Register `SPRAM_CTRL` writer"]
pub type W = crate::W<SPRAM_CTRL_SPEC>;
#[doc = "Field `SPRAM_MEM_AUX_CTRL` reader - configure lp memory power status"]
pub type SPRAM_MEM_AUX_CTRL_R = crate::FieldReader<u32>;
#[doc = "Field `SPRAM_MEM_AUX_CTRL` writer - configure lp memory power status"]
pub type SPRAM_MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - configure lp memory power status"]
    #[inline(always)]
    pub fn spram_mem_aux_ctrl(&self) -> SPRAM_MEM_AUX_CTRL_R {
        SPRAM_MEM_AUX_CTRL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPRAM_CTRL")
            .field("spram_mem_aux_ctrl", &self.spram_mem_aux_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - configure lp memory power status"]
    #[inline(always)]
    pub fn spram_mem_aux_ctrl(&mut self) -> SPRAM_MEM_AUX_CTRL_W<SPRAM_CTRL_SPEC> {
        SPRAM_MEM_AUX_CTRL_W::new(self, 0)
    }
}
#[doc = "configure lp memory power status\n\nYou can [`read`](crate::Reg::read) this register and get [`spram_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spram_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPRAM_CTRL_SPEC;
impl crate::RegisterSpec for SPRAM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spram_ctrl::R`](R) reader structure"]
impl crate::Readable for SPRAM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spram_ctrl::W`](W) writer structure"]
impl crate::Writable for SPRAM_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPRAM_CTRL to value 0x2070"]
impl crate::Resettable for SPRAM_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x2070;
}
