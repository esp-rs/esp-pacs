#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_RAW` writer"]
pub struct W(crate::W<INT_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_RAW_SPEC>;
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
impl From<crate::W<INT_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOTAL_TRANS_END_INT_RAW` reader - The raw bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt. 1: Triggered when SPI1 transfer is done and flash is already idle. When WRSR/PP/SE/BE/CE is sent and PES/PER command is sent, this bit is set when WRSR/PP/SE/BE/CE is success. 0: Others."]
pub type TOTAL_TRANS_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `TOTAL_TRANS_END_INT_RAW` writer - The raw bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt. 1: Triggered when SPI1 transfer is done and flash is already idle. When WRSR/PP/SE/BE/CE is sent and PES/PER command is sent, this bit is set when WRSR/PP/SE/BE/CE is success. 0: Others."]
pub type TOTAL_TRANS_END_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `ECC_ERR_INT_RAW` reader - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When APB_CTRL_FECC_ERR_INT_EN is set and APB_CTRL_SECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN is cleared and APB_CTRL_SECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN and APB_CTRL_SECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN and APB_CTRL_SECC_ERR_INT_EN are cleared, this bit will not be triggered."]
pub type ECC_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `ECC_ERR_INT_RAW` writer - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When APB_CTRL_FECC_ERR_INT_EN is set and APB_CTRL_SECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN is cleared and APB_CTRL_SECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN and APB_CTRL_SECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN and APB_CTRL_SECC_ERR_INT_EN are cleared, this bit will not be triggered."]
pub type ECC_ERR_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
impl R {
    #[doc = "Bit 2 - The raw bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt. 1: Triggered when SPI1 transfer is done and flash is already idle. When WRSR/PP/SE/BE/CE is sent and PES/PER command is sent, this bit is set when WRSR/PP/SE/BE/CE is success. 0: Others."]
    #[inline(always)]
    pub fn total_trans_end_int_raw(&self) -> TOTAL_TRANS_END_INT_RAW_R {
        TOTAL_TRANS_END_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When APB_CTRL_FECC_ERR_INT_EN is set and APB_CTRL_SECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN is cleared and APB_CTRL_SECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN and APB_CTRL_SECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN and APB_CTRL_SECC_ERR_INT_EN are cleared, this bit will not be triggered."]
    #[inline(always)]
    pub fn ecc_err_int_raw(&self) -> ECC_ERR_INT_RAW_R {
        ECC_ERR_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "total_trans_end_int_raw",
                &format_args!("{}", self.total_trans_end_int_raw().bit()),
            )
            .field(
                "ecc_err_int_raw",
                &format_args!("{}", self.ecc_err_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 2 - The raw bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt. 1: Triggered when SPI1 transfer is done and flash is already idle. When WRSR/PP/SE/BE/CE is sent and PES/PER command is sent, this bit is set when WRSR/PP/SE/BE/CE is success. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn total_trans_end_int_raw(&mut self) -> TOTAL_TRANS_END_INT_RAW_W<2> {
        TOTAL_TRANS_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 4 - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When APB_CTRL_FECC_ERR_INT_EN is set and APB_CTRL_SECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN is cleared and APB_CTRL_SECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN and APB_CTRL_SECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN and APB_CTRL_SECC_ERR_INT_EN are cleared, this bit will not be triggered."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_err_int_raw(&mut self) -> ECC_ERR_INT_RAW_W<4> {
        ECC_ERR_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 interrupt raw register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_raw::W](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
