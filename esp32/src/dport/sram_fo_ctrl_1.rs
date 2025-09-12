#[doc = "Register `SRAM_FO_CTRL_1` reader"]
pub type R = crate::R<SRAM_FO_CTRL_1_SPEC>;
#[doc = "Register `SRAM_FO_CTRL_1` writer"]
pub type W = crate::W<SRAM_FO_CTRL_1_SPEC>;
#[doc = "Field `SRAM_FO_1` reader - "]
pub type SRAM_FO_1_R = crate::BitReader;
#[doc = "Field `SRAM_FO_1` writer - "]
pub type SRAM_FO_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sram_fo_1(&self) -> SRAM_FO_1_R {
        SRAM_FO_1_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_FO_CTRL_1")
            .field("sram_fo_1", &self.sram_fo_1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sram_fo_1(&mut self) -> SRAM_FO_1_W<'_, SRAM_FO_CTRL_1_SPEC> {
        SRAM_FO_1_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_fo_ctrl_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_fo_ctrl_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_FO_CTRL_1_SPEC;
impl crate::RegisterSpec for SRAM_FO_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_fo_ctrl_1::R`](R) reader structure"]
impl crate::Readable for SRAM_FO_CTRL_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_fo_ctrl_1::W`](W) writer structure"]
impl crate::Writable for SRAM_FO_CTRL_1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_FO_CTRL_1 to value 0x01"]
impl crate::Resettable for SRAM_FO_CTRL_1_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
