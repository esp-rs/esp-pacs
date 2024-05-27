#[doc = "Register `CLK160M` reader"]
pub type R = crate::R<CLK160M_SPEC>;
#[doc = "Register `CLK160M` writer"]
pub type W = crate::W<CLK160M_SPEC>;
#[doc = "Field `CLK_I2C_MST_SEL_160M` reader - need des"]
pub type CLK_I2C_MST_SEL_160M_R = crate::BitReader;
#[doc = "Field `CLK_I2C_MST_SEL_160M` writer - need des"]
pub type CLK_I2C_MST_SEL_160M_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need des"]
    #[inline(always)]
    pub fn clk_i2c_mst_sel_160m(&self) -> CLK_I2C_MST_SEL_160M_R {
        CLK_I2C_MST_SEL_160M_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK160M")
            .field("clk_i2c_mst_sel_160m", &self.clk_i2c_mst_sel_160m())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c_mst_sel_160m(&mut self) -> CLK_I2C_MST_SEL_160M_W<CLK160M_SPEC> {
        CLK_I2C_MST_SEL_160M_W::new(self, 0)
    }
}
#[doc = "need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk160m::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk160m::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK160M_SPEC;
impl crate::RegisterSpec for CLK160M_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk160m::R`](R) reader structure"]
impl crate::Readable for CLK160M_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk160m::W`](W) writer structure"]
impl crate::Writable for CLK160M_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK160M to value 0"]
impl crate::Resettable for CLK160M_SPEC {
    const RESET_VALUE: u32 = 0;
}
