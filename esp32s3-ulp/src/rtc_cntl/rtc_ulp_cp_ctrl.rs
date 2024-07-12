#[doc = "Register `RTC_ULP_CP_CTRL` reader"]
pub type R = crate::R<RTC_ULP_CP_CTRL_SPEC>;
#[doc = "Register `RTC_ULP_CP_CTRL` writer"]
pub type W = crate::W<RTC_ULP_CP_CTRL_SPEC>;
#[doc = "Field `ULP_CP_MEM_ADDR_INIT` reader - No public"]
pub type ULP_CP_MEM_ADDR_INIT_R = crate::FieldReader<u16>;
#[doc = "Field `ULP_CP_MEM_ADDR_INIT` writer - No public"]
pub type ULP_CP_MEM_ADDR_INIT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `ULP_CP_MEM_ADDR_SIZE` reader - No public"]
pub type ULP_CP_MEM_ADDR_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `ULP_CP_MEM_ADDR_SIZE` writer - No public"]
pub type ULP_CP_MEM_ADDR_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `ULP_CP_MEM_OFFST_CLR` writer - No public"]
pub type ULP_CP_MEM_OFFST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_CP_CLK_FO` reader - ulp coprocessor clk force on"]
pub type ULP_CP_CLK_FO_R = crate::BitReader;
#[doc = "Field `ULP_CP_CLK_FO` writer - ulp coprocessor clk force on"]
pub type ULP_CP_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_CP_RESET` reader - ulp coprocessor clk software reset"]
pub type ULP_CP_RESET_R = crate::BitReader;
#[doc = "Field `ULP_CP_RESET` writer - ulp coprocessor clk software reset"]
pub type ULP_CP_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_CP_FORCE_START_TOP` reader - 1: ULP-coprocessor is started by SW"]
pub type ULP_CP_FORCE_START_TOP_R = crate::BitReader;
#[doc = "Field `ULP_CP_FORCE_START_TOP` writer - 1: ULP-coprocessor is started by SW"]
pub type ULP_CP_FORCE_START_TOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_CP_START_TOP` reader - Write 1 to start ULP-coprocessor"]
pub type ULP_CP_START_TOP_R = crate::BitReader;
#[doc = "Field `ULP_CP_START_TOP` writer - Write 1 to start ULP-coprocessor"]
pub type ULP_CP_START_TOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - No public"]
    #[inline(always)]
    pub fn ulp_cp_mem_addr_init(&self) -> ULP_CP_MEM_ADDR_INIT_R {
        ULP_CP_MEM_ADDR_INIT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - No public"]
    #[inline(always)]
    pub fn ulp_cp_mem_addr_size(&self) -> ULP_CP_MEM_ADDR_SIZE_R {
        ULP_CP_MEM_ADDR_SIZE_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - ulp coprocessor clk force on"]
    #[inline(always)]
    pub fn ulp_cp_clk_fo(&self) -> ULP_CP_CLK_FO_R {
        ULP_CP_CLK_FO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ulp coprocessor clk software reset"]
    #[inline(always)]
    pub fn ulp_cp_reset(&self) -> ULP_CP_RESET_R {
        ULP_CP_RESET_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: ULP-coprocessor is started by SW"]
    #[inline(always)]
    pub fn ulp_cp_force_start_top(&self) -> ULP_CP_FORCE_START_TOP_R {
        ULP_CP_FORCE_START_TOP_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Write 1 to start ULP-coprocessor"]
    #[inline(always)]
    pub fn ulp_cp_start_top(&self) -> ULP_CP_START_TOP_R {
        ULP_CP_START_TOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_ULP_CP_CTRL")
            .field("ulp_cp_mem_addr_init", &self.ulp_cp_mem_addr_init())
            .field("ulp_cp_mem_addr_size", &self.ulp_cp_mem_addr_size())
            .field("ulp_cp_clk_fo", &self.ulp_cp_clk_fo())
            .field("ulp_cp_reset", &self.ulp_cp_reset())
            .field("ulp_cp_force_start_top", &self.ulp_cp_force_start_top())
            .field("ulp_cp_start_top", &self.ulp_cp_start_top())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - No public"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_mem_addr_init(&mut self) -> ULP_CP_MEM_ADDR_INIT_W<RTC_ULP_CP_CTRL_SPEC> {
        ULP_CP_MEM_ADDR_INIT_W::new(self, 0)
    }
    #[doc = "Bits 11:21 - No public"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_mem_addr_size(&mut self) -> ULP_CP_MEM_ADDR_SIZE_W<RTC_ULP_CP_CTRL_SPEC> {
        ULP_CP_MEM_ADDR_SIZE_W::new(self, 11)
    }
    #[doc = "Bit 22 - No public"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_mem_offst_clr(&mut self) -> ULP_CP_MEM_OFFST_CLR_W<RTC_ULP_CP_CTRL_SPEC> {
        ULP_CP_MEM_OFFST_CLR_W::new(self, 22)
    }
    #[doc = "Bit 28 - ulp coprocessor clk force on"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_clk_fo(&mut self) -> ULP_CP_CLK_FO_W<RTC_ULP_CP_CTRL_SPEC> {
        ULP_CP_CLK_FO_W::new(self, 28)
    }
    #[doc = "Bit 29 - ulp coprocessor clk software reset"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_reset(&mut self) -> ULP_CP_RESET_W<RTC_ULP_CP_CTRL_SPEC> {
        ULP_CP_RESET_W::new(self, 29)
    }
    #[doc = "Bit 30 - 1: ULP-coprocessor is started by SW"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_force_start_top(&mut self) -> ULP_CP_FORCE_START_TOP_W<RTC_ULP_CP_CTRL_SPEC> {
        ULP_CP_FORCE_START_TOP_W::new(self, 30)
    }
    #[doc = "Bit 31 - Write 1 to start ULP-coprocessor"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_start_top(&mut self) -> ULP_CP_START_TOP_W<RTC_ULP_CP_CTRL_SPEC> {
        ULP_CP_START_TOP_W::new(self, 31)
    }
}
#[doc = "configure ulp\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_ulp_cp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_ulp_cp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_ULP_CP_CTRL_SPEC;
impl crate::RegisterSpec for RTC_ULP_CP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_ulp_cp_ctrl::R`](R) reader structure"]
impl crate::Readable for RTC_ULP_CP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_ulp_cp_ctrl::W`](W) writer structure"]
impl crate::Writable for RTC_ULP_CP_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_ULP_CP_CTRL to value 0x0010_0200"]
impl crate::Resettable for RTC_ULP_CP_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0010_0200;
}
