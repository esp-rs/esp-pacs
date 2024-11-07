#[doc = "Register `SRAM_CTRL_2` reader"]
pub type R = crate::R<SRAM_CTRL_2_SPEC>;
#[doc = "Register `SRAM_CTRL_2` writer"]
pub type W = crate::W<SRAM_CTRL_2_SPEC>;
#[doc = "Field `SRAM_FORCE_PU` reader - This field is used to power up internal SRAM."]
pub type SRAM_FORCE_PU_R = crate::FieldReader<u32>;
#[doc = "Field `SRAM_FORCE_PU` writer - This field is used to power up internal SRAM."]
pub type SRAM_FORCE_PU_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - This field is used to power up internal SRAM."]
    #[inline(always)]
    pub fn sram_force_pu(&self) -> SRAM_FORCE_PU_R {
        SRAM_FORCE_PU_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_CTRL_2")
            .field("sram_force_pu", &self.sram_force_pu())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:21 - This field is used to power up internal SRAM."]
    #[inline(always)]
    pub fn sram_force_pu(&mut self) -> SRAM_FORCE_PU_W<SRAM_CTRL_2_SPEC> {
        SRAM_FORCE_PU_W::new(self, 0)
    }
}
#[doc = "System SRAM configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_ctrl_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_ctrl_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_CTRL_2_SPEC;
impl crate::RegisterSpec for SRAM_CTRL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_ctrl_2::R`](R) reader structure"]
impl crate::Readable for SRAM_CTRL_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_ctrl_2::W`](W) writer structure"]
impl crate::Writable for SRAM_CTRL_2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAM_CTRL_2 to value 0x003f_ffff"]
impl crate::Resettable for SRAM_CTRL_2_SPEC {
    const RESET_VALUE: u32 = 0x003f_ffff;
}
