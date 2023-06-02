#[doc = "Register `SER_AFIFO_CONFIG` reader"]
pub struct R(crate::R<SER_AFIFO_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SER_AFIFO_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SER_AFIFO_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SER_AFIFO_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SER_AFIFO_CONFIG` writer"]
pub struct W(crate::W<SER_AFIFO_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SER_AFIFO_CONFIG_SPEC>;
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
impl From<crate::W<SER_AFIFO_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SER_AFIFO_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SERIAL_IN_AFIFO_RESET_WR` reader - Write 1 to reset CDC_ACM IN async FIFO write clock domain."]
pub type SERIAL_IN_AFIFO_RESET_WR_R = crate::BitReader;
#[doc = "Field `SERIAL_IN_AFIFO_RESET_WR` writer - Write 1 to reset CDC_ACM IN async FIFO write clock domain."]
pub type SERIAL_IN_AFIFO_RESET_WR_W<'a, const O: u8> =
    crate::BitWriter<'a, SER_AFIFO_CONFIG_SPEC, O>;
#[doc = "Field `SERIAL_IN_AFIFO_RESET_RD` reader - Write 1 to reset CDC_ACM IN async FIFO read clock domain."]
pub type SERIAL_IN_AFIFO_RESET_RD_R = crate::BitReader;
#[doc = "Field `SERIAL_IN_AFIFO_RESET_RD` writer - Write 1 to reset CDC_ACM IN async FIFO read clock domain."]
pub type SERIAL_IN_AFIFO_RESET_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, SER_AFIFO_CONFIG_SPEC, O>;
#[doc = "Field `SERIAL_OUT_AFIFO_RESET_WR` reader - Write 1 to reset CDC_ACM OUT async FIFO write clock domain."]
pub type SERIAL_OUT_AFIFO_RESET_WR_R = crate::BitReader;
#[doc = "Field `SERIAL_OUT_AFIFO_RESET_WR` writer - Write 1 to reset CDC_ACM OUT async FIFO write clock domain."]
pub type SERIAL_OUT_AFIFO_RESET_WR_W<'a, const O: u8> =
    crate::BitWriter<'a, SER_AFIFO_CONFIG_SPEC, O>;
#[doc = "Field `SERIAL_OUT_AFIFO_RESET_RD` reader - Write 1 to reset CDC_ACM OUT async FIFO read clock domain."]
pub type SERIAL_OUT_AFIFO_RESET_RD_R = crate::BitReader;
#[doc = "Field `SERIAL_OUT_AFIFO_RESET_RD` writer - Write 1 to reset CDC_ACM OUT async FIFO read clock domain."]
pub type SERIAL_OUT_AFIFO_RESET_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, SER_AFIFO_CONFIG_SPEC, O>;
#[doc = "Field `SERIAL_OUT_AFIFO_REMPTY` reader - CDC_ACM OUTOUT async FIFO empty signal in read clock domain."]
pub type SERIAL_OUT_AFIFO_REMPTY_R = crate::BitReader;
#[doc = "Field `SERIAL_IN_AFIFO_WFULL` reader - CDC_ACM OUT IN async FIFO empty signal in write clock domain."]
pub type SERIAL_IN_AFIFO_WFULL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Write 1 to reset CDC_ACM IN async FIFO write clock domain."]
    #[inline(always)]
    pub fn serial_in_afifo_reset_wr(&self) -> SERIAL_IN_AFIFO_RESET_WR_R {
        SERIAL_IN_AFIFO_RESET_WR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to reset CDC_ACM IN async FIFO read clock domain."]
    #[inline(always)]
    pub fn serial_in_afifo_reset_rd(&self) -> SERIAL_IN_AFIFO_RESET_RD_R {
        SERIAL_IN_AFIFO_RESET_RD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to reset CDC_ACM OUT async FIFO write clock domain."]
    #[inline(always)]
    pub fn serial_out_afifo_reset_wr(&self) -> SERIAL_OUT_AFIFO_RESET_WR_R {
        SERIAL_OUT_AFIFO_RESET_WR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to reset CDC_ACM OUT async FIFO read clock domain."]
    #[inline(always)]
    pub fn serial_out_afifo_reset_rd(&self) -> SERIAL_OUT_AFIFO_RESET_RD_R {
        SERIAL_OUT_AFIFO_RESET_RD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CDC_ACM OUTOUT async FIFO empty signal in read clock domain."]
    #[inline(always)]
    pub fn serial_out_afifo_rempty(&self) -> SERIAL_OUT_AFIFO_REMPTY_R {
        SERIAL_OUT_AFIFO_REMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CDC_ACM OUT IN async FIFO empty signal in write clock domain."]
    #[inline(always)]
    pub fn serial_in_afifo_wfull(&self) -> SERIAL_IN_AFIFO_WFULL_R {
        SERIAL_IN_AFIFO_WFULL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SER_AFIFO_CONFIG")
            .field(
                "serial_in_afifo_reset_wr",
                &format_args!("{}", self.serial_in_afifo_reset_wr().bit()),
            )
            .field(
                "serial_in_afifo_reset_rd",
                &format_args!("{}", self.serial_in_afifo_reset_rd().bit()),
            )
            .field(
                "serial_out_afifo_reset_wr",
                &format_args!("{}", self.serial_out_afifo_reset_wr().bit()),
            )
            .field(
                "serial_out_afifo_reset_rd",
                &format_args!("{}", self.serial_out_afifo_reset_rd().bit()),
            )
            .field(
                "serial_out_afifo_rempty",
                &format_args!("{}", self.serial_out_afifo_rempty().bit()),
            )
            .field(
                "serial_in_afifo_wfull",
                &format_args!("{}", self.serial_in_afifo_wfull().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SER_AFIFO_CONFIG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to reset CDC_ACM IN async FIFO write clock domain."]
    #[inline(always)]
    #[must_use]
    pub fn serial_in_afifo_reset_wr(&mut self) -> SERIAL_IN_AFIFO_RESET_WR_W<0> {
        SERIAL_IN_AFIFO_RESET_WR_W::new(self)
    }
    #[doc = "Bit 1 - Write 1 to reset CDC_ACM IN async FIFO read clock domain."]
    #[inline(always)]
    #[must_use]
    pub fn serial_in_afifo_reset_rd(&mut self) -> SERIAL_IN_AFIFO_RESET_RD_W<1> {
        SERIAL_IN_AFIFO_RESET_RD_W::new(self)
    }
    #[doc = "Bit 2 - Write 1 to reset CDC_ACM OUT async FIFO write clock domain."]
    #[inline(always)]
    #[must_use]
    pub fn serial_out_afifo_reset_wr(&mut self) -> SERIAL_OUT_AFIFO_RESET_WR_W<2> {
        SERIAL_OUT_AFIFO_RESET_WR_W::new(self)
    }
    #[doc = "Bit 3 - Write 1 to reset CDC_ACM OUT async FIFO read clock domain."]
    #[inline(always)]
    #[must_use]
    pub fn serial_out_afifo_reset_rd(&mut self) -> SERIAL_OUT_AFIFO_RESET_RD_W<3> {
        SERIAL_OUT_AFIFO_RESET_RD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Serial AFIFO configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ser_afifo_config](index.html) module"]
pub struct SER_AFIFO_CONFIG_SPEC;
impl crate::RegisterSpec for SER_AFIFO_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ser_afifo_config::R](R) reader structure"]
impl crate::Readable for SER_AFIFO_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ser_afifo_config::W](W) writer structure"]
impl crate::Writable for SER_AFIFO_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SER_AFIFO_CONFIG to value 0x10"]
impl crate::Resettable for SER_AFIFO_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
