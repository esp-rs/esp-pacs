#[doc = "Register `MEM_MISC` reader"]
pub type R = crate::R<MEM_MISC_SPEC>;
#[doc = "Register `MEM_MISC` writer"]
pub type W = crate::W<MEM_MISC_SPEC>;
#[doc = "Field `MEM_FSUB_PIN` reader - For SPI0, flash is connected to SUBPINs."]
pub type MEM_FSUB_PIN_R = crate::BitReader;
#[doc = "Field `MEM_SSUB_PIN` reader - For SPI0, sram is connected to SUBPINs."]
pub type MEM_SSUB_PIN_R = crate::BitReader;
#[doc = "Field `MEM_CK_IDLE_EDGE` reader - 1: SPI_CLK line is high when idle 0: spi clk line is low when idle"]
pub type MEM_CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `MEM_CK_IDLE_EDGE` writer - 1: SPI_CLK line is high when idle 0: spi clk line is low when idle"]
pub type MEM_CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_CS_KEEP_ACTIVE` reader - SPI_CS line keep low when the bit is set."]
pub type MEM_CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `MEM_CS_KEEP_ACTIVE` writer - SPI_CS line keep low when the bit is set."]
pub type MEM_CS_KEEP_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - For SPI0, flash is connected to SUBPINs."]
    #[inline(always)]
    pub fn mem_fsub_pin(&self) -> MEM_FSUB_PIN_R {
        MEM_FSUB_PIN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - For SPI0, sram is connected to SUBPINs."]
    #[inline(always)]
    pub fn mem_ssub_pin(&self) -> MEM_SSUB_PIN_R {
        MEM_SSUB_PIN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: SPI_CLK line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn mem_ck_idle_edge(&self) -> MEM_CK_IDLE_EDGE_R {
        MEM_CK_IDLE_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI_CS line keep low when the bit is set."]
    #[inline(always)]
    pub fn mem_cs_keep_active(&self) -> MEM_CS_KEEP_ACTIVE_R {
        MEM_CS_KEEP_ACTIVE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_MISC")
            .field("mem_fsub_pin", &self.mem_fsub_pin())
            .field("mem_ssub_pin", &self.mem_ssub_pin())
            .field("mem_ck_idle_edge", &self.mem_ck_idle_edge())
            .field("mem_cs_keep_active", &self.mem_cs_keep_active())
            .finish()
    }
}
impl W {
    #[doc = "Bit 9 - 1: SPI_CLK line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn mem_ck_idle_edge(&mut self) -> MEM_CK_IDLE_EDGE_W<MEM_MISC_SPEC> {
        MEM_CK_IDLE_EDGE_W::new(self, 9)
    }
    #[doc = "Bit 10 - SPI_CS line keep low when the bit is set."]
    #[inline(always)]
    pub fn mem_cs_keep_active(&mut self) -> MEM_CS_KEEP_ACTIVE_W<MEM_MISC_SPEC> {
        MEM_CS_KEEP_ACTIVE_W::new(self, 10)
    }
}
#[doc = "SPI0 misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_MISC_SPEC;
impl crate::RegisterSpec for MEM_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_misc::R`](R) reader structure"]
impl crate::Readable for MEM_MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_misc::W`](W) writer structure"]
impl crate::Writable for MEM_MISC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_MISC to value 0"]
impl crate::Resettable for MEM_MISC_SPEC {}
