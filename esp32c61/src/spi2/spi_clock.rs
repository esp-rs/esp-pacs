#[doc = "Register `SPI_CLOCK` reader"]
pub type R = crate::R<SPI_CLOCK_SPEC>;
#[doc = "Register `SPI_CLOCK` writer"]
pub type W = crate::W<SPI_CLOCK_SPEC>;
#[doc = "Field `SPI_CLKCNT_L` reader - In master transfer, this field must be equal to SPI_CLKCNT_N. In slave mode, it must be 0. Can be configured in CONF state."]
pub type SPI_CLKCNT_L_R = crate::FieldReader;
#[doc = "Field `SPI_CLKCNT_L` writer - In master transfer, this field must be equal to SPI_CLKCNT_N. In slave mode, it must be 0. Can be configured in CONF state."]
pub type SPI_CLKCNT_L_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SPI_CLKCNT_H` reader - Configures the duty cycle of SPI_CLK (high level) in master transfer. It's recommended to configure this value to floor((SPI_CLKCNT_N + 1)/2 - 1). floor() here is to round a number down, e.g., floor(2.2) = 2. In slave mode, it must be 0. \\\\ Can be configured in CONF state."]
pub type SPI_CLKCNT_H_R = crate::FieldReader;
#[doc = "Field `SPI_CLKCNT_H` writer - Configures the duty cycle of SPI_CLK (high level) in master transfer. It's recommended to configure this value to floor((SPI_CLKCNT_N + 1)/2 - 1). floor() here is to round a number down, e.g., floor(2.2) = 2. In slave mode, it must be 0. \\\\ Can be configured in CONF state."]
pub type SPI_CLKCNT_H_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SPI_CLKCNT_N` reader - Configures the divider of SPI_CLK in master transfer. SPI_CLK frequency is 12_{\\textrm{apb_clk}}$/(SPI_CLKDIV_PRE + 1)/(SPI_CLKCNT_N + 1). \\\\ Can be configured in CONF state."]
pub type SPI_CLKCNT_N_R = crate::FieldReader;
#[doc = "Field `SPI_CLKCNT_N` writer - Configures the divider of SPI_CLK in master transfer. SPI_CLK frequency is 12_{\\textrm{apb_clk}}$/(SPI_CLKDIV_PRE + 1)/(SPI_CLKCNT_N + 1). \\\\ Can be configured in CONF state."]
pub type SPI_CLKCNT_N_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SPI_CLKDIV_PRE` reader - Configures the pre-divider of SPI_CLK in master transfer. Can be configured in CONF state."]
pub type SPI_CLKDIV_PRE_R = crate::FieldReader;
#[doc = "Field `SPI_CLKDIV_PRE` writer - Configures the pre-divider of SPI_CLK in master transfer. Can be configured in CONF state."]
pub type SPI_CLKDIV_PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SPI_CLK_EDGE_SEL` reader - Configures use standard clock sampling edge or delay the sampling edge by half a cycle in master transfer.\\\\ 0: clock sampling edge is delayed by half a cycle.\\\\ 1: clock sampling edge is standard.\\\\ Can be configured in CONF state."]
pub type SPI_CLK_EDGE_SEL_R = crate::BitReader;
#[doc = "Field `SPI_CLK_EDGE_SEL` writer - Configures use standard clock sampling edge or delay the sampling edge by half a cycle in master transfer.\\\\ 0: clock sampling edge is delayed by half a cycle.\\\\ 1: clock sampling edge is standard.\\\\ Can be configured in CONF state."]
pub type SPI_CLK_EDGE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CLK_EQU_SYSCLK` reader - Configures whether or not the SPI_CLK is equal to APB_CLK in master transfer.\\\\ 0: SPI_CLK is divided from APB_CLK.\\\\ 1: SPI_CLK is eqaul to APB_CLK.\\\\ Can be configured in CONF state."]
pub type SPI_CLK_EQU_SYSCLK_R = crate::BitReader;
#[doc = "Field `SPI_CLK_EQU_SYSCLK` writer - Configures whether or not the SPI_CLK is equal to APB_CLK in master transfer.\\\\ 0: SPI_CLK is divided from APB_CLK.\\\\ 1: SPI_CLK is eqaul to APB_CLK.\\\\ Can be configured in CONF state."]
pub type SPI_CLK_EQU_SYSCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - In master transfer, this field must be equal to SPI_CLKCNT_N. In slave mode, it must be 0. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clkcnt_l(&self) -> SPI_CLKCNT_L_R {
        SPI_CLKCNT_L_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Configures the duty cycle of SPI_CLK (high level) in master transfer. It's recommended to configure this value to floor((SPI_CLKCNT_N + 1)/2 - 1). floor() here is to round a number down, e.g., floor(2.2) = 2. In slave mode, it must be 0. \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clkcnt_h(&self) -> SPI_CLKCNT_H_R {
        SPI_CLKCNT_H_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Configures the divider of SPI_CLK in master transfer. SPI_CLK frequency is 12_{\\textrm{apb_clk}}$/(SPI_CLKDIV_PRE + 1)/(SPI_CLKCNT_N + 1). \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clkcnt_n(&self) -> SPI_CLKCNT_N_R {
        SPI_CLKCNT_N_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:21 - Configures the pre-divider of SPI_CLK in master transfer. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clkdiv_pre(&self) -> SPI_CLKDIV_PRE_R {
        SPI_CLKDIV_PRE_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Configures use standard clock sampling edge or delay the sampling edge by half a cycle in master transfer.\\\\ 0: clock sampling edge is delayed by half a cycle.\\\\ 1: clock sampling edge is standard.\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clk_edge_sel(&self) -> SPI_CLK_EDGE_SEL_R {
        SPI_CLK_EDGE_SEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Configures whether or not the SPI_CLK is equal to APB_CLK in master transfer.\\\\ 0: SPI_CLK is divided from APB_CLK.\\\\ 1: SPI_CLK is eqaul to APB_CLK.\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clk_equ_sysclk(&self) -> SPI_CLK_EQU_SYSCLK_R {
        SPI_CLK_EQU_SYSCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CLOCK")
            .field("spi_clkcnt_l", &self.spi_clkcnt_l())
            .field("spi_clkcnt_h", &self.spi_clkcnt_h())
            .field("spi_clkcnt_n", &self.spi_clkcnt_n())
            .field("spi_clkdiv_pre", &self.spi_clkdiv_pre())
            .field("spi_clk_edge_sel", &self.spi_clk_edge_sel())
            .field("spi_clk_equ_sysclk", &self.spi_clk_equ_sysclk())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - In master transfer, this field must be equal to SPI_CLKCNT_N. In slave mode, it must be 0. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clkcnt_l(&mut self) -> SPI_CLKCNT_L_W<SPI_CLOCK_SPEC> {
        SPI_CLKCNT_L_W::new(self, 0)
    }
    #[doc = "Bits 6:11 - Configures the duty cycle of SPI_CLK (high level) in master transfer. It's recommended to configure this value to floor((SPI_CLKCNT_N + 1)/2 - 1). floor() here is to round a number down, e.g., floor(2.2) = 2. In slave mode, it must be 0. \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clkcnt_h(&mut self) -> SPI_CLKCNT_H_W<SPI_CLOCK_SPEC> {
        SPI_CLKCNT_H_W::new(self, 6)
    }
    #[doc = "Bits 12:17 - Configures the divider of SPI_CLK in master transfer. SPI_CLK frequency is 12_{\\textrm{apb_clk}}$/(SPI_CLKDIV_PRE + 1)/(SPI_CLKCNT_N + 1). \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clkcnt_n(&mut self) -> SPI_CLKCNT_N_W<SPI_CLOCK_SPEC> {
        SPI_CLKCNT_N_W::new(self, 12)
    }
    #[doc = "Bits 18:21 - Configures the pre-divider of SPI_CLK in master transfer. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clkdiv_pre(&mut self) -> SPI_CLKDIV_PRE_W<SPI_CLOCK_SPEC> {
        SPI_CLKDIV_PRE_W::new(self, 18)
    }
    #[doc = "Bit 30 - Configures use standard clock sampling edge or delay the sampling edge by half a cycle in master transfer.\\\\ 0: clock sampling edge is delayed by half a cycle.\\\\ 1: clock sampling edge is standard.\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clk_edge_sel(&mut self) -> SPI_CLK_EDGE_SEL_W<SPI_CLOCK_SPEC> {
        SPI_CLK_EDGE_SEL_W::new(self, 30)
    }
    #[doc = "Bit 31 - Configures whether or not the SPI_CLK is equal to APB_CLK in master transfer.\\\\ 0: SPI_CLK is divided from APB_CLK.\\\\ 1: SPI_CLK is eqaul to APB_CLK.\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clk_equ_sysclk(&mut self) -> SPI_CLK_EQU_SYSCLK_W<SPI_CLOCK_SPEC> {
        SPI_CLK_EQU_SYSCLK_W::new(self, 31)
    }
}
#[doc = "SPI clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_clock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_clock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_CLOCK_SPEC;
impl crate::RegisterSpec for SPI_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_clock::R`](R) reader structure"]
impl crate::Readable for SPI_CLOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_clock::W`](W) writer structure"]
impl crate::Writable for SPI_CLOCK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_CLOCK to value 0x8000_3043"]
impl crate::Resettable for SPI_CLOCK_SPEC {
    const RESET_VALUE: u32 = 0x8000_3043;
}
