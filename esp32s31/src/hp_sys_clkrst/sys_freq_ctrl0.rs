#[doc = "Register `SYS_FREQ_CTRL0` reader"]
pub type R = crate::R<SYS_FREQ_CTRL0_SPEC>;
#[doc = "Register `SYS_FREQ_CTRL0` writer"]
pub type W = crate::W<SYS_FREQ_CTRL0_SPEC>;
#[doc = "Field `SYS_CLK_DIV_NUM` reader - need_des"]
pub type SYS_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `SYS_CLK_DIV_NUM` writer - need_des"]
pub type SYS_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SYS_CLK_DIV_NUMERATOR` reader - need_des"]
pub type SYS_CLK_DIV_NUMERATOR_R = crate::FieldReader;
#[doc = "Field `SYS_CLK_DIV_NUMERATOR` writer - need_des"]
pub type SYS_CLK_DIV_NUMERATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SYS_CLK_DIV_DENOMINATOR` reader - need_des"]
pub type SYS_CLK_DIV_DENOMINATOR_R = crate::FieldReader;
#[doc = "Field `SYS_CLK_DIV_DENOMINATOR` writer - need_des"]
pub type SYS_CLK_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn sys_clk_div_num(&self) -> SYS_CLK_DIV_NUM_R {
        SYS_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - need_des"]
    #[inline(always)]
    pub fn sys_clk_div_numerator(&self) -> SYS_CLK_DIV_NUMERATOR_R {
        SYS_CLK_DIV_NUMERATOR_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - need_des"]
    #[inline(always)]
    pub fn sys_clk_div_denominator(&self) -> SYS_CLK_DIV_DENOMINATOR_R {
        SYS_CLK_DIV_DENOMINATOR_R::new(((self.bits >> 11) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_FREQ_CTRL0")
            .field("sys_clk_div_num", &self.sys_clk_div_num())
            .field("sys_clk_div_numerator", &self.sys_clk_div_numerator())
            .field("sys_clk_div_denominator", &self.sys_clk_div_denominator())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn sys_clk_div_num(&mut self) -> SYS_CLK_DIV_NUM_W<'_, SYS_FREQ_CTRL0_SPEC> {
        SYS_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - need_des"]
    #[inline(always)]
    pub fn sys_clk_div_numerator(&mut self) -> SYS_CLK_DIV_NUMERATOR_W<'_, SYS_FREQ_CTRL0_SPEC> {
        SYS_CLK_DIV_NUMERATOR_W::new(self, 8)
    }
    #[doc = "Bits 11:13 - need_des"]
    #[inline(always)]
    pub fn sys_clk_div_denominator(
        &mut self,
    ) -> SYS_CLK_DIV_DENOMINATOR_W<'_, SYS_FREQ_CTRL0_SPEC> {
        SYS_CLK_DIV_DENOMINATOR_W::new(self, 11)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_freq_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_freq_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_FREQ_CTRL0_SPEC;
impl crate::RegisterSpec for SYS_FREQ_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_freq_ctrl0::R`](R) reader structure"]
impl crate::Readable for SYS_FREQ_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_freq_ctrl0::W`](W) writer structure"]
impl crate::Writable for SYS_FREQ_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_FREQ_CTRL0 to value 0x02"]
impl crate::Resettable for SYS_FREQ_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
