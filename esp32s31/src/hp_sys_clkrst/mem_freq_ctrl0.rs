#[doc = "Register `MEM_FREQ_CTRL0` reader"]
pub type R = crate::R<MEM_FREQ_CTRL0_SPEC>;
#[doc = "Register `MEM_FREQ_CTRL0` writer"]
pub type W = crate::W<MEM_FREQ_CTRL0_SPEC>;
#[doc = "Field `REG_MEM_CLK_DIV_NUM` reader - need_des"]
pub type REG_MEM_CLK_DIV_NUM_R = crate::BitReader;
#[doc = "Field `REG_MEM_CLK_DIV_NUM` writer - need_des"]
pub type REG_MEM_CLK_DIV_NUM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_mem_clk_div_num(&self) -> REG_MEM_CLK_DIV_NUM_R {
        REG_MEM_CLK_DIV_NUM_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_FREQ_CTRL0")
            .field("reg_mem_clk_div_num", &self.reg_mem_clk_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_mem_clk_div_num(&mut self) -> REG_MEM_CLK_DIV_NUM_W<'_, MEM_FREQ_CTRL0_SPEC> {
        REG_MEM_CLK_DIV_NUM_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_freq_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_freq_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_FREQ_CTRL0_SPEC;
impl crate::RegisterSpec for MEM_FREQ_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_freq_ctrl0::R`](R) reader structure"]
impl crate::Readable for MEM_FREQ_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_freq_ctrl0::W`](W) writer structure"]
impl crate::Writable for MEM_FREQ_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_FREQ_CTRL0 to value 0x01"]
impl crate::Resettable for MEM_FREQ_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
