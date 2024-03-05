#[doc = "Register `INF_ST` reader"]
pub type R = crate::R<INF_ST_SPEC>;
#[doc = "Register `INF_ST` writer"]
pub type W = crate::W<INF_ST_SPEC>;
#[doc = "Field `SDIO20_MODE` reader - *******Description***********"]
pub type SDIO20_MODE_R = crate::FieldReader;
#[doc = "Field `SDIO_NEG_SAMP` reader - *******Description***********"]
pub type SDIO_NEG_SAMP_R = crate::FieldReader;
#[doc = "Field `SDIO_QUICK_IN` reader - *******Description***********"]
pub type SDIO_QUICK_IN_R = crate::FieldReader;
#[doc = "Field `DLL_ON_SW` reader - dll is controlled by software"]
pub type DLL_ON_SW_R = crate::BitReader;
#[doc = "Field `DLL_ON_SW` writer - dll is controlled by software"]
pub type DLL_ON_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLL_ON` reader - Software dll on"]
pub type DLL_ON_R = crate::BitReader;
#[doc = "Field `DLL_ON` writer - Software dll on"]
pub type DLL_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MODE_SW` reader - dll clock mode is controlled by software"]
pub type CLK_MODE_SW_R = crate::BitReader;
#[doc = "Field `CLK_MODE_SW` writer - dll clock mode is controlled by software"]
pub type CLK_MODE_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MODE` reader - Software set clock mode"]
pub type CLK_MODE_R = crate::FieldReader;
#[doc = "Field `CLK_MODE` writer - Software set clock mode"]
pub type CLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4 - *******Description***********"]
    #[inline(always)]
    pub fn sdio20_mode(&self) -> SDIO20_MODE_R {
        SDIO20_MODE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - *******Description***********"]
    #[inline(always)]
    pub fn sdio_neg_samp(&self) -> SDIO_NEG_SAMP_R {
        SDIO_NEG_SAMP_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - *******Description***********"]
    #[inline(always)]
    pub fn sdio_quick_in(&self) -> SDIO_QUICK_IN_R {
        SDIO_QUICK_IN_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - dll is controlled by software"]
    #[inline(always)]
    pub fn dll_on_sw(&self) -> DLL_ON_SW_R {
        DLL_ON_SW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Software dll on"]
    #[inline(always)]
    pub fn dll_on(&self) -> DLL_ON_R {
        DLL_ON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - dll clock mode is controlled by software"]
    #[inline(always)]
    pub fn clk_mode_sw(&self) -> CLK_MODE_SW_R {
        CLK_MODE_SW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Software set clock mode"]
    #[inline(always)]
    pub fn clk_mode(&self) -> CLK_MODE_R {
        CLK_MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INF_ST")
            .field(
                "sdio20_mode",
                &format_args!("{}", self.sdio20_mode().bits()),
            )
            .field(
                "sdio_neg_samp",
                &format_args!("{}", self.sdio_neg_samp().bits()),
            )
            .field(
                "sdio_quick_in",
                &format_args!("{}", self.sdio_quick_in().bits()),
            )
            .field("dll_on_sw", &format_args!("{}", self.dll_on_sw().bit()))
            .field("dll_on", &format_args!("{}", self.dll_on().bit()))
            .field("clk_mode_sw", &format_args!("{}", self.clk_mode_sw().bit()))
            .field("clk_mode", &format_args!("{}", self.clk_mode().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INF_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 15 - dll is controlled by software"]
    #[inline(always)]
    #[must_use]
    pub fn dll_on_sw(&mut self) -> DLL_ON_SW_W<INF_ST_SPEC> {
        DLL_ON_SW_W::new(self, 15)
    }
    #[doc = "Bit 16 - Software dll on"]
    #[inline(always)]
    #[must_use]
    pub fn dll_on(&mut self) -> DLL_ON_W<INF_ST_SPEC> {
        DLL_ON_W::new(self, 16)
    }
    #[doc = "Bit 17 - dll clock mode is controlled by software"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mode_sw(&mut self) -> CLK_MODE_SW_W<INF_ST_SPEC> {
        CLK_MODE_SW_W::new(self, 17)
    }
    #[doc = "Bits 18:19 - Software set clock mode"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mode(&mut self) -> CLK_MODE_W<INF_ST_SPEC> {
        CLK_MODE_W::new(self, 18)
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inf_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inf_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INF_ST_SPEC;
impl crate::RegisterSpec for INF_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inf_st::R`](R) reader structure"]
impl crate::Readable for INF_ST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inf_st::W`](W) writer structure"]
impl crate::Writable for INF_ST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INF_ST to value 0"]
impl crate::Resettable for INF_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
