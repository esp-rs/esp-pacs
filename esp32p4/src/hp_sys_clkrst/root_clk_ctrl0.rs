#[doc = "Register `ROOT_CLK_CTRL0` reader"]
pub type R = crate::R<ROOT_CLK_CTRL0_SPEC>;
#[doc = "Register `ROOT_CLK_CTRL0` writer"]
pub type W = crate::W<ROOT_CLK_CTRL0_SPEC>;
#[doc = "Field `CPUICM_DELAY_NUM` reader - Reserved"]
pub type CPUICM_DELAY_NUM_R = crate::FieldReader;
#[doc = "Field `CPUICM_DELAY_NUM` writer - Reserved"]
pub type CPUICM_DELAY_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SOC_CLK_DIV_UPDATE` reader - Reserved"]
pub type SOC_CLK_DIV_UPDATE_R = crate::BitReader;
#[doc = "Field `SOC_CLK_DIV_UPDATE` writer - Reserved"]
pub type SOC_CLK_DIV_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_CLK_DIV_NUM` reader - Reserved"]
pub type CPU_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `CPU_CLK_DIV_NUM` writer - Reserved"]
pub type CPU_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CPU_CLK_DIV_NUMERATOR` reader - Reserved"]
pub type CPU_CLK_DIV_NUMERATOR_R = crate::FieldReader;
#[doc = "Field `CPU_CLK_DIV_NUMERATOR` writer - Reserved"]
pub type CPU_CLK_DIV_NUMERATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CPU_CLK_DIV_DENOMINATOR` reader - Reserved"]
pub type CPU_CLK_DIV_DENOMINATOR_R = crate::FieldReader;
#[doc = "Field `CPU_CLK_DIV_DENOMINATOR` writer - Reserved"]
pub type CPU_CLK_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Reserved"]
    #[inline(always)]
    pub fn cpuicm_delay_num(&self) -> CPUICM_DELAY_NUM_R {
        CPUICM_DELAY_NUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn soc_clk_div_update(&self) -> SOC_CLK_DIV_UPDATE_R {
        SOC_CLK_DIV_UPDATE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:12 - Reserved"]
    #[inline(always)]
    pub fn cpu_clk_div_num(&self) -> CPU_CLK_DIV_NUM_R {
        CPU_CLK_DIV_NUM_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bits 13:20 - Reserved"]
    #[inline(always)]
    pub fn cpu_clk_div_numerator(&self) -> CPU_CLK_DIV_NUMERATOR_R {
        CPU_CLK_DIV_NUMERATOR_R::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 21:28 - Reserved"]
    #[inline(always)]
    pub fn cpu_clk_div_denominator(&self) -> CPU_CLK_DIV_DENOMINATOR_R {
        CPU_CLK_DIV_DENOMINATOR_R::new(((self.bits >> 21) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROOT_CLK_CTRL0")
            .field("cpuicm_delay_num", &self.cpuicm_delay_num())
            .field("soc_clk_div_update", &self.soc_clk_div_update())
            .field("cpu_clk_div_num", &self.cpu_clk_div_num())
            .field("cpu_clk_div_numerator", &self.cpu_clk_div_numerator())
            .field("cpu_clk_div_denominator", &self.cpu_clk_div_denominator())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Reserved"]
    #[inline(always)]
    pub fn cpuicm_delay_num(&mut self) -> CPUICM_DELAY_NUM_W<ROOT_CLK_CTRL0_SPEC> {
        CPUICM_DELAY_NUM_W::new(self, 0)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn soc_clk_div_update(&mut self) -> SOC_CLK_DIV_UPDATE_W<ROOT_CLK_CTRL0_SPEC> {
        SOC_CLK_DIV_UPDATE_W::new(self, 4)
    }
    #[doc = "Bits 5:12 - Reserved"]
    #[inline(always)]
    pub fn cpu_clk_div_num(&mut self) -> CPU_CLK_DIV_NUM_W<ROOT_CLK_CTRL0_SPEC> {
        CPU_CLK_DIV_NUM_W::new(self, 5)
    }
    #[doc = "Bits 13:20 - Reserved"]
    #[inline(always)]
    pub fn cpu_clk_div_numerator(&mut self) -> CPU_CLK_DIV_NUMERATOR_W<ROOT_CLK_CTRL0_SPEC> {
        CPU_CLK_DIV_NUMERATOR_W::new(self, 13)
    }
    #[doc = "Bits 21:28 - Reserved"]
    #[inline(always)]
    pub fn cpu_clk_div_denominator(&mut self) -> CPU_CLK_DIV_DENOMINATOR_W<ROOT_CLK_CTRL0_SPEC> {
        CPU_CLK_DIV_DENOMINATOR_W::new(self, 21)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`root_clk_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`root_clk_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROOT_CLK_CTRL0_SPEC;
impl crate::RegisterSpec for ROOT_CLK_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`root_clk_ctrl0::R`](R) reader structure"]
impl crate::Readable for ROOT_CLK_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`root_clk_ctrl0::W`](W) writer structure"]
impl crate::Writable for ROOT_CLK_CTRL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROOT_CLK_CTRL0 to value 0"]
impl crate::Resettable for ROOT_CLK_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
