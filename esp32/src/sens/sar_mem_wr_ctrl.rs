#[doc = "Register `SAR_MEM_WR_CTRL` reader"]
pub type R = crate::R<SAR_MEM_WR_CTRL_SPEC>;
#[doc = "Register `SAR_MEM_WR_CTRL` writer"]
pub type W = crate::W<SAR_MEM_WR_CTRL_SPEC>;
#[doc = "Field `MEM_WR_ADDR_INIT` reader - "]
pub type MEM_WR_ADDR_INIT_R = crate::FieldReader<u16>;
#[doc = "Field `MEM_WR_ADDR_INIT` writer - "]
pub type MEM_WR_ADDR_INIT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `MEM_WR_ADDR_SIZE` reader - "]
pub type MEM_WR_ADDR_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `MEM_WR_ADDR_SIZE` writer - "]
pub type MEM_WR_ADDR_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `RTC_MEM_WR_OFFST_CLR` writer - "]
pub type RTC_MEM_WR_OFFST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn mem_wr_addr_init(&self) -> MEM_WR_ADDR_INIT_R {
        MEM_WR_ADDR_INIT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn mem_wr_addr_size(&self) -> MEM_WR_ADDR_SIZE_R {
        MEM_WR_ADDR_SIZE_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_MEM_WR_CTRL")
            .field(
                "mem_wr_addr_init",
                &format_args!("{}", self.mem_wr_addr_init().bits()),
            )
            .field(
                "mem_wr_addr_size",
                &format_args!("{}", self.mem_wr_addr_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_MEM_WR_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn mem_wr_addr_init(&mut self) -> MEM_WR_ADDR_INIT_W<SAR_MEM_WR_CTRL_SPEC> {
        MEM_WR_ADDR_INIT_W::new(self, 0)
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    #[must_use]
    pub fn mem_wr_addr_size(&mut self) -> MEM_WR_ADDR_SIZE_W<SAR_MEM_WR_CTRL_SPEC> {
        MEM_WR_ADDR_SIZE_W::new(self, 11)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_mem_wr_offst_clr(&mut self) -> RTC_MEM_WR_OFFST_CLR_W<SAR_MEM_WR_CTRL_SPEC> {
        RTC_MEM_WR_OFFST_CLR_W::new(self, 22)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_mem_wr_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_mem_wr_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_MEM_WR_CTRL_SPEC;
impl crate::RegisterSpec for SAR_MEM_WR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_mem_wr_ctrl::R`](R) reader structure"]
impl crate::Readable for SAR_MEM_WR_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_mem_wr_ctrl::W`](W) writer structure"]
impl crate::Writable for SAR_MEM_WR_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_MEM_WR_CTRL to value 0x0010_0200"]
impl crate::Resettable for SAR_MEM_WR_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0010_0200;
}
