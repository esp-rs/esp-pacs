#[doc = "Register `AT_CMD_GAPTOUT` reader"]
pub struct R(crate::R<AT_CMD_GAPTOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AT_CMD_GAPTOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AT_CMD_GAPTOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AT_CMD_GAPTOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AT_CMD_GAPTOUT` writer"]
pub struct W(crate::W<AT_CMD_GAPTOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AT_CMD_GAPTOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<AT_CMD_GAPTOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AT_CMD_GAPTOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_GAP_TOUT` reader - This register is used to configure the duration time between the AT_CMD characters. It will not take the data as continuous AT_CMD characters when the duration time is less than this register's value."]
pub type RX_GAP_TOUT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RX_GAP_TOUT` writer - This register is used to configure the duration time between the AT_CMD characters. It will not take the data as continuous AT_CMD characters when the duration time is less than this register's value."]
pub type RX_GAP_TOUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, AT_CMD_GAPTOUT_SPEC, 16, O, u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is used to configure the duration time between the AT_CMD characters. It will not take the data as continuous AT_CMD characters when the duration time is less than this register's value."]
    #[inline(always)]
    pub fn rx_gap_tout(&self) -> RX_GAP_TOUT_R {
        RX_GAP_TOUT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AT_CMD_GAPTOUT")
            .field(
                "rx_gap_tout",
                &format_args!("{}", self.rx_gap_tout().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AT_CMD_GAPTOUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to configure the duration time between the AT_CMD characters. It will not take the data as continuous AT_CMD characters when the duration time is less than this register's value."]
    #[inline(always)]
    #[must_use]
    pub fn rx_gap_tout(&mut self) -> RX_GAP_TOUT_W<0> {
        RX_GAP_TOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timeout configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [at_cmd_gaptout](index.html) module"]
pub struct AT_CMD_GAPTOUT_SPEC;
impl crate::RegisterSpec for AT_CMD_GAPTOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [at_cmd_gaptout::R](R) reader structure"]
impl crate::Readable for AT_CMD_GAPTOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [at_cmd_gaptout::W](W) writer structure"]
impl crate::Writable for AT_CMD_GAPTOUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AT_CMD_GAPTOUT to value 0x0b"]
impl crate::Resettable for AT_CMD_GAPTOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b;
}
