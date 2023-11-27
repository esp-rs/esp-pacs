#[doc = "Register `IN_PERI_SEL_CH%s` reader"]
pub type R = crate::R<IN_PERI_SEL_CH_SPEC>;
#[doc = "Register `IN_PERI_SEL_CH%s` writer"]
pub type W = crate::W<IN_PERI_SEL_CH_SPEC>;
#[doc = "Field `PERI_IN_SEL_CH` reader - This register is used to select peripheral for Rx channel 0. 0:lcdcam. 1: gpspi_2. 2: gpspi_3. 3: parl_io. 4: aes. 5: sha. 6~15: Dummy"]
pub type PERI_IN_SEL_CH_R = crate::FieldReader;
#[doc = "Field `PERI_IN_SEL_CH` writer - This register is used to select peripheral for Rx channel 0. 0:lcdcam. 1: gpspi_2. 2: gpspi_3. 3: parl_io. 4: aes. 5: sha. 6~15: Dummy"]
pub type PERI_IN_SEL_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - This register is used to select peripheral for Rx channel 0. 0:lcdcam. 1: gpspi_2. 2: gpspi_3. 3: parl_io. 4: aes. 5: sha. 6~15: Dummy"]
    #[inline(always)]
    pub fn peri_in_sel_ch(&self) -> PERI_IN_SEL_CH_R {
        PERI_IN_SEL_CH_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_PERI_SEL_CH")
            .field(
                "peri_in_sel_ch",
                &format_args!("{}", self.peri_in_sel_ch().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_PERI_SEL_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - This register is used to select peripheral for Rx channel 0. 0:lcdcam. 1: gpspi_2. 2: gpspi_3. 3: parl_io. 4: aes. 5: sha. 6~15: Dummy"]
    #[inline(always)]
    #[must_use]
    pub fn peri_in_sel_ch(&mut self) -> PERI_IN_SEL_CH_W<IN_PERI_SEL_CH_SPEC> {
        PERI_IN_SEL_CH_W::new(self, 0)
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
#[doc = "Peripheral selection of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_peri_sel_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_peri_sel_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_PERI_SEL_CH_SPEC;
impl crate::RegisterSpec for IN_PERI_SEL_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_peri_sel_ch::R`](R) reader structure"]
impl crate::Readable for IN_PERI_SEL_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_peri_sel_ch::W`](W) writer structure"]
impl crate::Writable for IN_PERI_SEL_CH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_PERI_SEL_CH%s to value 0x3f"]
impl crate::Resettable for IN_PERI_SEL_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
