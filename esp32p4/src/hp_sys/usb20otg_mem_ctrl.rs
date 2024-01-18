#[doc = "Register `USB20OTG_MEM_CTRL` reader"]
pub type R = crate::R<USB20OTG_MEM_CTRL_SPEC>;
#[doc = "Register `USB20OTG_MEM_CTRL` writer"]
pub type W = crate::W<USB20OTG_MEM_CTRL_SPEC>;
#[doc = "Field `REG_USB20_MEM_CLK_FORCE_ON` reader - NA"]
pub type REG_USB20_MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `REG_USB20_MEM_CLK_FORCE_ON` writer - NA"]
pub type REG_USB20_MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_usb20_mem_clk_force_on(&self) -> REG_USB20_MEM_CLK_FORCE_ON_R {
        REG_USB20_MEM_CLK_FORCE_ON_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB20OTG_MEM_CTRL")
            .field(
                "reg_usb20_mem_clk_force_on",
                &format_args!("{}", self.reg_usb20_mem_clk_force_on().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<USB20OTG_MEM_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb20_mem_clk_force_on(
        &mut self,
    ) -> REG_USB20_MEM_CLK_FORCE_ON_W<USB20OTG_MEM_CTRL_SPEC> {
        REG_USB20_MEM_CLK_FORCE_ON_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb20otg_mem_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb20otg_mem_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB20OTG_MEM_CTRL_SPEC;
impl crate::RegisterSpec for USB20OTG_MEM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb20otg_mem_ctrl::R`](R) reader structure"]
impl crate::Readable for USB20OTG_MEM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb20otg_mem_ctrl::W`](W) writer structure"]
impl crate::Writable for USB20OTG_MEM_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB20OTG_MEM_CTRL to value 0"]
impl crate::Resettable for USB20OTG_MEM_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
