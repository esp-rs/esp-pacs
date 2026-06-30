#[doc = "Register `SPI_MEM_MISC` reader"]
pub type R = crate::R<SPI_MEM_MISC_SPEC>;
#[doc = "Register `SPI_MEM_MISC` writer"]
pub type W = crate::W<SPI_MEM_MISC_SPEC>;
#[doc = "Field `SPI_MEM_DQ_OE_CTRL` reader - For SPI BUS IO, APB ctrl IO DQ OE func.1: enable 0: disable."]
pub type SPI_MEM_DQ_OE_CTRL_R = crate::BitReader;
#[doc = "Field `SPI_MEM_DQ_OE_CTRL` writer - For SPI BUS IO, APB ctrl IO DQ OE func.1: enable 0: disable."]
pub type SPI_MEM_DQ_OE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_CK_OE_CTRL` reader - For SPI BUS IO, APB ctrl IO CK OE func.1: enable 0: disable."]
pub type SPI_MEM_CK_OE_CTRL_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CK_OE_CTRL` writer - For SPI BUS IO, APB ctrl IO CK OE func.1: enable 0: disable."]
pub type SPI_MEM_CK_OE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_CS_OE_CTRL` reader - For SPI BUS IO, APB ctrl IO CS OE func.1: enable 0: disable."]
pub type SPI_MEM_CS_OE_CTRL_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CS_OE_CTRL` writer - For SPI BUS IO, APB ctrl IO CS OE func.1: enable 0: disable."]
pub type SPI_MEM_CS_OE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FSUB_PIN` reader - For SPI0, flash is connected to SUBPINs."]
pub type SPI_MEM_FSUB_PIN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SSUB_PIN` reader - For SPI0, sram is connected to SUBPINs."]
pub type SPI_MEM_SSUB_PIN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CK_IDLE_EDGE` reader - 1: SPI_CLK line is high when idle 0: spi clk line is low when idle"]
pub type SPI_MEM_CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CK_IDLE_EDGE` writer - 1: SPI_CLK line is high when idle 0: spi clk line is low when idle"]
pub type SPI_MEM_CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_CS_KEEP_ACTIVE` reader - SPI_CS line keep low when the bit is set."]
pub type SPI_MEM_CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CS_KEEP_ACTIVE` writer - SPI_CS line keep low when the bit is set."]
pub type SPI_MEM_CS_KEEP_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - For SPI BUS IO, APB ctrl IO DQ OE func.1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_dq_oe_ctrl(&self) -> SPI_MEM_DQ_OE_CTRL_R {
        SPI_MEM_DQ_OE_CTRL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - For SPI BUS IO, APB ctrl IO CK OE func.1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_ck_oe_ctrl(&self) -> SPI_MEM_CK_OE_CTRL_R {
        SPI_MEM_CK_OE_CTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - For SPI BUS IO, APB ctrl IO CS OE func.1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_cs_oe_ctrl(&self) -> SPI_MEM_CS_OE_CTRL_R {
        SPI_MEM_CS_OE_CTRL_R::new(((self.bits >> 6) & 1) != 0)
    }
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
            .field("spi_mem_dq_oe_ctrl", &self.spi_mem_dq_oe_ctrl())
            .field("spi_mem_ck_oe_ctrl", &self.spi_mem_ck_oe_ctrl())
            .field("spi_mem_cs_oe_ctrl", &self.spi_mem_cs_oe_ctrl())
            .field("spi_mem_fsub_pin", &self.spi_mem_fsub_pin())
            .field("spi_mem_ssub_pin", &self.spi_mem_ssub_pin())
            .field("spi_mem_ck_idle_edge", &self.spi_mem_ck_idle_edge())
            .field("spi_mem_cs_keep_active", &self.spi_mem_cs_keep_active())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - For SPI BUS IO, APB ctrl IO DQ OE func.1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_dq_oe_ctrl(&mut self) -> SPI_MEM_DQ_OE_CTRL_W<'_, SPI_MEM_MISC_SPEC> {
        SPI_MEM_DQ_OE_CTRL_W::new(self, 4)
    }
    #[doc = "Bit 5 - For SPI BUS IO, APB ctrl IO CK OE func.1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_ck_oe_ctrl(&mut self) -> SPI_MEM_CK_OE_CTRL_W<'_, SPI_MEM_MISC_SPEC> {
        SPI_MEM_CK_OE_CTRL_W::new(self, 5)
    }
    #[doc = "Bit 6 - For SPI BUS IO, APB ctrl IO CS OE func.1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_cs_oe_ctrl(&mut self) -> SPI_MEM_CS_OE_CTRL_W<'_, SPI_MEM_MISC_SPEC> {
        SPI_MEM_CS_OE_CTRL_W::new(self, 6)
    }
    #[doc = "Bit 9 - 1: SPI_CLK line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn spi_mem_ck_idle_edge(&mut self) -> SPI_MEM_CK_IDLE_EDGE_W<'_, SPI_MEM_MISC_SPEC> {
        SPI_MEM_CK_IDLE_EDGE_W::new(self, 9)
    }
    #[doc = "Bit 10 - SPI_CS line keep low when the bit is set."]
    #[inline(always)]
    pub fn spi_mem_cs_keep_active(&mut self) -> SPI_MEM_CS_KEEP_ACTIVE_W<'_, SPI_MEM_MISC_SPEC> {
        SPI_MEM_CS_KEEP_ACTIVE_W::new(self, 10)
    }
}
#[doc = "SPI0 misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_mem_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_MISC_SPEC;
impl crate::RegisterSpec for SPI_MEM_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_misc::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_misc::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_MISC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_MEM_MISC to value 0x70"]
impl crate::Resettable for SPI_MEM_MISC_SPEC {
    const RESET_VALUE: u32 = 0x70;
}
