#[doc = "Register `DEBUG` reader"]
pub type R = crate::R<DEBUG_SPEC>;
#[doc = "Register `DEBUG` writer"]
pub type W = crate::W<DEBUG_SPEC>;
#[doc = "Field `FPGA_DEBUG` reader - Reserved"]
pub type FPGA_DEBUG_R = crate::BitReader;
#[doc = "Field `FPGA_DEBUG` writer - Reserved"]
pub type FPGA_DEBUG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn fpga_debug(&self) -> FPGA_DEBUG_R {
        FPGA_DEBUG_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG")
            .field("fpga_debug", &self.fpga_debug())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn fpga_debug(&mut self) -> FPGA_DEBUG_W<DEBUG_SPEC> {
        FPGA_DEBUG_W::new(self, 0)
    }
}
#[doc = "HP-SYSTEM debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`debug::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_SPEC;
impl crate::RegisterSpec for DEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug::R`](R) reader structure"]
impl crate::Readable for DEBUG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`debug::W`](W) writer structure"]
impl crate::Writable for DEBUG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG to value 0x01"]
impl crate::Resettable for DEBUG_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
