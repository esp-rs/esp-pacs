#[doc = "Register `SPI_MEM_MISC` reader"]
pub type R = crate::R<SPI_MEM_MISC_SPEC>;
#[doc = "Register `SPI_MEM_MISC` writer"]
pub type W = crate::W<SPI_MEM_MISC_SPEC>;
#[doc = "Field `SPI_MEM_CS0_DIS` reader - SPI_CS0 pin enable, 1: disable SPI_CS0, 0: SPI_CS0 pin is active to select SPI device, such as flash, external RAM and so on."]
pub type SPI_MEM_CS0_DIS_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CS0_DIS` writer - SPI_CS0 pin enable, 1: disable SPI_CS0, 0: SPI_CS0 pin is active to select SPI device, such as flash, external RAM and so on."]
pub type SPI_MEM_CS0_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_CS1_DIS` reader - SPI_CS1 pin enable, 1: disable SPI_CS1, 0: SPI_CS1 pin is active to select SPI device, such as flash, external RAM and so on."]
pub type SPI_MEM_CS1_DIS_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CS1_DIS` writer - SPI_CS1 pin enable, 1: disable SPI_CS1, 0: SPI_CS1 pin is active to select SPI device, such as flash, external RAM and so on."]
pub type SPI_MEM_CS1_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_CK_IDLE_EDGE` reader - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type SPI_MEM_CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CK_IDLE_EDGE` writer - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type SPI_MEM_CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_CS_KEEP_ACTIVE` reader - spi cs line keep low when the bit is set."]
pub type SPI_MEM_CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CS_KEEP_ACTIVE` writer - spi cs line keep low when the bit is set."]
pub type SPI_MEM_CS_KEEP_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI_CS0 pin enable, 1: disable SPI_CS0, 0: SPI_CS0 pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    pub fn spi_mem_cs0_dis(&self) -> SPI_MEM_CS0_DIS_R {
        SPI_MEM_CS0_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI_CS1 pin enable, 1: disable SPI_CS1, 0: SPI_CS1 pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    pub fn spi_mem_cs1_dis(&self) -> SPI_MEM_CS1_DIS_R {
        SPI_MEM_CS1_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn spi_mem_ck_idle_edge(&self) -> SPI_MEM_CK_IDLE_EDGE_R {
        SPI_MEM_CK_IDLE_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    pub fn spi_mem_cs_keep_active(&self) -> SPI_MEM_CS_KEEP_ACTIVE_R {
        SPI_MEM_CS_KEEP_ACTIVE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_MISC")
            .field("spi_mem_cs0_dis", &self.spi_mem_cs0_dis())
            .field("spi_mem_cs1_dis", &self.spi_mem_cs1_dis())
            .field("spi_mem_ck_idle_edge", &self.spi_mem_ck_idle_edge())
            .field("spi_mem_cs_keep_active", &self.spi_mem_cs_keep_active())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SPI_CS0 pin enable, 1: disable SPI_CS0, 0: SPI_CS0 pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    pub fn spi_mem_cs0_dis(&mut self) -> SPI_MEM_CS0_DIS_W<'_, SPI_MEM_MISC_SPEC> {
        SPI_MEM_CS0_DIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - SPI_CS1 pin enable, 1: disable SPI_CS1, 0: SPI_CS1 pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    pub fn spi_mem_cs1_dis(&mut self) -> SPI_MEM_CS1_DIS_W<'_, SPI_MEM_MISC_SPEC> {
        SPI_MEM_CS1_DIS_W::new(self, 1)
    }
    #[doc = "Bit 9 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn spi_mem_ck_idle_edge(&mut self) -> SPI_MEM_CK_IDLE_EDGE_W<'_, SPI_MEM_MISC_SPEC> {
        SPI_MEM_CK_IDLE_EDGE_W::new(self, 9)
    }
    #[doc = "Bit 10 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    pub fn spi_mem_cs_keep_active(&mut self) -> SPI_MEM_CS_KEEP_ACTIVE_W<'_, SPI_MEM_MISC_SPEC> {
        SPI_MEM_CS_KEEP_ACTIVE_W::new(self, 10)
    }
}
#[doc = "SPI1 misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_mem_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets SPI_MEM_MISC to value 0x02"]
impl crate::Resettable for SPI_MEM_MISC_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
