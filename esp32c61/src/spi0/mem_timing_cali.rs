#[doc = "Register `MEM_TIMING_CALI` reader"]
pub type R = crate::R<MEM_TIMING_CALI_SPEC>;
#[doc = "Register `MEM_TIMING_CALI` writer"]
pub type W = crate::W<MEM_TIMING_CALI_SPEC>;
#[doc = "Field `MEM_TIMING_CLK_ENA` reader - The bit is used to enable timing adjust clock for all reading operations."]
pub type MEM_TIMING_CLK_ENA_R = crate::BitReader;
#[doc = "Field `MEM_TIMING_CLK_ENA` writer - The bit is used to enable timing adjust clock for all reading operations."]
pub type MEM_TIMING_CLK_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_TIMING_CALI` reader - The bit is used to enable timing auto-calibration for all reading operations."]
pub type MEM_TIMING_CALI_R = crate::BitReader;
#[doc = "Field `MEM_TIMING_CALI` writer - The bit is used to enable timing auto-calibration for all reading operations."]
pub type MEM_TIMING_CALI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_EXTRA_DUMMY_CYCLELEN` reader - add extra dummy spi clock cycle length for spi clock calibration."]
pub type MEM_EXTRA_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `MEM_EXTRA_DUMMY_CYCLELEN` writer - add extra dummy spi clock cycle length for spi clock calibration."]
pub type MEM_EXTRA_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MEM_DLL_TIMING_CALI` reader - Set this bit to enable DLL for timing calibration in DDR mode when accessed to flash."]
pub type MEM_DLL_TIMING_CALI_R = crate::BitReader;
#[doc = "Field `MEM_DLL_TIMING_CALI` writer - Set this bit to enable DLL for timing calibration in DDR mode when accessed to flash."]
pub type MEM_DLL_TIMING_CALI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDATE` writer - Set this bit to update delay mode, delay num and extra dummy in MSPI."]
pub type UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable timing adjust clock for all reading operations."]
    #[inline(always)]
    pub fn mem_timing_clk_ena(&self) -> MEM_TIMING_CLK_ENA_R {
        MEM_TIMING_CLK_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable timing auto-calibration for all reading operations."]
    #[inline(always)]
    pub fn mem_timing_cali(&self) -> MEM_TIMING_CALI_R {
        MEM_TIMING_CALI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - add extra dummy spi clock cycle length for spi clock calibration."]
    #[inline(always)]
    pub fn mem_extra_dummy_cyclelen(&self) -> MEM_EXTRA_DUMMY_CYCLELEN_R {
        MEM_EXTRA_DUMMY_CYCLELEN_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Set this bit to enable DLL for timing calibration in DDR mode when accessed to flash."]
    #[inline(always)]
    pub fn mem_dll_timing_cali(&self) -> MEM_DLL_TIMING_CALI_R {
        MEM_DLL_TIMING_CALI_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_TIMING_CALI")
            .field("mem_timing_clk_ena", &self.mem_timing_clk_ena())
            .field("mem_timing_cali", &self.mem_timing_cali())
            .field("mem_extra_dummy_cyclelen", &self.mem_extra_dummy_cyclelen())
            .field("mem_dll_timing_cali", &self.mem_dll_timing_cali())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable timing adjust clock for all reading operations."]
    #[inline(always)]
    pub fn mem_timing_clk_ena(&mut self) -> MEM_TIMING_CLK_ENA_W<MEM_TIMING_CALI_SPEC> {
        MEM_TIMING_CLK_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to enable timing auto-calibration for all reading operations."]
    #[inline(always)]
    pub fn mem_timing_cali(&mut self) -> MEM_TIMING_CALI_W<MEM_TIMING_CALI_SPEC> {
        MEM_TIMING_CALI_W::new(self, 1)
    }
    #[doc = "Bits 2:4 - add extra dummy spi clock cycle length for spi clock calibration."]
    #[inline(always)]
    pub fn mem_extra_dummy_cyclelen(&mut self) -> MEM_EXTRA_DUMMY_CYCLELEN_W<MEM_TIMING_CALI_SPEC> {
        MEM_EXTRA_DUMMY_CYCLELEN_W::new(self, 2)
    }
    #[doc = "Bit 5 - Set this bit to enable DLL for timing calibration in DDR mode when accessed to flash."]
    #[inline(always)]
    pub fn mem_dll_timing_cali(&mut self) -> MEM_DLL_TIMING_CALI_W<MEM_TIMING_CALI_SPEC> {
        MEM_DLL_TIMING_CALI_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to update delay mode, delay num and extra dummy in MSPI."]
    #[inline(always)]
    pub fn update(&mut self) -> UPDATE_W<MEM_TIMING_CALI_SPEC> {
        UPDATE_W::new(self, 6)
    }
}
#[doc = "SPI0 flash timing calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_timing_cali::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_timing_cali::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_TIMING_CALI_SPEC;
impl crate::RegisterSpec for MEM_TIMING_CALI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_timing_cali::R`](R) reader structure"]
impl crate::Readable for MEM_TIMING_CALI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_timing_cali::W`](W) writer structure"]
impl crate::Writable for MEM_TIMING_CALI_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_TIMING_CALI to value 0x01"]
impl crate::Resettable for MEM_TIMING_CALI_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
