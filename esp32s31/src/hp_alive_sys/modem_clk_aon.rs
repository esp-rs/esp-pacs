#[doc = "Register `MODEM_CLK_AON` reader"]
pub type R = crate::R<MODEM_CLK_AON_SPEC>;
#[doc = "Register `MODEM_CLK_AON` writer"]
pub type W = crate::W<MODEM_CLK_AON_SPEC>;
#[doc = "Field `SEL` reader - Modem clk aon sel. 0: foscl 1:xtal"]
pub type SEL_R = crate::BitReader;
#[doc = "Field `SEL` writer - Modem clk aon sel. 0: foscl 1:xtal"]
pub type SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Modem clk aon sel. 0: foscl 1:xtal"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_CLK_AON")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - Modem clk aon sel. 0: foscl 1:xtal"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<'_, MODEM_CLK_AON_SPEC> {
        SEL_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_clk_aon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_clk_aon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODEM_CLK_AON_SPEC;
impl crate::RegisterSpec for MODEM_CLK_AON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modem_clk_aon::R`](R) reader structure"]
impl crate::Readable for MODEM_CLK_AON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modem_clk_aon::W`](W) writer structure"]
impl crate::Writable for MODEM_CLK_AON_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODEM_CLK_AON to value 0x8000_0000"]
impl crate::Resettable for MODEM_CLK_AON_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}
