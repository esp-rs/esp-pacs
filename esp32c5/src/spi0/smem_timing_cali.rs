#[doc = "Register `SMEM_TIMING_CALI` reader"]
pub type R = crate::R<SMEM_TIMING_CALI_SPEC>;
#[doc = "Register `SMEM_TIMING_CALI` writer"]
pub type W = crate::W<SMEM_TIMING_CALI_SPEC>;
#[doc = "Field `SMEM_TIMING_CLK_ENA` reader - For sram, the bit is used to enable timing adjust clock for all reading operations."]
pub type SMEM_TIMING_CLK_ENA_R = crate::BitReader;
#[doc = "Field `SMEM_TIMING_CLK_ENA` writer - For sram, the bit is used to enable timing adjust clock for all reading operations."]
pub type SMEM_TIMING_CLK_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_TIMING_CALI` reader - For sram, the bit is used to enable timing auto-calibration for all reading operations."]
pub type SMEM_TIMING_CALI_R = crate::BitReader;
#[doc = "Field `SMEM_TIMING_CALI` writer - For sram, the bit is used to enable timing auto-calibration for all reading operations."]
pub type SMEM_TIMING_CALI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_EXTRA_DUMMY_CYCLELEN` reader - For sram, add extra dummy spi clock cycle length for spi clock calibration."]
pub type SMEM_EXTRA_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `SMEM_EXTRA_DUMMY_CYCLELEN` writer - For sram, add extra dummy spi clock cycle length for spi clock calibration."]
pub type SMEM_EXTRA_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMEM_DLL_TIMING_CALI` reader - Set this bit to enable DLL for timing calibration in DDR mode when accessed to EXT_RAM."]
pub type SMEM_DLL_TIMING_CALI_R = crate::BitReader;
#[doc = "Field `SMEM_DLL_TIMING_CALI` writer - Set this bit to enable DLL for timing calibration in DDR mode when accessed to EXT_RAM."]
pub type SMEM_DLL_TIMING_CALI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_DQS0_270_SEL` reader - Set these bits to delay dqs signal & invert delayed signal for DLL timing adjust. 2'd0: 0.5ns, 2'd1: 1.0ns, 2'd2: 1.5ns 2'd3: 2.0ns."]
pub type SMEM_DQS0_270_SEL_R = crate::FieldReader;
#[doc = "Field `SMEM_DQS0_270_SEL` writer - Set these bits to delay dqs signal & invert delayed signal for DLL timing adjust. 2'd0: 0.5ns, 2'd1: 1.0ns, 2'd2: 1.5ns 2'd3: 2.0ns."]
pub type SMEM_DQS0_270_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SMEM_DQS0_90_SEL` reader - Set these bits to delay dqs signal for DLL timing adjust. 2'd0: 0.5ns, 2'd1: 1.0ns, 2'd2: 1.5ns 2'd3: 2.0ns."]
pub type SMEM_DQS0_90_SEL_R = crate::FieldReader;
#[doc = "Field `SMEM_DQS0_90_SEL` writer - Set these bits to delay dqs signal for DLL timing adjust. 2'd0: 0.5ns, 2'd1: 1.0ns, 2'd2: 1.5ns 2'd3: 2.0ns."]
pub type SMEM_DQS0_90_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - For sram, the bit is used to enable timing adjust clock for all reading operations."]
    #[inline(always)]
    pub fn smem_timing_clk_ena(&self) -> SMEM_TIMING_CLK_ENA_R {
        SMEM_TIMING_CLK_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For sram, the bit is used to enable timing auto-calibration for all reading operations."]
    #[inline(always)]
    pub fn smem_timing_cali(&self) -> SMEM_TIMING_CALI_R {
        SMEM_TIMING_CALI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - For sram, add extra dummy spi clock cycle length for spi clock calibration."]
    #[inline(always)]
    pub fn smem_extra_dummy_cyclelen(&self) -> SMEM_EXTRA_DUMMY_CYCLELEN_R {
        SMEM_EXTRA_DUMMY_CYCLELEN_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Set this bit to enable DLL for timing calibration in DDR mode when accessed to EXT_RAM."]
    #[inline(always)]
    pub fn smem_dll_timing_cali(&self) -> SMEM_DLL_TIMING_CALI_R {
        SMEM_DLL_TIMING_CALI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Set these bits to delay dqs signal & invert delayed signal for DLL timing adjust. 2'd0: 0.5ns, 2'd1: 1.0ns, 2'd2: 1.5ns 2'd3: 2.0ns."]
    #[inline(always)]
    pub fn smem_dqs0_270_sel(&self) -> SMEM_DQS0_270_SEL_R {
        SMEM_DQS0_270_SEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - Set these bits to delay dqs signal for DLL timing adjust. 2'd0: 0.5ns, 2'd1: 1.0ns, 2'd2: 1.5ns 2'd3: 2.0ns."]
    #[inline(always)]
    pub fn smem_dqs0_90_sel(&self) -> SMEM_DQS0_90_SEL_R {
        SMEM_DQS0_90_SEL_R::new(((self.bits >> 9) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMEM_TIMING_CALI")
            .field("smem_timing_clk_ena", &self.smem_timing_clk_ena())
            .field("smem_timing_cali", &self.smem_timing_cali())
            .field(
                "smem_extra_dummy_cyclelen",
                &self.smem_extra_dummy_cyclelen(),
            )
            .field("smem_dll_timing_cali", &self.smem_dll_timing_cali())
            .field("smem_dqs0_270_sel", &self.smem_dqs0_270_sel())
            .field("smem_dqs0_90_sel", &self.smem_dqs0_90_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - For sram, the bit is used to enable timing adjust clock for all reading operations."]
    #[inline(always)]
    pub fn smem_timing_clk_ena(&mut self) -> SMEM_TIMING_CLK_ENA_W<SMEM_TIMING_CALI_SPEC> {
        SMEM_TIMING_CLK_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - For sram, the bit is used to enable timing auto-calibration for all reading operations."]
    #[inline(always)]
    pub fn smem_timing_cali(&mut self) -> SMEM_TIMING_CALI_W<SMEM_TIMING_CALI_SPEC> {
        SMEM_TIMING_CALI_W::new(self, 1)
    }
    #[doc = "Bits 2:4 - For sram, add extra dummy spi clock cycle length for spi clock calibration."]
    #[inline(always)]
    pub fn smem_extra_dummy_cyclelen(
        &mut self,
    ) -> SMEM_EXTRA_DUMMY_CYCLELEN_W<SMEM_TIMING_CALI_SPEC> {
        SMEM_EXTRA_DUMMY_CYCLELEN_W::new(self, 2)
    }
    #[doc = "Bit 5 - Set this bit to enable DLL for timing calibration in DDR mode when accessed to EXT_RAM."]
    #[inline(always)]
    pub fn smem_dll_timing_cali(&mut self) -> SMEM_DLL_TIMING_CALI_W<SMEM_TIMING_CALI_SPEC> {
        SMEM_DLL_TIMING_CALI_W::new(self, 5)
    }
    #[doc = "Bits 7:8 - Set these bits to delay dqs signal & invert delayed signal for DLL timing adjust. 2'd0: 0.5ns, 2'd1: 1.0ns, 2'd2: 1.5ns 2'd3: 2.0ns."]
    #[inline(always)]
    pub fn smem_dqs0_270_sel(&mut self) -> SMEM_DQS0_270_SEL_W<SMEM_TIMING_CALI_SPEC> {
        SMEM_DQS0_270_SEL_W::new(self, 7)
    }
    #[doc = "Bits 9:10 - Set these bits to delay dqs signal for DLL timing adjust. 2'd0: 0.5ns, 2'd1: 1.0ns, 2'd2: 1.5ns 2'd3: 2.0ns."]
    #[inline(always)]
    pub fn smem_dqs0_90_sel(&mut self) -> SMEM_DQS0_90_SEL_W<SMEM_TIMING_CALI_SPEC> {
        SMEM_DQS0_90_SEL_W::new(self, 9)
    }
}
#[doc = "MSPI external RAM timing calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_timing_cali::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_timing_cali::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMEM_TIMING_CALI_SPEC;
impl crate::RegisterSpec for SMEM_TIMING_CALI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smem_timing_cali::R`](R) reader structure"]
impl crate::Readable for SMEM_TIMING_CALI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smem_timing_cali::W`](W) writer structure"]
impl crate::Writable for SMEM_TIMING_CALI_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMEM_TIMING_CALI to value 0x0281"]
impl crate::Resettable for SMEM_TIMING_CALI_SPEC {
    const RESET_VALUE: u32 = 0x0281;
}
