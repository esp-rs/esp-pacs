#[doc = "Register `CLOCK` reader"]
pub type R = crate::R<CLOCK_SPEC>;
#[doc = "Register `CLOCK` writer"]
pub type W = crate::W<CLOCK_SPEC>;
#[doc = "Field `CLKCNT_L` reader - In master transfer, this field must be equal to SPI_CLKCNT_N. In slave mode, it must be 0. Can be configured in CONF state."]
pub type CLKCNT_L_R = crate::FieldReader;
#[doc = "Field `CLKCNT_L` writer - In master transfer, this field must be equal to SPI_CLKCNT_N. In slave mode, it must be 0. Can be configured in CONF state."]
pub type CLKCNT_L_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CLKCNT_H` reader - Configures the duty cycle of SPI_CLK (high level) in master transfer. It's recommended to configure this value to floor((SPI_CLKCNT_N + 1)/2 - 1). floor() here is to round a number down, e.g., floor(2.2) = 2. In slave mode, it must be 0. \\\\ Can be configured in CONF state."]
pub type CLKCNT_H_R = crate::FieldReader;
#[doc = "Field `CLKCNT_H` writer - Configures the duty cycle of SPI_CLK (high level) in master transfer. It's recommended to configure this value to floor((SPI_CLKCNT_N + 1)/2 - 1). floor() here is to round a number down, e.g., floor(2.2) = 2. In slave mode, it must be 0. \\\\ Can be configured in CONF state."]
pub type CLKCNT_H_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CLKCNT_N` reader - Configures the divider of SPI_CLK in master transfer. SPI_CLK frequency is 12_{\\textrm{apb_clk}}$/(SPI_CLKDIV_PRE + 1)/(SPI_CLKCNT_N + 1). \\\\ Can be configured in CONF state."]
pub type CLKCNT_N_R = crate::FieldReader;
#[doc = "Field `CLKCNT_N` writer - Configures the divider of SPI_CLK in master transfer. SPI_CLK frequency is 12_{\\textrm{apb_clk}}$/(SPI_CLKDIV_PRE + 1)/(SPI_CLKCNT_N + 1). \\\\ Can be configured in CONF state."]
pub type CLKCNT_N_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CLKDIV_PRE` reader - Configures the pre-divider of SPI_CLK in master transfer. Can be configured in CONF state."]
pub type CLKDIV_PRE_R = crate::FieldReader;
#[doc = "Field `CLKDIV_PRE` writer - Configures the pre-divider of SPI_CLK in master transfer. Can be configured in CONF state."]
pub type CLKDIV_PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLK_EQU_SYSCLK` reader - Configures whether or not the SPI_CLK is equal to APB_CLK in master transfer.\\\\ 0: SPI_CLK is divided from APB_CLK.\\\\ 1: SPI_CLK is eqaul to APB_CLK.\\\\ Can be configured in CONF state."]
pub type CLK_EQU_SYSCLK_R = crate::BitReader;
#[doc = "Field `CLK_EQU_SYSCLK` writer - Configures whether or not the SPI_CLK is equal to APB_CLK in master transfer.\\\\ 0: SPI_CLK is divided from APB_CLK.\\\\ 1: SPI_CLK is eqaul to APB_CLK.\\\\ Can be configured in CONF state."]
pub type CLK_EQU_SYSCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - In master transfer, this field must be equal to SPI_CLKCNT_N. In slave mode, it must be 0. Can be configured in CONF state."]
    #[inline(always)]
    pub fn clkcnt_l(&self) -> CLKCNT_L_R {
        CLKCNT_L_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Configures the duty cycle of SPI_CLK (high level) in master transfer. It's recommended to configure this value to floor((SPI_CLKCNT_N + 1)/2 - 1). floor() here is to round a number down, e.g., floor(2.2) = 2. In slave mode, it must be 0. \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn clkcnt_h(&self) -> CLKCNT_H_R {
        CLKCNT_H_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Configures the divider of SPI_CLK in master transfer. SPI_CLK frequency is 12_{\\textrm{apb_clk}}$/(SPI_CLKDIV_PRE + 1)/(SPI_CLKCNT_N + 1). \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn clkcnt_n(&self) -> CLKCNT_N_R {
        CLKCNT_N_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:21 - Configures the pre-divider of SPI_CLK in master transfer. Can be configured in CONF state."]
    #[inline(always)]
    pub fn clkdiv_pre(&self) -> CLKDIV_PRE_R {
        CLKDIV_PRE_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Configures whether or not the SPI_CLK is equal to APB_CLK in master transfer.\\\\ 0: SPI_CLK is divided from APB_CLK.\\\\ 1: SPI_CLK is eqaul to APB_CLK.\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn clk_equ_sysclk(&self) -> CLK_EQU_SYSCLK_R {
        CLK_EQU_SYSCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLOCK")
            .field("clkcnt_l", &self.clkcnt_l())
            .field("clkcnt_h", &self.clkcnt_h())
            .field("clkcnt_n", &self.clkcnt_n())
            .field("clkdiv_pre", &self.clkdiv_pre())
            .field("clk_equ_sysclk", &self.clk_equ_sysclk())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - In master transfer, this field must be equal to SPI_CLKCNT_N. In slave mode, it must be 0. Can be configured in CONF state."]
    #[inline(always)]
    pub fn clkcnt_l(&mut self) -> CLKCNT_L_W<'_, CLOCK_SPEC> {
        CLKCNT_L_W::new(self, 0)
    }
    #[doc = "Bits 6:11 - Configures the duty cycle of SPI_CLK (high level) in master transfer. It's recommended to configure this value to floor((SPI_CLKCNT_N + 1)/2 - 1). floor() here is to round a number down, e.g., floor(2.2) = 2. In slave mode, it must be 0. \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn clkcnt_h(&mut self) -> CLKCNT_H_W<'_, CLOCK_SPEC> {
        CLKCNT_H_W::new(self, 6)
    }
    #[doc = "Bits 12:17 - Configures the divider of SPI_CLK in master transfer. SPI_CLK frequency is 12_{\\textrm{apb_clk}}$/(SPI_CLKDIV_PRE + 1)/(SPI_CLKCNT_N + 1). \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn clkcnt_n(&mut self) -> CLKCNT_N_W<'_, CLOCK_SPEC> {
        CLKCNT_N_W::new(self, 12)
    }
    #[doc = "Bits 18:21 - Configures the pre-divider of SPI_CLK in master transfer. Can be configured in CONF state."]
    #[inline(always)]
    pub fn clkdiv_pre(&mut self) -> CLKDIV_PRE_W<'_, CLOCK_SPEC> {
        CLKDIV_PRE_W::new(self, 18)
    }
    #[doc = "Bit 31 - Configures whether or not the SPI_CLK is equal to APB_CLK in master transfer.\\\\ 0: SPI_CLK is divided from APB_CLK.\\\\ 1: SPI_CLK is eqaul to APB_CLK.\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn clk_equ_sysclk(&mut self) -> CLK_EQU_SYSCLK_W<'_, CLOCK_SPEC> {
        CLK_EQU_SYSCLK_W::new(self, 31)
    }
}
#[doc = "SPI clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLOCK_SPEC;
impl crate::RegisterSpec for CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock::R`](R) reader structure"]
impl crate::Readable for CLOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clock::W`](W) writer structure"]
impl crate::Writable for CLOCK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLOCK to value 0x8000_3043"]
impl crate::Resettable for CLOCK_SPEC {
    const RESET_VALUE: u32 = 0x8000_3043;
}
