#[doc = "Register `CLKDIV` reader"]
pub type R = crate::R<CLKDIV_SPEC>;
#[doc = "Register `CLKDIV` writer"]
pub type W = crate::W<CLKDIV_SPEC>;
#[doc = "Field `CLK_DIVIDER0` reader - Clock divider0 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type CLK_DIVIDER0_R = crate::FieldReader;
#[doc = "Field `CLK_DIVIDER0` writer - Clock divider0 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type CLK_DIVIDER0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLK_DIVIDER1` reader - Clock divider1 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type CLK_DIVIDER1_R = crate::FieldReader;
#[doc = "Field `CLK_DIVIDER1` writer - Clock divider1 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type CLK_DIVIDER1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLK_DIVIDER2` reader - Clock divider2 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type CLK_DIVIDER2_R = crate::FieldReader;
#[doc = "Field `CLK_DIVIDER2` writer - Clock divider2 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type CLK_DIVIDER2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLK_DIVIDER3` reader - Clock divider3 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type CLK_DIVIDER3_R = crate::FieldReader;
#[doc = "Field `CLK_DIVIDER3` writer - Clock divider3 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type CLK_DIVIDER3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clock divider0 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    pub fn clk_divider0(&self) -> CLK_DIVIDER0_R {
        CLK_DIVIDER0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock divider1 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    pub fn clk_divider1(&self) -> CLK_DIVIDER1_R {
        CLK_DIVIDER1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Clock divider2 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    pub fn clk_divider2(&self) -> CLK_DIVIDER2_R {
        CLK_DIVIDER2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Clock divider3 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    pub fn clk_divider3(&self) -> CLK_DIVIDER3_R {
        CLK_DIVIDER3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKDIV")
            .field("clk_divider0", &self.clk_divider0())
            .field("clk_divider1", &self.clk_divider1())
            .field("clk_divider2", &self.clk_divider2())
            .field("clk_divider3", &self.clk_divider3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divider0 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    pub fn clk_divider0(&mut self) -> CLK_DIVIDER0_W<CLKDIV_SPEC> {
        CLK_DIVIDER0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Clock divider1 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    pub fn clk_divider1(&mut self) -> CLK_DIVIDER1_W<CLKDIV_SPEC> {
        CLK_DIVIDER1_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Clock divider2 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    pub fn clk_divider2(&mut self) -> CLK_DIVIDER2_W<CLKDIV_SPEC> {
        CLK_DIVIDER2_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Clock divider3 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    pub fn clk_divider3(&mut self) -> CLK_DIVIDER3_W<CLKDIV_SPEC> {
        CLK_DIVIDER3_W::new(self, 24)
    }
}
#[doc = "Clock divider configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKDIV_SPEC;
impl crate::RegisterSpec for CLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv::R`](R) reader structure"]
impl crate::Readable for CLKDIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkdiv::W`](W) writer structure"]
impl crate::Writable for CLKDIV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKDIV to value 0"]
impl crate::Resettable for CLKDIV_SPEC {
    const RESET_VALUE: u32 = 0;
}
