#[doc = "Register `SRAM_CTRL_0` reader"]
pub type R = crate::R<SRAM_CTRL_0_SPEC>;
#[doc = "Register `SRAM_CTRL_0` writer"]
pub type W = crate::W<SRAM_CTRL_0_SPEC>;
#[doc = "Field `SRAM_FO` reader - This field is used to force on clock gate of internal SRAM."]
pub type SRAM_FO_R = crate::FieldReader<u32>;
#[doc = "Field `SRAM_FO` writer - This field is used to force on clock gate of internal SRAM."]
pub type SRAM_FO_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - This field is used to force on clock gate of internal SRAM."]
    #[inline(always)]
    pub fn sram_fo(&self) -> SRAM_FO_R {
        SRAM_FO_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_CTRL_0")
            .field("sram_fo", &self.sram_fo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:21 - This field is used to force on clock gate of internal SRAM."]
    #[inline(always)]
    pub fn sram_fo(&mut self) -> SRAM_FO_W<'_, SRAM_CTRL_0_SPEC> {
        SRAM_FO_W::new(self, 0)
    }
}
#[doc = "System SRAM configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_ctrl_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_ctrl_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_CTRL_0_SPEC;
impl crate::RegisterSpec for SRAM_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_ctrl_0::R`](R) reader structure"]
impl crate::Readable for SRAM_CTRL_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_ctrl_0::W`](W) writer structure"]
impl crate::Writable for SRAM_CTRL_0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_CTRL_0 to value 0x003f_ffff"]
impl crate::Resettable for SRAM_CTRL_0_SPEC {
    const RESET_VALUE: u32 = 0x003f_ffff;
}
