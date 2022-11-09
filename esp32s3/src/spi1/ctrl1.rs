#[doc = "Register `CTRL1` reader"]
pub struct R(crate::R<CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL1` writer"]
pub struct W(crate::W<CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL1_SPEC>;
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
impl From<crate::W<CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_MODE` reader - SPI Bus clock (SPI_CLK) mode bits. 0: SPI Bus clock (SPI_CLK) is off when CS inactive 1: SPI_CLK is delayed one cycle after SPI_CS inactive 2: SPI_CLK is delayed two cycles after SPI_CS inactive 3: SPI_CLK is always on."]
pub type CLK_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLK_MODE` writer - SPI Bus clock (SPI_CLK) mode bits. 0: SPI Bus clock (SPI_CLK) is off when CS inactive 1: SPI_CLK is delayed one cycle after SPI_CS inactive 2: SPI_CLK is delayed two cycles after SPI_CS inactive 3: SPI_CLK is always on."]
pub type CLK_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CS_HOLD_DLY_RES` reader - After RES/DP/HPM/PES/PER command is sent, SPI1 may waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or * 256) SPI_CLK cycles."]
pub type CS_HOLD_DLY_RES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CS_HOLD_DLY_RES` writer - After RES/DP/HPM/PES/PER command is sent, SPI1 may waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or * 256) SPI_CLK cycles."]
pub type CS_HOLD_DLY_RES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL1_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:1 - SPI Bus clock (SPI_CLK) mode bits. 0: SPI Bus clock (SPI_CLK) is off when CS inactive 1: SPI_CLK is delayed one cycle after SPI_CS inactive 2: SPI_CLK is delayed two cycles after SPI_CS inactive 3: SPI_CLK is always on."]
    #[inline(always)]
    pub fn clk_mode(&self) -> CLK_MODE_R {
        CLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:11 - After RES/DP/HPM/PES/PER command is sent, SPI1 may waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or * 256) SPI_CLK cycles."]
    #[inline(always)]
    pub fn cs_hold_dly_res(&self) -> CS_HOLD_DLY_RES_R {
        CS_HOLD_DLY_RES_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI Bus clock (SPI_CLK) mode bits. 0: SPI Bus clock (SPI_CLK) is off when CS inactive 1: SPI_CLK is delayed one cycle after SPI_CS inactive 2: SPI_CLK is delayed two cycles after SPI_CS inactive 3: SPI_CLK is always on."]
    #[inline(always)]
    #[must_use]
    pub fn clk_mode(&mut self) -> CLK_MODE_W<0> {
        CLK_MODE_W::new(self)
    }
    #[doc = "Bits 2:11 - After RES/DP/HPM/PES/PER command is sent, SPI1 may waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or * 256) SPI_CLK cycles."]
    #[inline(always)]
    #[must_use]
    pub fn cs_hold_dly_res(&mut self) -> CS_HOLD_DLY_RES_W<2> {
        CS_HOLD_DLY_RES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 control1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](index.html) module"]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl1::R](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0x0ffc"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0ffc;
}
