#[doc = "Register `ROOT_CLK_CTRL1` reader"]
pub type R = crate::R<ROOT_CLK_CTRL1_SPEC>;
#[doc = "Register `ROOT_CLK_CTRL1` writer"]
pub type W = crate::W<ROOT_CLK_CTRL1_SPEC>;
#[doc = "Field `MEM_CLK_DIV_NUM` reader - Reserved"]
pub type MEM_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `MEM_CLK_DIV_NUM` writer - Reserved"]
pub type MEM_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEM_CLK_DIV_NUMERATOR` reader - Reserved"]
pub type MEM_CLK_DIV_NUMERATOR_R = crate::FieldReader;
#[doc = "Field `MEM_CLK_DIV_NUMERATOR` writer - Reserved"]
pub type MEM_CLK_DIV_NUMERATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEM_CLK_DIV_DENOMINATOR` reader - Reserved"]
pub type MEM_CLK_DIV_DENOMINATOR_R = crate::FieldReader;
#[doc = "Field `MEM_CLK_DIV_DENOMINATOR` writer - Reserved"]
pub type MEM_CLK_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SYS_CLK_DIV_NUM` reader - Reserved"]
pub type SYS_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `SYS_CLK_DIV_NUM` writer - Reserved"]
pub type SYS_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn mem_clk_div_num(&self) -> MEM_CLK_DIV_NUM_R {
        MEM_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn mem_clk_div_numerator(&self) -> MEM_CLK_DIV_NUMERATOR_R {
        MEM_CLK_DIV_NUMERATOR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn mem_clk_div_denominator(&self) -> MEM_CLK_DIV_DENOMINATOR_R {
        MEM_CLK_DIV_DENOMINATOR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Reserved"]
    #[inline(always)]
    pub fn sys_clk_div_num(&self) -> SYS_CLK_DIV_NUM_R {
        SYS_CLK_DIV_NUM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROOT_CLK_CTRL1")
            .field("mem_clk_div_num", &self.mem_clk_div_num())
            .field("mem_clk_div_numerator", &self.mem_clk_div_numerator())
            .field("mem_clk_div_denominator", &self.mem_clk_div_denominator())
            .field("sys_clk_div_num", &self.sys_clk_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn mem_clk_div_num(&mut self) -> MEM_CLK_DIV_NUM_W<ROOT_CLK_CTRL1_SPEC> {
        MEM_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn mem_clk_div_numerator(&mut self) -> MEM_CLK_DIV_NUMERATOR_W<ROOT_CLK_CTRL1_SPEC> {
        MEM_CLK_DIV_NUMERATOR_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn mem_clk_div_denominator(&mut self) -> MEM_CLK_DIV_DENOMINATOR_W<ROOT_CLK_CTRL1_SPEC> {
        MEM_CLK_DIV_DENOMINATOR_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Reserved"]
    #[inline(always)]
    pub fn sys_clk_div_num(&mut self) -> SYS_CLK_DIV_NUM_W<ROOT_CLK_CTRL1_SPEC> {
        SYS_CLK_DIV_NUM_W::new(self, 24)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`root_clk_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`root_clk_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROOT_CLK_CTRL1_SPEC;
impl crate::RegisterSpec for ROOT_CLK_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`root_clk_ctrl1::R`](R) reader structure"]
impl crate::Readable for ROOT_CLK_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`root_clk_ctrl1::W`](W) writer structure"]
impl crate::Writable for ROOT_CLK_CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROOT_CLK_CTRL1 to value 0x01"]
impl crate::Resettable for ROOT_CLK_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
