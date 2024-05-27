///Register `SPI_SMEM_DIN_MODE` reader
pub type R = crate::R<SPI_SMEM_DIN_MODE_SPEC>;
///Register `SPI_SMEM_DIN_MODE` writer
pub type W = crate::W<SPI_SMEM_DIN_MODE_SPEC>;
///Field `SPI_SMEM_DIN0_MODE` reader - SPI_D input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type SPI_SMEM_DIN0_MODE_R = crate::FieldReader;
///Field `SPI_SMEM_DIN0_MODE` writer - SPI_D input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type SPI_SMEM_DIN0_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPI_SMEM_DIN1_MODE` reader - SPI_Q input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type SPI_SMEM_DIN1_MODE_R = crate::FieldReader;
///Field `SPI_SMEM_DIN1_MODE` writer - SPI_Q input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type SPI_SMEM_DIN1_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPI_SMEM_DIN2_MODE` reader - SPI_WP input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type SPI_SMEM_DIN2_MODE_R = crate::FieldReader;
///Field `SPI_SMEM_DIN2_MODE` writer - SPI_WP input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type SPI_SMEM_DIN2_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPI_SMEM_DIN3_MODE` reader - SPI_HD input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type SPI_SMEM_DIN3_MODE_R = crate::FieldReader;
///Field `SPI_SMEM_DIN3_MODE` writer - SPI_HD input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type SPI_SMEM_DIN3_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPI_SMEM_DIN4_MODE` reader - SPI_IO4 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type SPI_SMEM_DIN4_MODE_R = crate::FieldReader;
///Field `SPI_SMEM_DIN4_MODE` writer - SPI_IO4 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type SPI_SMEM_DIN4_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPI_SMEM_DIN5_MODE` reader - SPI_IO5 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type SPI_SMEM_DIN5_MODE_R = crate::FieldReader;
///Field `SPI_SMEM_DIN5_MODE` writer - SPI_IO5 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type SPI_SMEM_DIN5_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPI_SMEM_DIN6_MODE` reader - SPI_IO6 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type SPI_SMEM_DIN6_MODE_R = crate::FieldReader;
///Field `SPI_SMEM_DIN6_MODE` writer - SPI_IO6 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type SPI_SMEM_DIN6_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPI_SMEM_DIN7_MODE` reader - SPI_IO7 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type SPI_SMEM_DIN7_MODE_R = crate::FieldReader;
///Field `SPI_SMEM_DIN7_MODE` writer - SPI_IO7 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type SPI_SMEM_DIN7_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPI_SMEM_DINS_MODE` reader - SPI_DQS input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DINS_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type SPI_SMEM_DINS_MODE_R = crate::FieldReader;
///Field `SPI_SMEM_DINS_MODE` writer - SPI_DQS input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DINS_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type SPI_SMEM_DINS_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - SPI_D input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    pub fn spi_smem_din0_mode(&self) -> SPI_SMEM_DIN0_MODE_R {
        SPI_SMEM_DIN0_MODE_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - SPI_Q input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    pub fn spi_smem_din1_mode(&self) -> SPI_SMEM_DIN1_MODE_R {
        SPI_SMEM_DIN1_MODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - SPI_WP input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    pub fn spi_smem_din2_mode(&self) -> SPI_SMEM_DIN2_MODE_R {
        SPI_SMEM_DIN2_MODE_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - SPI_HD input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    pub fn spi_smem_din3_mode(&self) -> SPI_SMEM_DIN3_MODE_R {
        SPI_SMEM_DIN3_MODE_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - SPI_IO4 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    pub fn spi_smem_din4_mode(&self) -> SPI_SMEM_DIN4_MODE_R {
        SPI_SMEM_DIN4_MODE_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - SPI_IO5 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    pub fn spi_smem_din5_mode(&self) -> SPI_SMEM_DIN5_MODE_R {
        SPI_SMEM_DIN5_MODE_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - SPI_IO6 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    pub fn spi_smem_din6_mode(&self) -> SPI_SMEM_DIN6_MODE_R {
        SPI_SMEM_DIN6_MODE_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - SPI_IO7 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    pub fn spi_smem_din7_mode(&self) -> SPI_SMEM_DIN7_MODE_R {
        SPI_SMEM_DIN7_MODE_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:26 - SPI_DQS input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DINS_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    pub fn spi_smem_dins_mode(&self) -> SPI_SMEM_DINS_MODE_R {
        SPI_SMEM_DINS_MODE_R::new(((self.bits >> 24) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SMEM_DIN_MODE")
            .field("spi_smem_din0_mode", &self.spi_smem_din0_mode())
            .field("spi_smem_din1_mode", &self.spi_smem_din1_mode())
            .field("spi_smem_din2_mode", &self.spi_smem_din2_mode())
            .field("spi_smem_din3_mode", &self.spi_smem_din3_mode())
            .field("spi_smem_din4_mode", &self.spi_smem_din4_mode())
            .field("spi_smem_din5_mode", &self.spi_smem_din5_mode())
            .field("spi_smem_din6_mode", &self.spi_smem_din6_mode())
            .field("spi_smem_din7_mode", &self.spi_smem_din7_mode())
            .field("spi_smem_dins_mode", &self.spi_smem_dins_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SPI_D input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_din0_mode(&mut self) -> SPI_SMEM_DIN0_MODE_W<SPI_SMEM_DIN_MODE_SPEC> {
        SPI_SMEM_DIN0_MODE_W::new(self, 0)
    }
    ///Bits 3:5 - SPI_Q input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_din1_mode(&mut self) -> SPI_SMEM_DIN1_MODE_W<SPI_SMEM_DIN_MODE_SPEC> {
        SPI_SMEM_DIN1_MODE_W::new(self, 3)
    }
    ///Bits 6:8 - SPI_WP input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_din2_mode(&mut self) -> SPI_SMEM_DIN2_MODE_W<SPI_SMEM_DIN_MODE_SPEC> {
        SPI_SMEM_DIN2_MODE_W::new(self, 6)
    }
    ///Bits 9:11 - SPI_HD input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_din3_mode(&mut self) -> SPI_SMEM_DIN3_MODE_W<SPI_SMEM_DIN_MODE_SPEC> {
        SPI_SMEM_DIN3_MODE_W::new(self, 9)
    }
    ///Bits 12:14 - SPI_IO4 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_din4_mode(&mut self) -> SPI_SMEM_DIN4_MODE_W<SPI_SMEM_DIN_MODE_SPEC> {
        SPI_SMEM_DIN4_MODE_W::new(self, 12)
    }
    ///Bits 15:17 - SPI_IO5 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_din5_mode(&mut self) -> SPI_SMEM_DIN5_MODE_W<SPI_SMEM_DIN_MODE_SPEC> {
        SPI_SMEM_DIN5_MODE_W::new(self, 15)
    }
    ///Bits 18:20 - SPI_IO6 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_din6_mode(&mut self) -> SPI_SMEM_DIN6_MODE_W<SPI_SMEM_DIN_MODE_SPEC> {
        SPI_SMEM_DIN6_MODE_W::new(self, 18)
    }
    ///Bits 21:23 - SPI_IO7 input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_din7_mode(&mut self) -> SPI_SMEM_DIN7_MODE_W<SPI_SMEM_DIN_MODE_SPEC> {
        SPI_SMEM_DIN7_MODE_W::new(self, 21)
    }
    ///Bits 24:26 - SPI_DQS input delay mode. 0: No delay. 1: Delay for (SPI_SMEM_DINS_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_SMEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_dins_mode(&mut self) -> SPI_SMEM_DINS_MODE_W<SPI_SMEM_DIN_MODE_SPEC> {
        SPI_SMEM_DINS_MODE_W::new(self, 24)
    }
}
/**MSPI input timing delay mode control register when accesses to Ext_RAM.

You can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_din_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_din_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPI_SMEM_DIN_MODE_SPEC;
impl crate::RegisterSpec for SPI_SMEM_DIN_MODE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`spi_smem_din_mode::R`](R) reader structure
impl crate::Readable for SPI_SMEM_DIN_MODE_SPEC {}
///`write(|w| ..)` method takes [`spi_smem_din_mode::W`](W) writer structure
impl crate::Writable for SPI_SMEM_DIN_MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SPI_SMEM_DIN_MODE to value 0
impl crate::Resettable for SPI_SMEM_DIN_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
