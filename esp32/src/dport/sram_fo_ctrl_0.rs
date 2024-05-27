#[doc = "Register `SRAM_FO_CTRL_0` reader"]
pub type R = crate::R<SRAM_FO_CTRL_0_SPEC>;
#[doc = "Register `SRAM_FO_CTRL_0` writer"]
pub type W = crate::W<SRAM_FO_CTRL_0_SPEC>;
#[doc = "Field `SRAM_FO_0` reader - "]
pub type SRAM_FO_0_R = crate::FieldReader<u32>;
#[doc = "Field `SRAM_FO_0` writer - "]
pub type SRAM_FO_0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sram_fo_0(&self) -> SRAM_FO_0_R {
        SRAM_FO_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_FO_CTRL_0")
            .field("sram_fo_0", &self.sram_fo_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn sram_fo_0(&mut self) -> SRAM_FO_0_W<SRAM_FO_CTRL_0_SPEC> {
        SRAM_FO_0_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_fo_ctrl_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_fo_ctrl_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_FO_CTRL_0_SPEC;
impl crate::RegisterSpec for SRAM_FO_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_fo_ctrl_0::R`](R) reader structure"]
impl crate::Readable for SRAM_FO_CTRL_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_fo_ctrl_0::W`](W) writer structure"]
impl crate::Writable for SRAM_FO_CTRL_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAM_FO_CTRL_0 to value 0xffff_ffff"]
impl crate::Resettable for SRAM_FO_CTRL_0_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
