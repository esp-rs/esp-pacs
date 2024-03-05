#[doc = "Register `APB_TSENS_CTRL` reader"]
pub type R = crate::R<APB_TSENS_CTRL_SPEC>;
#[doc = "Register `APB_TSENS_CTRL` writer"]
pub type W = crate::W<APB_TSENS_CTRL_SPEC>;
#[doc = "Field `TSENS_OUT` reader - temperature sensor data out"]
pub type TSENS_OUT_R = crate::FieldReader;
#[doc = "Field `TSENS_IN_INV` reader - invert temperature sensor data"]
pub type TSENS_IN_INV_R = crate::BitReader;
#[doc = "Field `TSENS_IN_INV` writer - invert temperature sensor data"]
pub type TSENS_IN_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENS_CLK_DIV` reader - temperature sensor clock divider"]
pub type TSENS_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `TSENS_CLK_DIV` writer - temperature sensor clock divider"]
pub type TSENS_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TSENS_PU` reader - temperature sensor power up"]
pub type TSENS_PU_R = crate::BitReader;
#[doc = "Field `TSENS_PU` writer - temperature sensor power up"]
pub type TSENS_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - temperature sensor data out"]
    #[inline(always)]
    pub fn tsens_out(&self) -> TSENS_OUT_R {
        TSENS_OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 13 - invert temperature sensor data"]
    #[inline(always)]
    pub fn tsens_in_inv(&self) -> TSENS_IN_INV_R {
        TSENS_IN_INV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:21 - temperature sensor clock divider"]
    #[inline(always)]
    pub fn tsens_clk_div(&self) -> TSENS_CLK_DIV_R {
        TSENS_CLK_DIV_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bit 22 - temperature sensor power up"]
    #[inline(always)]
    pub fn tsens_pu(&self) -> TSENS_PU_R {
        TSENS_PU_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_TSENS_CTRL")
            .field("tsens_out", &format_args!("{}", self.tsens_out().bits()))
            .field(
                "tsens_in_inv",
                &format_args!("{}", self.tsens_in_inv().bit()),
            )
            .field(
                "tsens_clk_div",
                &format_args!("{}", self.tsens_clk_div().bits()),
            )
            .field("tsens_pu", &format_args!("{}", self.tsens_pu().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_TSENS_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 13 - invert temperature sensor data"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_in_inv(&mut self) -> TSENS_IN_INV_W<APB_TSENS_CTRL_SPEC> {
        TSENS_IN_INV_W::new(self, 13)
    }
    #[doc = "Bits 14:21 - temperature sensor clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_div(&mut self) -> TSENS_CLK_DIV_W<APB_TSENS_CTRL_SPEC> {
        TSENS_CLK_DIV_W::new(self, 14)
    }
    #[doc = "Bit 22 - temperature sensor power up"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_pu(&mut self) -> TSENS_PU_W<APB_TSENS_CTRL_SPEC> {
        TSENS_PU_W::new(self, 22)
    }
}
#[doc = "digital tsens configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_tsens_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_tsens_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_TSENS_CTRL_SPEC;
impl crate::RegisterSpec for APB_TSENS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_tsens_ctrl::R`](R) reader structure"]
impl crate::Readable for APB_TSENS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_tsens_ctrl::W`](W) writer structure"]
impl crate::Writable for APB_TSENS_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB_TSENS_CTRL to value 0x0001_8000"]
impl crate::Resettable for APB_TSENS_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0001_8000;
}
