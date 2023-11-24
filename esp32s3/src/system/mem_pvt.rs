#[doc = "Register `MEM_PVT` reader"]
pub type R = crate::R<MEM_PVT_SPEC>;
#[doc = "Register `MEM_PVT` writer"]
pub type W = crate::W<MEM_PVT_SPEC>;
#[doc = "Field `MEM_PATH_LEN` reader - ******* Description ***********"]
pub type MEM_PATH_LEN_R = crate::FieldReader;
#[doc = "Field `MEM_PATH_LEN` writer - ******* Description ***********"]
pub type MEM_PATH_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MEM_ERR_CNT_CLR` writer - ******* Description ***********"]
pub type MEM_ERR_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONITOR_EN` reader - ******* Description ***********"]
pub type MONITOR_EN_R = crate::BitReader;
#[doc = "Field `MONITOR_EN` writer - ******* Description ***********"]
pub type MONITOR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_TIMING_ERR_CNT` reader - ******* Description ***********"]
pub type MEM_TIMING_ERR_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `MEM_VT_SEL` reader - ******* Description ***********"]
pub type MEM_VT_SEL_R = crate::FieldReader;
#[doc = "Field `MEM_VT_SEL` writer - ******* Description ***********"]
pub type MEM_VT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - ******* Description ***********"]
    #[inline(always)]
    pub fn mem_path_len(&self) -> MEM_PATH_LEN_R {
        MEM_PATH_LEN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - ******* Description ***********"]
    #[inline(always)]
    pub fn monitor_en(&self) -> MONITOR_EN_R {
        MONITOR_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:21 - ******* Description ***********"]
    #[inline(always)]
    pub fn mem_timing_err_cnt(&self) -> MEM_TIMING_ERR_CNT_R {
        MEM_TIMING_ERR_CNT_R::new(((self.bits >> 6) & 0xffff) as u16)
    }
    #[doc = "Bits 22:23 - ******* Description ***********"]
    #[inline(always)]
    pub fn mem_vt_sel(&self) -> MEM_VT_SEL_R {
        MEM_VT_SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_PVT")
            .field(
                "mem_path_len",
                &format_args!("{}", self.mem_path_len().bits()),
            )
            .field("monitor_en", &format_args!("{}", self.monitor_en().bit()))
            .field(
                "mem_timing_err_cnt",
                &format_args!("{}", self.mem_timing_err_cnt().bits()),
            )
            .field("mem_vt_sel", &format_args!("{}", self.mem_vt_sel().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_PVT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn mem_path_len(&mut self) -> MEM_PATH_LEN_W<MEM_PVT_SPEC> {
        MEM_PATH_LEN_W::new(self, 0)
    }
    #[doc = "Bit 4 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn mem_err_cnt_clr(&mut self) -> MEM_ERR_CNT_CLR_W<MEM_PVT_SPEC> {
        MEM_ERR_CNT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn monitor_en(&mut self) -> MONITOR_EN_W<MEM_PVT_SPEC> {
        MONITOR_EN_W::new(self, 5)
    }
    #[doc = "Bits 22:23 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn mem_vt_sel(&mut self) -> MEM_VT_SEL_W<MEM_PVT_SPEC> {
        MEM_VT_SEL_W::new(self, 22)
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
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_pvt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_pvt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_PVT_SPEC;
impl crate::RegisterSpec for MEM_PVT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_pvt::R`](R) reader structure"]
impl crate::Readable for MEM_PVT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_pvt::W`](W) writer structure"]
impl crate::Writable for MEM_PVT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEM_PVT to value 0x03"]
impl crate::Resettable for MEM_PVT_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
