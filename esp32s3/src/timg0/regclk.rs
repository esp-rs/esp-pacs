#[doc = "Register `REGCLK` reader"]
pub type R = crate::R<REGCLK_SPEC>;
#[doc = "Register `REGCLK` writer"]
pub type W = crate::W<REGCLK_SPEC>;
#[doc = "Field `CLK_EN` reader - Register clock gate signal. 1: The clock for software to read and write registers is always on. 0: The clock for software to read and write registers only exits when the operation happens."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Register clock gate signal. 1: The clock for software to read and write registers is always on. 0: The clock for software to read and write registers only exits when the operation happens."]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Register clock gate signal. 1: The clock for software to read and write registers is always on. 0: The clock for software to read and write registers only exits when the operation happens."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGCLK")
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - Register clock gate signal. 1: The clock for software to read and write registers is always on. 0: The clock for software to read and write registers only exits when the operation happens."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<REGCLK_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
#[doc = "Timer group clock gate register\n\nYou can [`read`](crate::Reg::read) this register and get [`regclk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regclk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGCLK_SPEC;
impl crate::RegisterSpec for REGCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regclk::R`](R) reader structure"]
impl crate::Readable for REGCLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`regclk::W`](W) writer structure"]
impl crate::Writable for REGCLK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGCLK to value 0"]
impl crate::Resettable for REGCLK_SPEC {
    const RESET_VALUE: u32 = 0;
}
