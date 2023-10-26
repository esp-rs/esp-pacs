#[doc = "Register `TX_DSCR_CONF` reader"]
pub type R = crate::R<TX_DSCR_CONF_SPEC>;
#[doc = "Register `TX_DSCR_CONF` writer"]
pub type W = crate::W<TX_DSCR_CONF_SPEC>;
#[doc = "Field `WR_RETRY_THRESHOLD` reader - "]
pub type WR_RETRY_THRESHOLD_R = crate::FieldReader<u16>;
#[doc = "Field `WR_RETRY_THRESHOLD` writer - "]
pub type WR_RETRY_THRESHOLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn wr_retry_threshold(&self) -> WR_RETRY_THRESHOLD_R {
        WR_RETRY_THRESHOLD_R::new((self.bits & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_DSCR_CONF")
            .field(
                "wr_retry_threshold",
                &format_args!("{}", self.wr_retry_threshold().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_DSCR_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn wr_retry_threshold(&mut self) -> WR_RETRY_THRESHOLD_W<TX_DSCR_CONF_SPEC, 0> {
        WR_RETRY_THRESHOLD_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_dscr_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_dscr_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_DSCR_CONF_SPEC;
impl crate::RegisterSpec for TX_DSCR_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_dscr_conf::R`](R) reader structure"]
impl crate::Readable for TX_DSCR_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_dscr_conf::W`](W) writer structure"]
impl crate::Writable for TX_DSCR_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_DSCR_CONF to value 0x80"]
impl crate::Resettable for TX_DSCR_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
