#[doc = "Register `SPI_MEM_MISC` reader"]
pub type R = crate::R<SPI_MEM_MISC_SPEC>;
#[doc = "Register `SPI_MEM_MISC` writer"]
pub type W = crate::W<SPI_MEM_MISC_SPEC>;
#[doc = "Field `SPI_MEM_FSUB_PIN` reader - For SPI0, flash is connected to SUBPINs."]
pub type SPI_MEM_FSUB_PIN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FSUB_PIN` writer - For SPI0, flash is connected to SUBPINs."]
pub type SPI_MEM_FSUB_PIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_SSUB_PIN` reader - For SPI0, sram is connected to SUBPINs."]
pub type SPI_MEM_SSUB_PIN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SSUB_PIN` writer - For SPI0, sram is connected to SUBPINs."]
pub type SPI_MEM_SSUB_PIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_CK_IDLE_EDGE` reader - 1: SPI_CLK line is high when idle 0: spi clk line is low when idle"]
pub type SPI_MEM_CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CK_IDLE_EDGE` writer - 1: SPI_CLK line is high when idle 0: spi clk line is low when idle"]
pub type SPI_MEM_CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_CS_KEEP_ACTIVE` reader - SPI_CS line keep low when the bit is set."]
pub type SPI_MEM_CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CS_KEEP_ACTIVE` writer - SPI_CS line keep low when the bit is set."]
pub type SPI_MEM_CS_KEEP_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - For SPI0, flash is connected to SUBPINs."]
    #[inline(always)]
    pub fn spi_mem_fsub_pin(&self) -> SPI_MEM_FSUB_PIN_R {
        SPI_MEM_FSUB_PIN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - For SPI0, sram is connected to SUBPINs."]
    #[inline(always)]
    pub fn spi_mem_ssub_pin(&self) -> SPI_MEM_SSUB_PIN_R {
        SPI_MEM_SSUB_PIN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: SPI_CLK line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn spi_mem_ck_idle_edge(&self) -> SPI_MEM_CK_IDLE_EDGE_R {
        SPI_MEM_CK_IDLE_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI_CS line keep low when the bit is set."]
    #[inline(always)]
    pub fn spi_mem_cs_keep_active(&self) -> SPI_MEM_CS_KEEP_ACTIVE_R {
        SPI_MEM_CS_KEEP_ACTIVE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_MISC")
            .field(
                "spi_mem_fsub_pin",
                &format_args!("{}", self.spi_mem_fsub_pin().bit()),
            )
            .field(
                "spi_mem_ssub_pin",
                &format_args!("{}", self.spi_mem_ssub_pin().bit()),
            )
            .field(
                "spi_mem_ck_idle_edge",
                &format_args!("{}", self.spi_mem_ck_idle_edge().bit()),
            )
            .field(
                "spi_mem_cs_keep_active",
                &format_args!("{}", self.spi_mem_cs_keep_active().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_MISC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 7 - For SPI0, flash is connected to SUBPINs."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fsub_pin(&mut self) -> SPI_MEM_FSUB_PIN_W<SPI_MEM_MISC_SPEC> {
        SPI_MEM_FSUB_PIN_W::new(self, 7)
    }
    #[doc = "Bit 8 - For SPI0, sram is connected to SUBPINs."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_ssub_pin(&mut self) -> SPI_MEM_SSUB_PIN_W<SPI_MEM_MISC_SPEC> {
        SPI_MEM_SSUB_PIN_W::new(self, 8)
    }
    #[doc = "Bit 9 - 1: SPI_CLK line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_ck_idle_edge(&mut self) -> SPI_MEM_CK_IDLE_EDGE_W<SPI_MEM_MISC_SPEC> {
        SPI_MEM_CK_IDLE_EDGE_W::new(self, 9)
    }
    #[doc = "Bit 10 - SPI_CS line keep low when the bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_cs_keep_active(&mut self) -> SPI_MEM_CS_KEEP_ACTIVE_W<SPI_MEM_MISC_SPEC> {
        SPI_MEM_CS_KEEP_ACTIVE_W::new(self, 10)
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
#[doc = "SPI0 misc register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_misc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_misc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_MISC_SPEC;
impl crate::RegisterSpec for SPI_MEM_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_misc::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_misc::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_MISC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MEM_MISC to value 0"]
impl crate::Resettable for SPI_MEM_MISC_SPEC {
    const RESET_VALUE: u32 = 0;
}
