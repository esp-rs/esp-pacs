#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `IN_DONE` reader - The interrupt enable bit for the IN_DONE interrupt."]
pub type IN_DONE_R = crate::BitReader;
#[doc = "Field `IN_DONE` writer - The interrupt enable bit for the IN_DONE interrupt."]
pub type IN_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF` reader - The interrupt enable bit for the IN_SUC_EOF interrupt."]
pub type IN_SUC_EOF_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF` writer - The interrupt enable bit for the IN_SUC_EOF interrupt."]
pub type IN_SUC_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF` reader - The interrupt enable bit for the IN_ERR_EOF interrupt."]
pub type IN_ERR_EOF_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF` writer - The interrupt enable bit for the IN_ERR_EOF interrupt."]
pub type IN_ERR_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DONE` reader - The interrupt enable bit for the OUT_DONE interrupt."]
pub type OUT_DONE_R = crate::BitReader;
#[doc = "Field `OUT_DONE` writer - The interrupt enable bit for the OUT_DONE interrupt."]
pub type OUT_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF` reader - The interrupt enable bit for the OUT_EOF interrupt."]
pub type OUT_EOF_R = crate::BitReader;
#[doc = "Field `OUT_EOF` writer - The interrupt enable bit for the OUT_EOF interrupt."]
pub type OUT_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_ERR` reader - The interrupt enable bit for the IN_DSCR_ERR interrupt."]
pub type IN_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR` writer - The interrupt enable bit for the IN_DSCR_ERR interrupt."]
pub type IN_DSCR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR` reader - The interrupt enable bit for the OUT_DSCR_ERR interrupt."]
pub type OUT_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR` writer - The interrupt enable bit for the OUT_DSCR_ERR interrupt."]
pub type OUT_DSCR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY` reader - The interrupt enable bit for the IN_DSCR_EMPTY interrupt."]
pub type IN_DSCR_EMPTY_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY` writer - The interrupt enable bit for the IN_DSCR_EMPTY interrupt."]
pub type IN_DSCR_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF` reader - The interrupt enable bit for the OUT_TOTAL_EOF interrupt."]
pub type OUT_TOTAL_EOF_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF` writer - The interrupt enable bit for the OUT_TOTAL_EOF interrupt."]
pub type OUT_TOTAL_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_FULL_WM` reader - The interrupt enable bit for the INFIFO_FULL_WM interrupt."]
pub type INFIFO_FULL_WM_R = crate::BitReader;
#[doc = "Field `INFIFO_FULL_WM` writer - The interrupt enable bit for the INFIFO_FULL_WM interrupt."]
pub type INFIFO_FULL_WM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the IN_DONE interrupt."]
    #[inline(always)]
    pub fn in_done(&self) -> IN_DONE_R {
        IN_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the IN_SUC_EOF interrupt."]
    #[inline(always)]
    pub fn in_suc_eof(&self) -> IN_SUC_EOF_R {
        IN_SUC_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the IN_ERR_EOF interrupt."]
    #[inline(always)]
    pub fn in_err_eof(&self) -> IN_ERR_EOF_R {
        IN_ERR_EOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the OUT_DONE interrupt."]
    #[inline(always)]
    pub fn out_done(&self) -> OUT_DONE_R {
        OUT_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the OUT_EOF interrupt."]
    #[inline(always)]
    pub fn out_eof(&self) -> OUT_EOF_R {
        OUT_EOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the IN_DSCR_ERR interrupt."]
    #[inline(always)]
    pub fn in_dscr_err(&self) -> IN_DSCR_ERR_R {
        IN_DSCR_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the OUT_DSCR_ERR interrupt."]
    #[inline(always)]
    pub fn out_dscr_err(&self) -> OUT_DSCR_ERR_R {
        OUT_DSCR_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the IN_DSCR_EMPTY interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty(&self) -> IN_DSCR_EMPTY_R {
        IN_DSCR_EMPTY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the OUT_TOTAL_EOF interrupt."]
    #[inline(always)]
    pub fn out_total_eof(&self) -> OUT_TOTAL_EOF_R {
        OUT_TOTAL_EOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the INFIFO_FULL_WM interrupt."]
    #[inline(always)]
    pub fn infifo_full_wm(&self) -> INFIFO_FULL_WM_R {
        INFIFO_FULL_WM_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("in_done", &self.in_done())
            .field("in_suc_eof", &self.in_suc_eof())
            .field("in_err_eof", &self.in_err_eof())
            .field("out_done", &self.out_done())
            .field("out_eof", &self.out_eof())
            .field("in_dscr_err", &self.in_dscr_err())
            .field("out_dscr_err", &self.out_dscr_err())
            .field("in_dscr_empty", &self.in_dscr_empty())
            .field("out_total_eof", &self.out_total_eof())
            .field("infifo_full_wm", &self.infifo_full_wm())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the IN_DONE interrupt."]
    #[inline(always)]
    pub fn in_done(&mut self) -> IN_DONE_W<'_, INT_ENA_SPEC> {
        IN_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the IN_SUC_EOF interrupt."]
    #[inline(always)]
    pub fn in_suc_eof(&mut self) -> IN_SUC_EOF_W<'_, INT_ENA_SPEC> {
        IN_SUC_EOF_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the IN_ERR_EOF interrupt."]
    #[inline(always)]
    pub fn in_err_eof(&mut self) -> IN_ERR_EOF_W<'_, INT_ENA_SPEC> {
        IN_ERR_EOF_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the OUT_DONE interrupt."]
    #[inline(always)]
    pub fn out_done(&mut self) -> OUT_DONE_W<'_, INT_ENA_SPEC> {
        OUT_DONE_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the OUT_EOF interrupt."]
    #[inline(always)]
    pub fn out_eof(&mut self) -> OUT_EOF_W<'_, INT_ENA_SPEC> {
        OUT_EOF_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the IN_DSCR_ERR interrupt."]
    #[inline(always)]
    pub fn in_dscr_err(&mut self) -> IN_DSCR_ERR_W<'_, INT_ENA_SPEC> {
        IN_DSCR_ERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the OUT_DSCR_ERR interrupt."]
    #[inline(always)]
    pub fn out_dscr_err(&mut self) -> OUT_DSCR_ERR_W<'_, INT_ENA_SPEC> {
        OUT_DSCR_ERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the IN_DSCR_EMPTY interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty(&mut self) -> IN_DSCR_EMPTY_W<'_, INT_ENA_SPEC> {
        IN_DSCR_EMPTY_W::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the OUT_TOTAL_EOF interrupt."]
    #[inline(always)]
    pub fn out_total_eof(&mut self) -> OUT_TOTAL_EOF_W<'_, INT_ENA_SPEC> {
        OUT_TOTAL_EOF_W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the INFIFO_FULL_WM interrupt."]
    #[inline(always)]
    pub fn infifo_full_wm(&mut self) -> INFIFO_FULL_WM_W<'_, INT_ENA_SPEC> {
        INFIFO_FULL_WM_W::new(self, 9)
    }
}
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {}
