#[doc = "Register `ID` reader"]
pub struct R(crate::R<ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ID` writer"]
pub struct W(crate::W<ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ID_SPEC>;
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
impl From<crate::W<ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID` reader - This register is used to configure the uart_id."]
pub type ID_R = crate::FieldReader<u32>;
#[doc = "Field `ID` writer - This register is used to configure the uart_id."]
pub type ID_W<'a, const O: u8> = crate::FieldWriter<'a, ID_SPEC, 30, O, u32>;
#[doc = "Field `HIGH_SPEED` reader - This bit used to select synchronize mode. 1: Registers are auto synchronized into UART Core clock and UART core should be keep the same with APB clock. 0: After configure registers, software needs to write 1 to UART_REG_UPDATE to synchronize registers."]
pub type HIGH_SPEED_R = crate::BitReader;
#[doc = "Field `HIGH_SPEED` writer - This bit used to select synchronize mode. 1: Registers are auto synchronized into UART Core clock and UART core should be keep the same with APB clock. 0: After configure registers, software needs to write 1 to UART_REG_UPDATE to synchronize registers."]
pub type HIGH_SPEED_W<'a, const O: u8> = crate::BitWriter<'a, ID_SPEC, O>;
#[doc = "Field `REG_UPDATE` reader - Software write 1 would synchronize registers into UART Core clock domain and would be cleared by hardware after synchronization is done."]
pub type REG_UPDATE_R = crate::BitReader;
#[doc = "Field `REG_UPDATE` writer - Software write 1 would synchronize registers into UART Core clock domain and would be cleared by hardware after synchronization is done."]
pub type REG_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, ID_SPEC, O>;
impl R {
    #[doc = "Bits 0:29 - This register is used to configure the uart_id."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bit 30 - This bit used to select synchronize mode. 1: Registers are auto synchronized into UART Core clock and UART core should be keep the same with APB clock. 0: After configure registers, software needs to write 1 to UART_REG_UPDATE to synchronize registers."]
    #[inline(always)]
    pub fn high_speed(&self) -> HIGH_SPEED_R {
        HIGH_SPEED_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Software write 1 would synchronize registers into UART Core clock domain and would be cleared by hardware after synchronization is done."]
    #[inline(always)]
    pub fn reg_update(&self) -> REG_UPDATE_R {
        REG_UPDATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID")
            .field("id", &format_args!("{}", self.id().bits()))
            .field("high_speed", &format_args!("{}", self.high_speed().bit()))
            .field("reg_update", &format_args!("{}", self.reg_update().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:29 - This register is used to configure the uart_id."]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<0> {
        ID_W::new(self)
    }
    #[doc = "Bit 30 - This bit used to select synchronize mode. 1: Registers are auto synchronized into UART Core clock and UART core should be keep the same with APB clock. 0: After configure registers, software needs to write 1 to UART_REG_UPDATE to synchronize registers."]
    #[inline(always)]
    #[must_use]
    pub fn high_speed(&mut self) -> HIGH_SPEED_W<30> {
        HIGH_SPEED_W::new(self)
    }
    #[doc = "Bit 31 - Software write 1 would synchronize registers into UART Core clock domain and would be cleared by hardware after synchronization is done."]
    #[inline(always)]
    #[must_use]
    pub fn reg_update(&mut self) -> REG_UPDATE_W<31> {
        REG_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART ID register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id](index.html) module"]
pub struct ID_SPEC;
impl crate::RegisterSpec for ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id::R](R) reader structure"]
impl crate::Readable for ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [id::W](W) writer structure"]
impl crate::Writable for ID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ID to value 0x4000_0500"]
impl crate::Resettable for ID_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0500;
}
