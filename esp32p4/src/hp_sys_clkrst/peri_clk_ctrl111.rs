#[doc = "Register `PERI_CLK_CTRL111` reader"]
pub type R = crate::R<PERI_CLK_CTRL111_SPEC>;
#[doc = "Register `PERI_CLK_CTRL111` writer"]
pub type W = crate::W<PERI_CLK_CTRL111_SPEC>;
#[doc = "Field `REG_UART0_SCLK_DIV_NUM` reader - Reserved"]
pub type REG_UART0_SCLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_UART0_SCLK_DIV_NUM` writer - Reserved"]
pub type REG_UART0_SCLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_UART0_SCLK_DIV_NUMERATOR` reader - Reserved"]
pub type REG_UART0_SCLK_DIV_NUMERATOR_R = crate::FieldReader;
#[doc = "Field `REG_UART0_SCLK_DIV_NUMERATOR` writer - Reserved"]
pub type REG_UART0_SCLK_DIV_NUMERATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_UART0_SCLK_DIV_DENOMINATOR` reader - Reserved"]
pub type REG_UART0_SCLK_DIV_DENOMINATOR_R = crate::FieldReader;
#[doc = "Field `REG_UART0_SCLK_DIV_DENOMINATOR` writer - Reserved"]
pub type REG_UART0_SCLK_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_UART1_CLK_SRC_SEL` reader - Reserved"]
pub type REG_UART1_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `REG_UART1_CLK_SRC_SEL` writer - Reserved"]
pub type REG_UART1_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_UART1_CLK_EN` reader - Reserved"]
pub type REG_UART1_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_UART1_CLK_EN` writer - Reserved"]
pub type REG_UART1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn reg_uart0_sclk_div_num(&self) -> REG_UART0_SCLK_DIV_NUM_R {
        REG_UART0_SCLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn reg_uart0_sclk_div_numerator(&self) -> REG_UART0_SCLK_DIV_NUMERATOR_R {
        REG_UART0_SCLK_DIV_NUMERATOR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn reg_uart0_sclk_div_denominator(&self) -> REG_UART0_SCLK_DIV_DENOMINATOR_R {
        REG_UART0_SCLK_DIV_DENOMINATOR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Reserved"]
    #[inline(always)]
    pub fn reg_uart1_clk_src_sel(&self) -> REG_UART1_CLK_SRC_SEL_R {
        REG_UART1_CLK_SRC_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn reg_uart1_clk_en(&self) -> REG_UART1_CLK_EN_R {
        REG_UART1_CLK_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL111")
            .field(
                "reg_uart0_sclk_div_num",
                &format_args!("{}", self.reg_uart0_sclk_div_num().bits()),
            )
            .field(
                "reg_uart0_sclk_div_numerator",
                &format_args!("{}", self.reg_uart0_sclk_div_numerator().bits()),
            )
            .field(
                "reg_uart0_sclk_div_denominator",
                &format_args!("{}", self.reg_uart0_sclk_div_denominator().bits()),
            )
            .field(
                "reg_uart1_clk_src_sel",
                &format_args!("{}", self.reg_uart1_clk_src_sel().bits()),
            )
            .field(
                "reg_uart1_clk_en",
                &format_args!("{}", self.reg_uart1_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI_CLK_CTRL111_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_uart0_sclk_div_num(&mut self) -> REG_UART0_SCLK_DIV_NUM_W<PERI_CLK_CTRL111_SPEC> {
        REG_UART0_SCLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_uart0_sclk_div_numerator(
        &mut self,
    ) -> REG_UART0_SCLK_DIV_NUMERATOR_W<PERI_CLK_CTRL111_SPEC> {
        REG_UART0_SCLK_DIV_NUMERATOR_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_uart0_sclk_div_denominator(
        &mut self,
    ) -> REG_UART0_SCLK_DIV_DENOMINATOR_W<PERI_CLK_CTRL111_SPEC> {
        REG_UART0_SCLK_DIV_DENOMINATOR_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_uart1_clk_src_sel(&mut self) -> REG_UART1_CLK_SRC_SEL_W<PERI_CLK_CTRL111_SPEC> {
        REG_UART1_CLK_SRC_SEL_W::new(self, 24)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_uart1_clk_en(&mut self) -> REG_UART1_CLK_EN_W<PERI_CLK_CTRL111_SPEC> {
        REG_UART1_CLK_EN_W::new(self, 26)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_ctrl111::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_ctrl111::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL111_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL111_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl111::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL111_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl111::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL111_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL111 to value 0x0400_0000"]
impl crate::Resettable for PERI_CLK_CTRL111_SPEC {
    const RESET_VALUE: u32 = 0x0400_0000;
}
