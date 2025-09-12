#[doc = "Register `CMD` reader"]
pub type R = crate::R<CMD_SPEC>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `FLASH_PE` reader - "]
pub type FLASH_PE_R = crate::BitReader;
#[doc = "Field `FLASH_PE` writer - "]
pub type FLASH_PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR` reader - "]
pub type USR_R = crate::BitReader;
#[doc = "Field `USR` writer - "]
pub type USR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_HPM` reader - "]
pub type FLASH_HPM_R = crate::BitReader;
#[doc = "Field `FLASH_HPM` writer - "]
pub type FLASH_HPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_RES` reader - "]
pub type FLASH_RES_R = crate::BitReader;
#[doc = "Field `FLASH_RES` writer - "]
pub type FLASH_RES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_DP` reader - "]
pub type FLASH_DP_R = crate::BitReader;
#[doc = "Field `FLASH_DP` writer - "]
pub type FLASH_DP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_CE` reader - "]
pub type FLASH_CE_R = crate::BitReader;
#[doc = "Field `FLASH_CE` writer - "]
pub type FLASH_CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_BE` reader - "]
pub type FLASH_BE_R = crate::BitReader;
#[doc = "Field `FLASH_BE` writer - "]
pub type FLASH_BE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_SE` reader - "]
pub type FLASH_SE_R = crate::BitReader;
#[doc = "Field `FLASH_SE` writer - "]
pub type FLASH_SE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PP` reader - "]
pub type FLASH_PP_R = crate::BitReader;
#[doc = "Field `FLASH_PP` writer - "]
pub type FLASH_PP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_WRSR` reader - "]
pub type FLASH_WRSR_R = crate::BitReader;
#[doc = "Field `FLASH_WRSR` writer - "]
pub type FLASH_WRSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_RDSR` reader - "]
pub type FLASH_RDSR_R = crate::BitReader;
#[doc = "Field `FLASH_RDSR` writer - "]
pub type FLASH_RDSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_RDID` reader - "]
pub type FLASH_RDID_R = crate::BitReader;
#[doc = "Field `FLASH_RDID` writer - "]
pub type FLASH_RDID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_WRDI` reader - "]
pub type FLASH_WRDI_R = crate::BitReader;
#[doc = "Field `FLASH_WRDI` writer - "]
pub type FLASH_WRDI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_WREN` reader - "]
pub type FLASH_WREN_R = crate::BitReader;
#[doc = "Field `FLASH_WREN` writer - "]
pub type FLASH_WREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_READ` reader - "]
pub type FLASH_READ_R = crate::BitReader;
#[doc = "Field `FLASH_READ` writer - "]
pub type FLASH_READ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn flash_pe(&self) -> FLASH_PE_R {
        FLASH_PE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn usr(&self) -> USR_R {
        USR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn flash_hpm(&self) -> FLASH_HPM_R {
        FLASH_HPM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn flash_res(&self) -> FLASH_RES_R {
        FLASH_RES_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn flash_dp(&self) -> FLASH_DP_R {
        FLASH_DP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn flash_ce(&self) -> FLASH_CE_R {
        FLASH_CE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn flash_be(&self) -> FLASH_BE_R {
        FLASH_BE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn flash_se(&self) -> FLASH_SE_R {
        FLASH_SE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn flash_pp(&self) -> FLASH_PP_R {
        FLASH_PP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn flash_wrsr(&self) -> FLASH_WRSR_R {
        FLASH_WRSR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn flash_rdsr(&self) -> FLASH_RDSR_R {
        FLASH_RDSR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn flash_rdid(&self) -> FLASH_RDID_R {
        FLASH_RDID_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn flash_wrdi(&self) -> FLASH_WRDI_R {
        FLASH_WRDI_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn flash_wren(&self) -> FLASH_WREN_R {
        FLASH_WREN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn flash_read(&self) -> FLASH_READ_R {
        FLASH_READ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD")
            .field("flash_read", &self.flash_read())
            .field("flash_wren", &self.flash_wren())
            .field("flash_wrdi", &self.flash_wrdi())
            .field("flash_rdid", &self.flash_rdid())
            .field("flash_rdsr", &self.flash_rdsr())
            .field("flash_wrsr", &self.flash_wrsr())
            .field("flash_pp", &self.flash_pp())
            .field("flash_se", &self.flash_se())
            .field("flash_be", &self.flash_be())
            .field("flash_ce", &self.flash_ce())
            .field("flash_dp", &self.flash_dp())
            .field("flash_res", &self.flash_res())
            .field("flash_hpm", &self.flash_hpm())
            .field("usr", &self.usr())
            .field("flash_pe", &self.flash_pe())
            .finish()
    }
}
impl W {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn flash_pe(&mut self) -> FLASH_PE_W<'_, CMD_SPEC> {
        FLASH_PE_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn usr(&mut self) -> USR_W<'_, CMD_SPEC> {
        USR_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn flash_hpm(&mut self) -> FLASH_HPM_W<'_, CMD_SPEC> {
        FLASH_HPM_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn flash_res(&mut self) -> FLASH_RES_W<'_, CMD_SPEC> {
        FLASH_RES_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn flash_dp(&mut self) -> FLASH_DP_W<'_, CMD_SPEC> {
        FLASH_DP_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn flash_ce(&mut self) -> FLASH_CE_W<'_, CMD_SPEC> {
        FLASH_CE_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn flash_be(&mut self) -> FLASH_BE_W<'_, CMD_SPEC> {
        FLASH_BE_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn flash_se(&mut self) -> FLASH_SE_W<'_, CMD_SPEC> {
        FLASH_SE_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn flash_pp(&mut self) -> FLASH_PP_W<'_, CMD_SPEC> {
        FLASH_PP_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn flash_wrsr(&mut self) -> FLASH_WRSR_W<'_, CMD_SPEC> {
        FLASH_WRSR_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn flash_rdsr(&mut self) -> FLASH_RDSR_W<'_, CMD_SPEC> {
        FLASH_RDSR_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn flash_rdid(&mut self) -> FLASH_RDID_W<'_, CMD_SPEC> {
        FLASH_RDID_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn flash_wrdi(&mut self) -> FLASH_WRDI_W<'_, CMD_SPEC> {
        FLASH_WRDI_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn flash_wren(&mut self) -> FLASH_WREN_W<'_, CMD_SPEC> {
        FLASH_WREN_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn flash_read(&mut self) -> FLASH_READ_W<'_, CMD_SPEC> {
        FLASH_READ_W::new(self, 31)
    }
}
#[doc = "SPI Memory Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {}
