#[doc = "Register `SPI_MEM_MISC` reader"]
pub struct R(crate::R<SPI_MEM_MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_MISC` writer"]
pub struct W(crate::W<SPI_MEM_MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPI_MEM_MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_CS0_DIS` reader - SPI_CS0 pin enable, 1: disable SPI_CS0, 0: SPI_CS0 pin is active to select SPI device, such as flash, external RAM and so on."]
pub type SPI_MEM_CS0_DIS_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CS0_DIS` writer - SPI_CS0 pin enable, 1: disable SPI_CS0, 0: SPI_CS0 pin is active to select SPI device, such as flash, external RAM and so on."]
pub type SPI_MEM_CS0_DIS_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_MISC_SPEC, O>;
#[doc = "Field `SPI_MEM_CS1_DIS` reader - SPI_CS1 pin enable, 1: disable SPI_CS1, 0: SPI_CS1 pin is active to select SPI device, such as flash, external RAM and so on."]
pub type SPI_MEM_CS1_DIS_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CS1_DIS` writer - SPI_CS1 pin enable, 1: disable SPI_CS1, 0: SPI_CS1 pin is active to select SPI device, such as flash, external RAM and so on."]
pub type SPI_MEM_CS1_DIS_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_MISC_SPEC, O>;
#[doc = "Field `SPI_MEM_CK_IDLE_EDGE` reader - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type SPI_MEM_CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CK_IDLE_EDGE` writer - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type SPI_MEM_CK_IDLE_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_MISC_SPEC, O>;
#[doc = "Field `SPI_MEM_CS_KEEP_ACTIVE` reader - spi cs line keep low when the bit is set."]
pub type SPI_MEM_CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CS_KEEP_ACTIVE` writer - spi cs line keep low when the bit is set."]
pub type SPI_MEM_CS_KEEP_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_MISC_SPEC, O>;
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
            .field(
                "spi_mem_cs0_dis",
                &format_args!("{}", self.spi_mem_cs0_dis().bit()),
            )
            .field(
                "spi_mem_cs1_dis",
                &format_args!("{}", self.spi_mem_cs1_dis().bit()),
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
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - SPI_CS0 pin enable, 1: disable SPI_CS0, 0: SPI_CS0 pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_cs0_dis(&mut self) -> SPI_MEM_CS0_DIS_W<0> {
        SPI_MEM_CS0_DIS_W::new(self)
    }
    #[doc = "Bit 1 - SPI_CS1 pin enable, 1: disable SPI_CS1, 0: SPI_CS1 pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_cs1_dis(&mut self) -> SPI_MEM_CS1_DIS_W<1> {
        SPI_MEM_CS1_DIS_W::new(self)
    }
    #[doc = "Bit 9 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_ck_idle_edge(&mut self) -> SPI_MEM_CK_IDLE_EDGE_W<9> {
        SPI_MEM_CK_IDLE_EDGE_W::new(self)
    }
    #[doc = "Bit 10 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_cs_keep_active(&mut self) -> SPI_MEM_CS_KEEP_ACTIVE_W<10> {
        SPI_MEM_CS_KEEP_ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 misc register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_misc](index.html) module"]
pub struct SPI_MEM_MISC_SPEC;
impl crate::RegisterSpec for SPI_MEM_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_misc::R](R) reader structure"]
impl crate::Readable for SPI_MEM_MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_misc::W](W) writer structure"]
impl crate::Writable for SPI_MEM_MISC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_MISC to value 0x02"]
impl crate::Resettable for SPI_MEM_MISC_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
