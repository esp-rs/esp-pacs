#[doc = "Register `TX_CFG1` reader"]
pub type R = crate::R<TX_CFG1_SPEC>;
#[doc = "Register `TX_CFG1` writer"]
pub type W = crate::W<TX_CFG1_SPEC>;
#[doc = "Field `TX_IDLE_VALUE` reader - Configures data value on tx bus when IDLE state."]
pub type TX_IDLE_VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `TX_IDLE_VALUE` writer - Configures data value on tx bus when IDLE state."]
pub type TX_IDLE_VALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 16:31 - Configures data value on tx bus when IDLE state."]
    #[inline(always)]
    pub fn tx_idle_value(&self) -> TX_IDLE_VALUE_R {
        TX_IDLE_VALUE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CFG1")
            .field(
                "tx_idle_value",
                &format_args!("{}", self.tx_idle_value().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_CFG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 16:31 - Configures data value on tx bus when IDLE state."]
    #[inline(always)]
    #[must_use]
    pub fn tx_idle_value(&mut self) -> TX_IDLE_VALUE_W<TX_CFG1_SPEC, 16> {
        TX_IDLE_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Parallel TX module configuration register1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CFG1_SPEC;
impl crate::RegisterSpec for TX_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_cfg1::R`](R) reader structure"]
impl crate::Readable for TX_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_cfg1::W`](W) writer structure"]
impl crate::Writable for TX_CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_CFG1 to value 0"]
impl crate::Resettable for TX_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
