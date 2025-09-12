#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `TOTAL_TRANS_END` reader - The raw bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt. 1: Triggered when SPI1 transfer is done and flash is already idle. When WRSR/PP/SE/BE/CE is sent and PES/PER command is sent, this bit is set when WRSR/PP/SE/BE/CE is success. 0: Others."]
pub type TOTAL_TRANS_END_R = crate::BitReader;
#[doc = "Field `TOTAL_TRANS_END` writer - The raw bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt. 1: Triggered when SPI1 transfer is done and flash is already idle. When WRSR/PP/SE/BE/CE is sent and PES/PER command is sent, this bit is set when WRSR/PP/SE/BE/CE is success. 0: Others."]
pub type TOTAL_TRANS_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_ERR` reader - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When APB_CTRL_FECC_ERR_INT_EN is set and APB_CTRL_SECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN is cleared and APB_CTRL_SECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN and APB_CTRL_SECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN and APB_CTRL_SECC_ERR_INT_EN are cleared, this bit will not be triggered."]
pub type ECC_ERR_R = crate::BitReader;
#[doc = "Field `ECC_ERR` writer - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When APB_CTRL_FECC_ERR_INT_EN is set and APB_CTRL_SECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN is cleared and APB_CTRL_SECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN and APB_CTRL_SECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN and APB_CTRL_SECC_ERR_INT_EN are cleared, this bit will not be triggered."]
pub type ECC_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - The raw bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt. 1: Triggered when SPI1 transfer is done and flash is already idle. When WRSR/PP/SE/BE/CE is sent and PES/PER command is sent, this bit is set when WRSR/PP/SE/BE/CE is success. 0: Others."]
    #[inline(always)]
    pub fn total_trans_end(&self) -> TOTAL_TRANS_END_R {
        TOTAL_TRANS_END_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When APB_CTRL_FECC_ERR_INT_EN is set and APB_CTRL_SECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN is cleared and APB_CTRL_SECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN and APB_CTRL_SECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN and APB_CTRL_SECC_ERR_INT_EN are cleared, this bit will not be triggered."]
    #[inline(always)]
    pub fn ecc_err(&self) -> ECC_ERR_R {
        ECC_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("total_trans_end", &self.total_trans_end())
            .field("ecc_err", &self.ecc_err())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - The raw bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt. 1: Triggered when SPI1 transfer is done and flash is already idle. When WRSR/PP/SE/BE/CE is sent and PES/PER command is sent, this bit is set when WRSR/PP/SE/BE/CE is success. 0: Others."]
    #[inline(always)]
    pub fn total_trans_end(&mut self) -> TOTAL_TRANS_END_W<'_, INT_RAW_SPEC> {
        TOTAL_TRANS_END_W::new(self, 2)
    }
    #[doc = "Bit 4 - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When APB_CTRL_FECC_ERR_INT_EN is set and APB_CTRL_SECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN is cleared and APB_CTRL_SECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN and APB_CTRL_SECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than APB_CTRL_ECC_ERR_INT_NUM. When APB_CTRL_FECC_ERR_INT_EN and APB_CTRL_SECC_ERR_INT_EN are cleared, this bit will not be triggered."]
    #[inline(always)]
    pub fn ecc_err(&mut self) -> ECC_ERR_W<'_, INT_RAW_SPEC> {
        ECC_ERR_W::new(self, 4)
    }
}
#[doc = "SPI1 interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {}
