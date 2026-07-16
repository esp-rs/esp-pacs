#[doc = "Register `SRAM_CTRL` reader"]
pub type R = crate::R<SRAM_CTRL_SPEC>;
#[doc = "Register `SRAM_CTRL` writer"]
pub type W = crate::W<SRAM_CTRL_SPEC>;
#[doc = "Field `MEM_AUX_CTRL` reader - Control signals"]
pub type MEM_AUX_CTRL_R = crate::FieldReader<u16>;
#[doc = "Field `MEM_AUX_CTRL` writer - Control signals"]
pub type MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Control signals"]
    #[inline(always)]
    pub fn mem_aux_ctrl(&self) -> MEM_AUX_CTRL_R {
        MEM_AUX_CTRL_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_CTRL")
            .field("mem_aux_ctrl", &self.mem_aux_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - Control signals"]
    #[inline(always)]
    pub fn mem_aux_ctrl(&mut self) -> MEM_AUX_CTRL_W<'_, SRAM_CTRL_SPEC> {
        MEM_AUX_CTRL_W::new(self, 0)
    }
}
#[doc = "PPA SRAM Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_CTRL_SPEC;
impl crate::RegisterSpec for SRAM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_ctrl::R`](R) reader structure"]
impl crate::Readable for SRAM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_ctrl::W`](W) writer structure"]
impl crate::Writable for SRAM_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_CTRL to value 0x1320"]
impl crate::Resettable for SRAM_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x1320;
}
