///Register `CLK` reader
pub type R = crate::R<CLK_SPEC>;
///Register `CLK` writer
pub type W = crate::W<CLK_SPEC>;
///Field `EN` reader - Force clock on for this register file
pub type EN_R = crate::BitReader;
///Field `EN` writer - Force clock on for this register file
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Force clock on for this register file
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK").field("en", &self.en()).finish()
    }
}
impl W {
    ///Bit 0 - Force clock on for this register file
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CLK_SPEC> {
        EN_W::new(self, 0)
    }
}
/**Parallel IO clk configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`clk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CLK_SPEC;
impl crate::RegisterSpec for CLK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`clk::R`](R) reader structure
impl crate::Readable for CLK_SPEC {}
///`write(|w| ..)` method takes [`clk::W`](W) writer structure
impl crate::Writable for CLK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CLK to value 0
impl crate::Resettable for CLK_SPEC {
    const RESET_VALUE: u32 = 0;
}
