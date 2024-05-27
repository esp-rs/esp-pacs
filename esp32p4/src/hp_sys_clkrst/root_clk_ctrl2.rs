#[doc = "Register `ROOT_CLK_CTRL2` reader"]
pub type R = crate::R<ROOT_CLK_CTRL2_SPEC>;
#[doc = "Register `ROOT_CLK_CTRL2` writer"]
pub type W = crate::W<ROOT_CLK_CTRL2_SPEC>;
#[doc = "Field `SYS_CLK_DIV_NUMERATOR` reader - Reserved"]
pub type SYS_CLK_DIV_NUMERATOR_R = crate::FieldReader;
#[doc = "Field `SYS_CLK_DIV_NUMERATOR` writer - Reserved"]
pub type SYS_CLK_DIV_NUMERATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SYS_CLK_DIV_DENOMINATOR` reader - Reserved"]
pub type SYS_CLK_DIV_DENOMINATOR_R = crate::FieldReader;
#[doc = "Field `SYS_CLK_DIV_DENOMINATOR` writer - Reserved"]
pub type SYS_CLK_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `APB_CLK_DIV_NUM` reader - Reserved"]
pub type APB_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `APB_CLK_DIV_NUM` writer - Reserved"]
pub type APB_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `APB_CLK_DIV_NUMERATOR` reader - Reserved"]
pub type APB_CLK_DIV_NUMERATOR_R = crate::FieldReader;
#[doc = "Field `APB_CLK_DIV_NUMERATOR` writer - Reserved"]
pub type APB_CLK_DIV_NUMERATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn sys_clk_div_numerator(&self) -> SYS_CLK_DIV_NUMERATOR_R {
        SYS_CLK_DIV_NUMERATOR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn sys_clk_div_denominator(&self) -> SYS_CLK_DIV_DENOMINATOR_R {
        SYS_CLK_DIV_DENOMINATOR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn apb_clk_div_num(&self) -> APB_CLK_DIV_NUM_R {
        APB_CLK_DIV_NUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Reserved"]
    #[inline(always)]
    pub fn apb_clk_div_numerator(&self) -> APB_CLK_DIV_NUMERATOR_R {
        APB_CLK_DIV_NUMERATOR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROOT_CLK_CTRL2")
            .field("sys_clk_div_numerator", &self.sys_clk_div_numerator())
            .field("sys_clk_div_denominator", &self.sys_clk_div_denominator())
            .field("apb_clk_div_num", &self.apb_clk_div_num())
            .field("apb_clk_div_numerator", &self.apb_clk_div_numerator())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn sys_clk_div_numerator(&mut self) -> SYS_CLK_DIV_NUMERATOR_W<ROOT_CLK_CTRL2_SPEC> {
        SYS_CLK_DIV_NUMERATOR_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn sys_clk_div_denominator(&mut self) -> SYS_CLK_DIV_DENOMINATOR_W<ROOT_CLK_CTRL2_SPEC> {
        SYS_CLK_DIV_DENOMINATOR_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn apb_clk_div_num(&mut self) -> APB_CLK_DIV_NUM_W<ROOT_CLK_CTRL2_SPEC> {
        APB_CLK_DIV_NUM_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn apb_clk_div_numerator(&mut self) -> APB_CLK_DIV_NUMERATOR_W<ROOT_CLK_CTRL2_SPEC> {
        APB_CLK_DIV_NUMERATOR_W::new(self, 24)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`root_clk_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`root_clk_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROOT_CLK_CTRL2_SPEC;
impl crate::RegisterSpec for ROOT_CLK_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`root_clk_ctrl2::R`](R) reader structure"]
impl crate::Readable for ROOT_CLK_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`root_clk_ctrl2::W`](W) writer structure"]
impl crate::Writable for ROOT_CLK_CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROOT_CLK_CTRL2 to value 0x0001_0000"]
impl crate::Resettable for ROOT_CLK_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x0001_0000;
}
